use crate::{ActionId, Binding, ControllerKind};
use std::collections::HashMap;

pub type BindingList = HashMap<ControllerKind, Vec<Binding>>;

pub struct ActionBindingMap {
	bindings: HashMap<ActionId, BindingList>,
}

impl Default for ActionBindingMap {
	fn default() -> Self {
		Self {
			bindings: HashMap::new(),
		}
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
		self.bindings.insert(action, controllers);
		self
	}
}
