use crate::ActionBehavior;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActionKind {
	Button,
	Axis,
}

pub type ActionId = &'static str;

#[derive(Clone)]
pub struct Action {
	pub(crate) kind: ActionKind,
	pub(crate) behavior: ActionBehavior,
}

impl Action {
	pub fn new(kind: ActionKind) -> Self {
		Self {
			kind,
			behavior: ActionBehavior::default(),
		}
	}

	pub fn with_behavior(mut self, behavior: ActionBehavior) -> Self {
		self.behavior = behavior;
		self
	}
}
