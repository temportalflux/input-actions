use crate::action;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
	Button,
	Axis,
}

pub type Id = &'static str;

#[derive(Clone)]
pub struct Action {
	pub(crate) kind: Kind,
	pub(crate) behavior: action::Behavior,
}

impl Action {
	pub fn new(kind: Kind) -> Self {
		Self {
			kind,
			behavior: action::Behavior::default(),
		}
	}

	pub fn with_behavior(mut self, behavior: action::Behavior) -> Self {
		self.behavior = behavior;
		self
	}
}
