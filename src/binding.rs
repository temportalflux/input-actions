use crate::{
	event,
	source::{Axis, Button, Key, MouseButton},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Binding {
	MouseButton(MouseButton),
	MouseMove,
	MouseScroll,
	KeyboardKey(Key),
	GamepadButton(Button),
	GamepadAxis(Axis),
}

impl Binding {
	pub fn is_axis(&self) -> bool {
		match *self {
			Self::MouseButton(_) | Self::KeyboardKey(_) | Self::GamepadButton(_) => false,
			Self::GamepadAxis(_) => true,
			Self::MouseMove | Self::MouseScroll => false,
		}
	}

	pub fn is_button(&self) -> bool {
		!self.is_axis()
	}
}

#[derive(Debug, Clone, Copy)]
pub struct Behavior {
	pub binding: Binding,
	pub modifier: f32,
}

impl Behavior {
	pub fn new(binding: Binding) -> Self {
		Self {
			binding,
			modifier: 1.0,
		}
	}

	pub(crate) fn apply(&self, event: event::Event) -> event::Event {
		let mut event = event;
		match &mut event.state {
			event::State::ButtonState(_btn_state) => {}
			event::State::MouseMove(_x, _y) => {}
			event::State::MouseScroll(_x, _y) => {}
			event::State::ValueChanged(value) => *value *= self.modifier,
		}
		event
	}
}
