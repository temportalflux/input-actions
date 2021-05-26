use crate::action::BindingMap;
use std::collections::HashMap;

pub type Layout = &'static str;
pub type CategoryId = &'static str;

#[derive(Debug, Clone)]
pub struct Category(HashMap<Option<Layout>, BindingMap>);

impl Default for Category {
	fn default() -> Self {
		Self(HashMap::new())
	}
}

impl Category {
	pub fn with(mut self, layout: Option<Layout>, map: BindingMap) -> Self {
		self.0.insert(layout, map);
		self
	}

	pub fn get(&self, layout: &Option<Layout>) -> Option<&BindingMap> {
		self.0.get(layout)
	}
}
