use crate::{action, event, source};
use std::time::{Duration, Instant};

/// The state of an active [`action`](action::Action) for a given user.
#[derive(Debug, Clone)]
pub struct State {
	kind: source::Kind,
	behavior: action::Behavior,
	/// Used to indicate if a button is pressed or released
	active: bool,
	value: f32,
	modified_at: Instant,
}

impl State {
	pub(crate) fn new(action: action::Action) -> Self {
		Self {
			kind: action.kind,
			behavior: action.behavior,
			active: false,
			value: 0.0,
			modified_at: Instant::now(),
		}
	}

	pub(crate) fn process_event(&mut self, event: event::Event, time: &Instant) {
		if match event.state {
			event::State::ButtonState(btn_state) => {
				self.active = btn_state == event::ButtonState::Pressed;
				true
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
		self.behavior.digital_axis.is_some()
	}

	pub(crate) fn update(&mut self, _time: &Instant) {}

	/// Returns true if the amount of time elapsed since the action was last modified
	/// is less than or equal to the provided duration.
	fn modified_within(&self, duration: Duration) -> bool {
		self.modified_at.elapsed() <= duration
	}

	/// Returns true when a [`button binding`](crate::source::Kind::Button) is pressed,
	/// and this function is called <= 1ms after it being pressed.
	pub fn on_button_pressed(&self) -> bool {
		self.active && self.modified_within(Duration::from_millis(1))
	}

	/// Returns true when a [`button binding`](crate::source::Kind::Button) is not pressed,
	/// and this function is called <= 1ms after it being released.
	pub fn on_button_released(&self) -> bool {
		!self.active && self.modified_within(Duration::from_millis(1))
	}
}
