use crate::{Axis, Button, Key};

#[derive(Debug, PartialEq, Eq)]
pub enum EventButtonState {
	Pressed,
	Released,
}

#[derive(Debug)]
pub enum MouseButton {
	Left,
	Center,
	Right,
}

pub enum EventSource {
	Mouse,
	Keyboard,
}

#[derive(Debug)]
pub enum Event {
	MouseButton(MouseButton, EventButtonState),
	MouseMove(/*delta pixels x*/ f64, /*delta pixels y*/ f64),
	MouseScroll(f32, f32),
	KeyboardKey(Key, EventButtonState),
	GamepadButtonState(Button, EventButtonState),
	GamepadButtonChanged(Button, f32),
	GamepadAxisChanged(Axis, f32),
}
