use crate::Binding;
use std::{collections::HashMap, time::Instant};

#[derive(Debug, Clone)]
pub(crate) struct ControllerState(HashMap<Binding, BindingState>);

#[derive(Debug, Clone)]
pub(crate) struct BindingState {
	/// Used to indicate if a button is pressed or released
	active: bool,
	value: f32,
	modified_at: Instant,
}

impl Default for ControllerState {
	fn default() -> Self {
		Self(HashMap::new())
	}
}

impl ControllerState {
	pub(crate) fn set_binding_state(&mut self, binding: Binding, state: BindingState) {
		self.0.insert(binding, state);
	}
}
