use crate::{action, binding::Binding};
use std::collections::HashMap;

/// A mapping of the supported device bindings for specific [`actions`](action::Action).
#[derive(Debug, Clone)]
pub struct ActionMap(HashMap<action::Id, Vec<Binding>>);

impl Default for ActionMap {
	fn default() -> Self {
		Self(HashMap::new())
	}
}

impl ActionMap {
	/// Bind a list of device inputs to an [`action`](action::Action) by its [`id`](action::Id).
	pub fn bind(mut self, action: action::Id, bindings: Vec<Binding>) -> Self {
		self.0.insert(action, bindings);
		self
	}

	pub(crate) fn iter(&self) -> std::collections::hash_map::Iter<'_, action::Id, Vec<Binding>> {
		self.0.iter()
	}
}
