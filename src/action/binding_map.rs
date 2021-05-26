use crate::{action, binding, device};
use std::collections::HashMap;

pub type BindingList = HashMap<device::Kind, Vec<binding::Behavior>>;

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
		bindings: &[(device::Kind, binding::Behavior)],
	) -> Self {
		let mut devices = HashMap::new();
		for (kind, binding) in bindings {
			if !devices.contains_key(kind) {
				devices.insert(*kind, Vec::new());
			}
			devices.get_mut(kind).unwrap().push(binding.clone());
		}
		self.0.insert(action, devices);
		self
	}
}
