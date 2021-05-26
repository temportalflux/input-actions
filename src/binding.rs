use crate::{Axis, Button, Key, MouseButton};

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
