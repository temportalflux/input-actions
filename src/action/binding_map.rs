use crate::{action, binding, ControllerKind};
use std::collections::HashMap;

pub type BindingList = HashMap<ControllerKind, Vec<binding::Behavior>>;

#[derive(Debug, Clone)]
pub struct BindingMap(pub(crate) HashMap<action::Id, BindingList>);

impl Default for BindingMap {
	fn default() -> Self {
		Self(HashMap::new())
	}
}

impl BindingMap {
	pub fn bind(
		mut self,
		action: action::Id,
		bindings: &[(ControllerKind, binding::Behavior)],
	) -> Self {
		let mut controllers = HashMap::new();
		for (kind, binding) in bindings {
			if !controllers.contains_key(kind) {
				controllers.insert(*kind, Vec::new());
			}
			controllers.get_mut(kind).unwrap().push(binding.clone());
		}
		self.0.insert(action, controllers);
		self
	}
}
