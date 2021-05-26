use crate::{EventState, ActionKind};

pub static ACTION_BEHAVIOR_DEFAULT_BUTTON: ActionBehavior = ActionBehavior {};

#[derive(Debug, Clone)]
pub struct ActionBehavior {}

impl Default for ActionBehavior {
	fn default() -> Self {
		Self {}
	}
}

impl ActionBehavior {
	pub(crate) fn modify_event_for(&self, event: EventState, _kind: &ActionKind) -> EventState {
		event
	}
}
