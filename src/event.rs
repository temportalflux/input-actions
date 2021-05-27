use crate::binding;

/// An event created by a third-party to send input to [`System`](crate::System::send_event).
#[derive(Debug, Clone)]
pub struct Event {
	pub(crate) source: binding::Source,
	pub(crate) state: State,
}

impl Event {
	pub fn new(source: binding::Source, state: State) -> Self {
		Self { source, state }
	}
}

/// The state of a [`gamepad`](crate::source::Button) or [`mouse`](crate::source::MouseButton) button.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonState {
	Pressed,
	Released,
}

/// What non-gamepad device caused the event.
pub enum Source {
	Mouse,
	Keyboard,
}

/// The data for [`Event`].
/// Can provide a mouse button/keyboard key state, mouse move, mouse scroll, or mouse button/keyboard key value.
#[derive(Debug, Clone, Copy)]
pub enum State {
	ButtonState(ButtonState),
	MouseMove(/*delta pixels x*/ f64, /*delta pixels y*/ f64),
	MouseScroll(f32, f32),
	ValueChanged(f32),
}
