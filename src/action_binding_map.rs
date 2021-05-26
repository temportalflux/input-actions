use crate::{ActionId, Binding, ControllerKind};
use std::collections::HashMap;

pub type BindingList = HashMap<ControllerKind, Vec<Binding>>;

#[derive(Debug, Clone)]
pub struct ActionBindingMap(pub(crate) HashMap<ActionId, BindingList>);

impl Default for ActionBindingMap {
	fn default() -> Self {
		Self(HashMap::new())
	}
}

impl ActionBindingMap {
	pub fn bind(mut self, action: ActionId, bindings: &[(ControllerKind, Binding)]) -> Self {
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
