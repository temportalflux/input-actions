use crate::binding::ActionMap;
use std::collections::HashMap;

/// An identifier representing the way device inputs are mapped
/// to actions via [`action maps`](ActionMap).
pub type LayoutId = Option<&'static str>;

/// An identifier representing [`ActionSet`], a set of bindings to a given action for each supported [`LayoutId`].
/// Can be toggled on/off per user via
/// [`User::enable_action_set`](crate::User::enable_action_set)/[`User::disable_action_set`](crate::User::disable_action_set).
pub type ActionSetId = Option<&'static str>;

/// Represented by [`ActionSetId`].
/// This is a collection of [`bindings`](ActionMap) per [`layout`](LayoutId)
/// which are bound to a specific action.
#[derive(Debug, Clone)]
pub struct ActionSet(HashMap<LayoutId, ActionMap>);

impl Default for ActionSet {
	fn default() -> Self {
		Self(HashMap::new())
	}
}

impl ActionSet {
	/// Associates a layout with a map of action to device bindings.
	pub fn with(mut self, layout: LayoutId, map: ActionMap) -> Self {
		self.0.insert(layout, map);
		self
	}

	pub(crate) fn get(&self, layout: &LayoutId) -> Option<&ActionMap> {
		self.0.get(layout)
	}
}
