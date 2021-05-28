use crate::{action, binding, device, event};
use std::{
	collections::{HashMap, HashSet},
	time::Instant,
};

#[derive(Debug, Clone)]
pub struct User {
	devices: HashSet<device::Id>,
	active_layout: binding::LayoutId,
	enabled_action_sets: HashMap<binding::ActionSetId, binding::ActionSet>,
	bound_actions: HashMap<BindingStateKey, (action::Id, binding::Binding)>,
	action_states: HashMap<action::Id, action::State>,
	ticking_states: HashSet<action::Id>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct BindingStateKey {
	set_id: binding::ActionSetId,
	layout: binding::LayoutId,
	source: binding::Source,
}

impl Default for User {
	fn default() -> Self {
		Self {
			devices: HashSet::new(),
			active_layout: binding::LayoutId::default(),
			enabled_action_sets: HashMap::new(),
			bound_actions: HashMap::new(),
			action_states: HashMap::new(),
			ticking_states: HashSet::new(),
		}
	}
}

impl User {
	pub fn set_layout(
		&mut self,
		layout: binding::LayoutId,
		actions: &HashMap<action::Id, action::Action>,
	) {
		self.active_layout = layout;
		self.bound_actions.clear();
		self.action_states.clear();
		self.ticking_states.clear();
		let set_ids = self.enabled_action_sets.keys().cloned().collect::<Vec<_>>();
		for set_id in set_ids {
			self.add_action_states(set_id, actions);
		}
	}

	pub(crate) fn add_device(&mut self, device: device::Id) {
		self.devices.insert(device);
	}

	pub(crate) fn remove_device(&mut self, device: device::Id) {
		self.devices.remove(&device);
	}

	pub fn has_gamepad_device(&self) -> bool {
		self.devices.iter().any(|id| match id {
			device::Id::Gamepad(_, _) => true,
			_ => false,
		})
	}

	pub fn enable_action_set(
		&mut self,
		id: binding::ActionSetId,
		set_id: &binding::ActionSet,
		actions: &HashMap<action::Id, action::Action>,
	) {
		self.enabled_action_sets.insert(id, set_id.clone());
		self.add_action_states(id, actions);
	}

	pub fn disable_action_set(&mut self, id: binding::ActionSetId) {
		self.enabled_action_sets.remove(&id);
		self.remove_action_states(&id);
	}

	fn add_action_states(
		&mut self,
		set_id: binding::ActionSetId,
		actions: &HashMap<action::Id, action::Action>,
	) {
		if let Some(action_binding_map) = self
			.enabled_action_sets
			.get(&set_id)
			.unwrap()
			.get(&self.active_layout)
		{
			for (action_id, behaviors) in action_binding_map.iter() {
				if let Some(action) = actions.get(action_id) {
					for behavior in behaviors {
						self.bound_actions.insert(
							BindingStateKey {
								set_id: set_id,
								layout: self.active_layout,
								source: behavior.source,
							},
							(action_id, *behavior),
						);
					}
					let action_state = action::State::new(action.clone());
					if action_state.requires_updates() {
						self.ticking_states.insert(action_id);
					}
					self.action_states.insert(action_id, action_state);
				}
			}
		}
	}

	fn remove_action_states(&mut self, set_id: &binding::ActionSetId) {
		// TODO: Can use `drain_filter` (https://github.com/rust-lang/rust/issues/59618) when stablized
		let mut retained_actions = HashMap::new();
		let mut removed_actions = HashMap::new();
		for (bound_state_key, action_id) in self.bound_actions.drain() {
			if bound_state_key.set_id == *set_id {
				removed_actions.insert(bound_state_key, action_id);
			} else {
				retained_actions.insert(bound_state_key, action_id);
			}
		}
		self.bound_actions = retained_actions;
		for (_, (action_id, _)) in removed_actions {
			self.action_states.remove(action_id);
			self.ticking_states.remove(action_id);
		}
	}

	pub(crate) fn process_event(&mut self, event: &event::Event, time: &Instant) {
		let mut matched_action_ids = Vec::new();
		for (key, action_id) in self.bound_actions.iter() {
			if key.source == event.source {
				matched_action_ids.push(action_id);
			}
		}
		for (action_id, behavior) in matched_action_ids {
			if let Some(state) = self.action_states.get_mut(action_id) {
				state.process_event(behavior.apply(event.clone()), &time);
			}
		}
	}

	pub fn update(&mut self, time: &Instant) {
		for action_id in self.ticking_states.iter() {
			self.action_states.get_mut(action_id).unwrap().update(time);
		}
	}

	pub fn get_action(&self, id: action::Id) -> Option<&action::State> {
		self.action_states.get(id)
	}
}
