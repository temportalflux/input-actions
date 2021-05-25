use crate::ActionBehavior;

#[derive(Debug, Clone, Copy)]
pub enum ActionKind {
	Button,
	Axis,
}

pub type ActionId = &'static str;

#[derive(Clone)]
pub struct Action {
	pub kind: ActionKind,
	pub behavior: ActionBehavior,
}
