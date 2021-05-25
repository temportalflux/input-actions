use crate::{CategoryId, Layout};
use std::collections::HashSet;

#[derive(Clone)]
pub struct User {
	active_layout: Option<Layout>,
	enabled_categories: HashSet<CategoryId>,
}

impl Default for User {
	fn default() -> Self {
		Self {
			active_layout: None,
			enabled_categories: HashSet::new(),
		}
	}
}

impl User {
	pub fn set_layout(&mut self, layout: Option<Layout>) {
		self.active_layout = layout;
	}

	pub fn set_category_enabled(&mut self, category: CategoryId, enabled: bool) {
		if enabled {
			self.enabled_categories.insert(category);
		} else {
			self.enabled_categories.remove(category);
		}
	}
}
