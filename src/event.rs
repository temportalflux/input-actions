use crate::Binding;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventButtonState {
	Pressed,
	Released,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MouseButton {
	Left,
	Center,
	Right,
}

pub enum EventSource {
	Mouse,
	Keyboard,
}

#[derive(Debug, Clone)]
pub struct Event {
	pub binding: Binding,
	pub state: EventState,
}

#[derive(Debug, Clone, Copy)]
pub enum EventState {
	ButtonState(EventButtonState),
	MouseMove(/*delta pixels x*/ f64, /*delta pixels y*/ f64),
	MouseScroll(f32, f32),
	ValueChanged(f32),
}