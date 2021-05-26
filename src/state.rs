use crate::{Action, ActionBehavior, ActionKind, EventButtonState, EventState};
use std::time::SystemTime;

#[derive(Debug, Clone)]
pub(crate) struct ActionState {
	kind: ActionKind,
	behavior: ActionBehavior,
	/// Used to indicate if a button is pressed or released
	active: bool,
	value: f32,
	modified_at: SystemTime,
}

impl ActionState {
	pub fn new(action: Action) -> Self {
		Self {
			kind: action.kind,
			behavior: action.behavior,
			active: false,
			value: 0.0,
			modified_at: SystemTime::UNIX_EPOCH,
		}
	}

	pub fn process_event(&mut self, state: EventState, time: &SystemTime) {
		if match self.behavior.modify_event_for(state, &self.kind) {
			EventState::ButtonState(btn_state) if self.kind == ActionKind::Button => {
				self.active = btn_state == EventButtonState::Pressed;
				true
			}
			EventState::ButtonState(btn_state) => false,
			EventState::MouseMove(x, y) => false,
			EventState::MouseScroll(x, y) => false,
			EventState::ValueChanged(value) => {
				self.value = value;
				true
			}
		} {
			self.modified_at = *time;
		}
	}
}
