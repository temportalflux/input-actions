use crate::ActionBindingMap;
use std::collections::HashMap;

pub type Layout = &'static str;
pub type CategoryId = &'static str;

#[derive(Debug, Clone)]
pub struct Category {
	pub(crate) binding_maps: HashMap<Option<Layout>, ActionBindingMap>,
}

impl Default for Category {
	fn default() -> Self {
		Self {
			binding_maps: HashMap::new(),
		}
	}
}

impl Category {
	pub fn with(mut self, layout: Option<Layout>, map: ActionBindingMap) -> Self {
		self.binding_maps.insert(layout, map);
		self
	}
}
