use crate::action::{self, BehaviorBinding};
use std::collections::HashMap;

/// A mapping of the supported device bindings for specific [`actions`](action::Action).
#[derive(Default, Debug, Clone)]
pub struct ActionMap(HashMap<action::Id, BehaviorBinding>);

impl ActionMap {
	/// Bind a list of device inputs to an [`action`](action::Action) by its [`id`](action::Id).
	pub fn bind<T>(mut self, action: action::Id, bindings: T) -> Self
	where
		T: Into<BehaviorBinding>,
	{
		self.0.insert(action, bindings.into());
		self
	}

	pub(crate) fn iter(&self) -> std::collections::hash_map::Iter<'_, action::Id, BehaviorBinding> {
		self.0.iter()
	}
}
