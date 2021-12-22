use crate::{
	binding, device, source,
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
	Move(MouseAxis),
	Scroll(MouseAxis),
}

/// The axes a mouse movement or scroll could be bound to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MouseAxis {
	MouseX,
	MouseY,
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
}
