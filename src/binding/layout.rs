use crate::binding::ActionMap;
use std::collections::HashMap;

/// An identifier representing the way device inputs are mapped
/// to [`actions`](`crate::action::Action`) via [`action maps`](ActionMap).
pub type Layout = Option<&'static str>;

/// An identifier representing [`LayoutBindings`], a set of bindings to a given action for each supported [`Layout`].
/// Can be toggled on/off per user via [`System::set_category_enabled`](crate::System::set_category_enabled).
pub type CategoryId = Option<&'static str>;

/// Represented by [`CategoryId`].
/// This is a collection of [`bindings`](ActionMap) per [`layout`](Layout)
/// which are bound to a specific [`action`](crate::action::Action).
#[derive(Debug, Clone)]
pub struct LayoutBindings(HashMap<Layout, ActionMap>);

impl Default for LayoutBindings {
	fn default() -> Self {
		Self(HashMap::new())
	}
}

impl LayoutBindings {

	/// Associates a layout with a map of action to device bindings,
	/// for this category.
	pub fn with(mut self, layout: Layout, map: ActionMap) -> Self {
		self.0.insert(layout, map);
		self
	}

	pub(crate) fn get(&self, layout: &Layout) -> Option<&ActionMap> {
		self.0.get(layout)
	}
}
