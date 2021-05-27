use crate::{
	binding, device, event, source,
	source::{Axis, Button, Key, MouseButton},
};

/// Enumeration containing all the possible input sources across all kinds of devices.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Source {
	Mouse(binding::Mouse),
	Keyboard(Key),
	Gamepad(device::GamepadKind, binding::Gamepad),
}

/// All possible inputs from a mouse device.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Mouse {
	Button(MouseButton),
	Move,
	Scroll,
}

/// All possible inputs from a gamepad device.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Gamepad {
	Button(Button),
	Axis(Axis),
}

impl Source {
	/// Returns the kind of source this the source is (button or axis).
	pub fn kind(&self) -> source::Kind {
		match *self {
			Self::Mouse(_) => source::Kind::Button,
			Self::Keyboard(_) => source::Kind::Button,
			Self::Gamepad(_, binding::Gamepad::Button(_)) => source::Kind::Button,
			Self::Gamepad(_, binding::Gamepad::Axis(_)) => source::Kind::Axis,
		}
	}

	/// Returns the kind of device that the source is (mouse, keyboard, gamepad).
	pub fn device_kind(&self) -> device::Kind {
		match *self {
			Self::Mouse(_) => device::Kind::Mouse,
			Self::Keyboard(_) => device::Kind::Keyboard,
			Self::Gamepad(gamepad, _) => device::Kind::Gamepad(gamepad),
		}
	}

	/// Converts the Source into a Binding with default properties/modifiers.
	pub fn bound(self) -> Binding {
		self.with_modifier(1.0)
	}

	/// Converts the Source into a Binding with a provided value modifier.
	pub fn with_modifier(self, modifier: f32) -> Binding {
		Binding {
			source: self,
			modifier,
		}
	}
}

/// A binding to some source with additional properties like a value modifier.
/// Useful for binding buttons to axis actions for example.
#[derive(Debug, Clone, Copy)]
pub struct Binding {
	pub source: Source,
	/// Multipier to the value of the source when its event is received.
	/// Can be used to invert button values from (0..1) range to (0..-1)
	/// before they are applied as the input to an axis action.
	pub modifier: f32,
}

impl Binding {
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
