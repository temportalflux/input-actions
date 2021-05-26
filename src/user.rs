use crate::{
	Binding, BindingState, CategoryId, ControllerId, ControllerKind, ControllerState, Layout,
};
use std::collections::{HashMap, HashSet};

#[derive(Clone)]
pub struct User {
	active_layout: Option<Layout>,
	enabled_categories: HashSet<CategoryId>,
	controller_states: HashMap<ControllerId, ControllerState>,
}

impl Default for User {
	fn default() -> Self {
		Self {
			active_layout: None,
			enabled_categories: HashSet::new(),
			controller_states: HashMap::new(),
		}
	}
}

impl User {
	pub fn set_layout(&mut self, layout: Option<Layout>) {
		self.active_layout = layout;
	}

	pub fn set_category_enabled(&mut self, category: CategoryId, enabled: bool) {
		if enabled {
			self.enabled_categories.insert(category);
		} else {
			self.enabled_categories.remove(category);
		}
	}

	pub(crate) fn add_controller(&mut self, controller: ControllerId) {
		self.controller_states
			.insert(controller, ControllerState::default());
	}

	pub(crate) fn remove_controller(&mut self, controller: ControllerId) {
		self.controller_states.remove(&controller);
	}

	pub(crate) fn has_gamepad_controller(&self) -> bool {
		self.controller_states.keys().any(|id| match id {
			ControllerId::Gamepad(_, _) => true,
			_ => false,
		})
	}

	pub(crate) fn set_binding_state(
		&mut self,
		controller: ControllerId,
		binding: Binding,
		state: BindingState,
	) {
		if let Some(controller_state) = self.controller_states.get_mut(&controller) {
			controller_state.set_binding_state(binding, state);
		}
	}
}
