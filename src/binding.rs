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
