use crate::{action, binding, device, Category, CategoryId, Event, Layout};
use std::{
	collections::{HashMap, HashSet},
	time::SystemTime,
};

#[derive(Debug, Clone)]
pub struct User {
	devices: HashSet<device::Id>,
	active_layout: Option<Layout>,
	enabled_categories: HashMap<Option<CategoryId>, Category>,
	bound_actions: HashMap<BindingStateKey, (action::Id, binding::Behavior)>,
	action_states: HashMap<action::Id, action::State>,
	ticking_states: HashSet<action::Id>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct BindingStateKey {
	category: Option<CategoryId>,
	layout: Option<Layout>,
	device_kind: device::Kind,
	binding: binding::Binding,
}

impl Default for User {
	fn default() -> Self {
		Self {
			devices: HashSet::new(),
			active_layout: None,
			enabled_categories: HashMap::new(),
			bound_actions: HashMap::new(),
			action_states: HashMap::new(),
			ticking_states: HashSet::new(),
		}
	}
}

impl User {
	pub fn set_layout(
		&mut self,
		layout: Option<Layout>,
		actions: &HashMap<action::Id, action::Action>,
	) {
		self.active_layout = layout;
		self.bound_actions.clear();
		self.action_states.clear();
		self.ticking_states.clear();
		let category_ids = self.enabled_categories.keys().cloned().collect::<Vec<_>>();
		for category_id in category_ids {
			self.add_action_states(category_id, actions);
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

	pub fn enable_category(
		&mut self,
		id: Option<CategoryId>,
		category: &Category,
		actions: &HashMap<action::Id, action::Action>,
	) {
		self.enabled_categories.insert(id, category.clone());
		self.add_action_states(id, actions);
	}

	pub fn disable_category(&mut self, id: Option<CategoryId>) {
		self.enabled_categories.remove(&id);
		self.remove_action_states(&id);
	}

	fn add_action_states(
		&mut self,
		category_id: Option<CategoryId>,
		actions: &HashMap<action::Id, action::Action>,
	) {
		if let Some(action_binding_map) = self
			.enabled_categories
			.get(&category_id)
			.unwrap()
			.get(&self.active_layout)
		{
			for (action_id, behavior_list) in action_binding_map.0.iter() {
				if let Some(action) = actions.get(action_id) {
					for (device_kind, behaviors) in behavior_list {
						for behavior in behaviors {
							self.bound_actions.insert(
								BindingStateKey {
									category: category_id,
									layout: self.active_layout,
									device_kind: *device_kind,
									binding: behavior.binding,
								},
								(action_id, *behavior),
							);
						}
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

	fn remove_action_states(&mut self, category_id: &Option<CategoryId>) {
		// TODO: Can use `drain_filter` (https://github.com/rust-lang/rust/issues/59618) when stablized
		let mut retained_actions = HashMap::new();
		let mut removed_actions = HashMap::new();
		for (bound_state_key, action_id) in self.bound_actions.drain() {
			if bound_state_key.category == *category_id {
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

	pub(crate) fn process_event(&mut self, device: device::Id, event: &Event, time: &SystemTime) {
		let mut matched_action_ids = Vec::new();
		for (key, action_id) in self.bound_actions.iter() {
			if key.device_kind == device.into() && key.binding == event.binding {
				matched_action_ids.push(action_id);
			}
		}
		for (action_id, behavior) in matched_action_ids {
			if let Some(state) = self.action_states.get_mut(action_id) {
				state.process_event(behavior.apply(event.clone()), &time);
			}
		}
	}

	pub fn update(&mut self, time: &SystemTime) {
		for action_id in self.ticking_states.iter() {
			self.action_states.get_mut(action_id).unwrap().update(time);
		}
	}
}
