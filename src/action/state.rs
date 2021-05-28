use crate::{action, event, source};
use std::time::Instant;

/// The state of an active [`action`](action::Action) for a given user.
#[derive(Debug, Clone)]
pub struct State {
	kind: source::Kind,
	behavior: action::Behavior,
	/// Used to indicate if a button is pressed or released
	prev_frame_active: bool,
	active: bool,
	active_state_changed_this_frame: bool,
	value: f32,
	modified_at: Instant,
}

impl State {
	pub(crate) fn new(action: action::Action) -> Self {
		Self {
			kind: action.kind(),
			behavior: action.behavior().clone(),
			prev_frame_active: false,
			active: false,
			active_state_changed_this_frame: false,
			value: 0.0,
			modified_at: Instant::now(),
		}
	}

	pub(crate) fn process_event(&mut self, event: event::Event, time: &Instant) {
		if match event.state {
			event::State::ButtonState(btn_state) => {
				let is_active = btn_state == event::ButtonState::Pressed;
				if self.active != is_active {
					self.active = is_active;
					true
				} else {
					false
				}
			}
			event::State::MouseMove(_x, _y) => false,
			event::State::MouseScroll(_x, _y) => false,
			event::State::ValueChanged(value) => {
				if self.kind == source::Kind::Axis {
					if event.source.kind() == source::Kind::Button {
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

	pub(crate) fn requires_updates(&self) -> bool {
		true //self.behavior.digital_axis().is_some()
	}

	pub(crate) fn update(&mut self, _time: &Instant) {
		self.active_state_changed_this_frame = self.active != self.prev_frame_active;
		if self.active_state_changed_this_frame {
			self.prev_frame_active = self.active;
		}
	}

	/// Returns true when a [`button binding`](crate::source::Kind::Button) is pressed,
	/// and this function is called in the same update frame as the input event which pressed the button.
	pub fn on_button_pressed(&self) -> bool {
		self.active && self.active_state_changed_this_frame
	}

	/// Returns true when a [`button binding`](crate::source::Kind::Button) is not pressed,
	/// and this function is called in the same update frame as the input event which released the button.
	pub fn on_button_released(&self) -> bool {
		!self.active && self.active_state_changed_this_frame
	}
}
