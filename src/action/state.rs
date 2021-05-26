use crate::{action, event};
use std::time::SystemTime;

#[derive(Debug, Clone)]
pub struct State {
	kind: action::Kind,
	behavior: action::Behavior,
	/// Used to indicate if a button is pressed or released
	active: bool,
	value: f32,
	modified_at: SystemTime,
}

impl State {
	pub fn new(action: action::Action) -> Self {
		Self {
			kind: action.kind,
			behavior: action.behavior,
			active: false,
			value: 0.0,
			modified_at: SystemTime::UNIX_EPOCH,
		}
	}

	pub fn process_event(&mut self, event: event::Event, time: &SystemTime) {
		if match event.state {
			event::State::ButtonState(btn_state) => {
				self.active = btn_state == event::ButtonState::Pressed;
				true
			}
			event::State::MouseMove(_x, _y) => false,
			event::State::MouseScroll(_x, _y) => false,
			event::State::ValueChanged(value) => {
				if self.kind == action::Kind::Axis {
					if !event.binding.is_axis() {
						// TODO: Handle digitial axis
					}
				}
				self.value = value;
				true
			}
		} {
			self.modified_at = *time;
		}
	}

	pub fn requires_updates(&self) -> bool {
		self.behavior.digital_axis.is_some()
	}

	pub fn update(&mut self, _time: &SystemTime) {}
}
