use crate::{action, binding::Behavior, device};
use std::collections::HashMap;

type DeviceBindings = HashMap<device::Kind, Vec<Behavior>>;

/// A mapping of the supported device bindings for specific [`actions`](action::Action).
#[derive(Debug, Clone)]
pub struct ActionMap(HashMap<action::Id, DeviceBindings>);

impl Default for ActionMap {
	fn default() -> Self {
		Self(HashMap::new())
	}
}

impl ActionMap {
	/// Bind a list of device inputs to an [`action`](action::Action) by its [`id`](action::Id).
	pub fn bind(mut self, action: action::Id, bindings: &[(device::Kind, Behavior)]) -> Self {
		let mut devices = DeviceBindings::new();
		for (kind, binding) in bindings {
			if !devices.contains_key(kind) {
				devices.insert(*kind, Vec::new());
			}
			devices.get_mut(kind).unwrap().push(binding.clone());
		}
		self.0.insert(action, devices);
		self
	}

	pub(crate) fn iter(&self) -> std::collections::hash_map::Iter<'_, action::Id, DeviceBindings> {
		self.0.iter()
	}
}
