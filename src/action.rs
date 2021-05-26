use crate::ActionBehavior;

#[derive(Debug, Clone, Copy)]
pub enum ActionKind {
	Button,
	Axis,
}

pub type ActionId = &'static str;

#[derive(Clone)]
pub struct Action {
	kind: ActionKind,
	behavior: ActionBehavior,
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
