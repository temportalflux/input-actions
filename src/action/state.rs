use crate::{action::BehaviorBinding, binding, event};
use std::{
	sync::{Arc, RwLock, Weak},
	time::Instant,
};

pub type ArcLockState = Arc<RwLock<State>>;
pub type WeakLockState = Weak<RwLock<State>>;

/// The state of an active action for a given user.
#[derive(Debug, Clone)]
pub struct State {
	behaviors: BehaviorBinding,
	/// Used to indicate if a button is pressed or released
	prev_frame_active: bool,
	active: bool,
	active_state_changed_this_frame: bool,
	value: f64,
	modified_at: Instant,
	last_update_time: Instant,
}

impl State {
	pub(crate) fn new(behaviors: BehaviorBinding) -> Self {
		Self {
			behaviors,
			prev_frame_active: false,
			active: false,
			active_state_changed_this_frame: false,
			value: 0.0,
			modified_at: Instant::now(),
			last_update_time: Instant::now(),
		}
	}

	pub(crate) fn arclocked(self) -> ArcLockState {
		Arc::new(RwLock::new(self))
	}

	pub(crate) fn process_event(
		&mut self,
		source: binding::Source,
		event: event::State,
		time: &Instant,
		screen_size: (f64, f64),
	) {
		if match event {
			event::State::ButtonState(btn_state) => {
				let is_active = btn_state == event::ButtonState::Pressed;

				let value = if is_active { 1.0 } else { 0.0 };
				self.value = self.behaviors.process(source, value, &time, &screen_size);

				if self.active != is_active {
					self.active = is_active;
					true
				} else {
					false
				}
			}
			event::State::MouseMove(delta_pixels) => {
				self.value = self
					.behaviors
					.process(source, delta_pixels, &time, &screen_size);
				true
			}
			event::State::MouseScroll(delta) => {
				self.value = delta as f64;
				true
			}
			event::State::ValueChanged(value) => {
				self.value = value as f64;
				true
			}
		} {
			self.modified_at = *time;
		}
	}

	pub(crate) fn requires_updates(&self) -> bool {
		true //self.behavior.digital_axis().is_some()
	}

	pub(crate) fn update(&mut self, time: &Instant) {
		self.last_update_time = *time;

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

	pub fn is_button_down(&self) -> bool {
		self.active
	}

	/// Returns true when a [`button binding`](crate::source::Kind::Button) is not pressed,
	/// and this function is called in the same update frame as the input event which released the button.
	pub fn on_button_released(&self) -> bool {
		!self.active && self.active_state_changed_this_frame
	}

	// TODO: Mouse inputs should have behavioral options similar to those described in:
	// https://guavaman.com/projects/rewired/docs/RewiredEditor.html#InputBehaviors
	pub fn axis_value(&self) -> f64 {
		let time_since_modified = self.last_update_time.duration_since(self.modified_at);
		let secs_since_modified = time_since_modified.as_secs_f64();
		let is_relevant = secs_since_modified < 0.1;
		if is_relevant {
			self.value
		} else {
			0.0
		}
	}

	pub fn value(&self) -> f64 {
		self.value
	}

	pub fn take_value(&mut self) -> f64 {
		let v = self.value;
		self.value = 0.0;
		v
	}
}
