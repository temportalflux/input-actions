use crate::{
	Action, ActionId, ActionState, Binding, Category, CategoryId, ControllerId, ControllerKind,
	Event, Layout,
};
use std::{
	collections::{HashMap, HashSet},
	time::SystemTime,
};

#[derive(Debug, Clone)]
pub(crate) struct User {
	controllers: HashSet<ControllerId>,
	active_layout: Option<Layout>,
	enabled_categories: HashMap<Option<CategoryId>, Category>,
	bound_actions: HashMap<BindingStateKey, ActionId>,
	action_states: HashMap<ActionId, ActionState>,
	ticking_states: HashSet<ActionId>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct BindingStateKey {
	category: Option<CategoryId>,
	layout: Option<Layout>,
	controller_kind: ControllerKind,
	binding: Binding,
}

impl Default for User {
	fn default() -> Self {
		Self {
			controllers: HashSet::new(),
			active_layout: None,
			enabled_categories: HashMap::new(),
			bound_actions: HashMap::new(),
			action_states: HashMap::new(),
			ticking_states: HashSet::new(),
		}
	}
}

impl User {
	pub fn set_layout(&mut self, layout: Option<Layout>, actions: &HashMap<ActionId, Action>) {
		self.active_layout = layout;
		self.bound_actions.clear();
		self.action_states.clear();
		self.ticking_states.clear();
		let category_ids = self.enabled_categories.keys().cloned().collect::<Vec<_>>();
		for category_id in category_ids {
			self.add_action_states(category_id, actions);
		}
	}

	pub fn add_controller(&mut self, controller: ControllerId) {
		self.controllers.insert(controller);
	}

	pub fn remove_controller(&mut self, controller: ControllerId) {
		self.controllers.remove(&controller);
	}

	pub fn has_gamepad_controller(&self) -> bool {
		self.controllers.iter().any(|id| match id {
			ControllerId::Gamepad(_, _) => true,
			_ => false,
		})
	}

	pub fn enable_category(
		&mut self,
		id: Option<CategoryId>,
		category: &Category,
		actions: &HashMap<ActionId, Action>,
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
		actions: &HashMap<ActionId, Action>,
	) {
		if let Some(action_binding_map) = self
			.enabled_categories
			.get(&category_id)
			.unwrap()
			.binding_maps
			.get(&self.active_layout)
		{
			for (action_id, binding_list) in action_binding_map.0.iter() {
				if let Some(action) = actions.get(action_id) {
					for (controller_kind, bindings) in binding_list {
						for binding in bindings {
							self.bound_actions.insert(
								BindingStateKey {
									category: category_id,
									layout: self.active_layout,
									controller_kind: *controller_kind,
									binding: *binding,
								},
								action_id,
							);
						}
					}
					let action_state = ActionState::new(action.clone());
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
		for (_, action_id) in removed_actions {
			self.action_states.remove(action_id);
			self.ticking_states.remove(action_id);
		}
	}

	pub fn process_event(&mut self, controller: ControllerId, event: &Event, time: &SystemTime) {
		let mut matched_action_ids = Vec::new();
		for (key, action_id) in self.bound_actions.iter() {
			if key.controller_kind == controller.into() && key.binding == event.binding {
				matched_action_ids.push(action_id);
			}
		}
		for action_id in matched_action_ids {
			if let Some(state) = self.action_states.get_mut(action_id) {
				state.process_event(event.clone(), &time);
			}
		}
	}

	pub fn update(&mut self, time: &SystemTime) {
		for action_id in self.ticking_states.iter() {
			self.action_states.get_mut(action_id).unwrap().update(time);
		}
	}
}
