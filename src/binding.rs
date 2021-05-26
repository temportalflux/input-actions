use crate::{Axis, Button, Key};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Binding {
	KeyboardKey(Key),
	MouseButton(u32),
	GamepadButton(Button),
	GamepadAxis(Axis),
}
