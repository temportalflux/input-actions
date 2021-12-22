use crate::{action, binding, device, event, source};
use std::{
	collections::{HashMap, HashSet},
	time::Instant,
};

#[derive(Debug)]
pub struct User {
	devices: HashSet<device::Id>,
	active_layout: binding::LayoutId,
	enabled_action_sets: HashMap<binding::ActionSetId, binding::ActionSet>,
	bound_actions: HashMap<BindingStateKey, action::Id>,
	action_states: HashMap<action::Id, action::State>,
	ticking_states: HashSet<action::Id>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct BindingStateKey {
	set_id: binding::ActionSetId,
	layout: binding::LayoutId,
	sources: Vec<binding::Source>,
}

impl BindingStateKey {
	pub fn contains(&self, source: binding::Source) -> bool {
		self.sources.contains(&source)
	}
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
		actions: &HashMap<action::Id, source::Kind>,
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
		actions: &HashMap<action::Id, source::Kind>,
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
		actions: &HashMap<action::Id, source::Kind>,
	) {
		if let Some(action_binding_map) = self
			.enabled_action_sets
			.get(&set_id)
			.unwrap()
			.get(&self.active_layout)
		{
			for (action_id, behavior_binding) in action_binding_map.iter() {
				if let Some(_action) = actions.get(action_id) {
					self.bound_actions.insert(
						BindingStateKey {
							set_id: set_id,
							layout: self.active_layout,
							sources: behavior_binding.sources(),
						},
						action_id,
					);
					let action_state = action::State::new(behavior_binding.clone());
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
		for (_, action_id) in removed_actions {
			self.action_states.remove(action_id);
			self.ticking_states.remove(action_id);
		}
	}

	pub(crate) fn process_event(
		&mut self,
		source: binding::Source,
		state: &event::State,
		time: &Instant,
		screen_size: (f64, f64),
	) {
		let action_ids_bound_to_source =
			self.bound_actions
				.iter()
				.filter_map(|(key, action_id)| match key.contains(source) {
					true => Some(action_id),
					false => None,
				});
		for action_id in action_ids_bound_to_source {
			if let Some(user_state) = self.action_states.get_mut(action_id) {
				user_state.process_event(source, state.clone(), &time, screen_size);
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
