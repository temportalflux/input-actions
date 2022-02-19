use crate::{binding, device};
use crossbeam_channel::{Receiver, Sender};

pub type InputSender = Sender<(binding::Source, State)>;
pub type InputReceiver = Receiver<(binding::Source, State)>;

/// An event created by a third-party to send input to [`DeviceCache`](crate::DeviceCache::send_event).
#[derive(Debug, Clone)]
pub enum Event {
	Input(device::Id, binding::Source, State),
	Window(WindowEvent),
}

#[derive(Debug, Clone)]
pub enum WindowEvent {
	ResolutionChanged(/*width*/ u32, /*height*/ u32),
	ScaleFactorChanged(
		/*width*/ u32,
		/*height*/ u32,
		/*scale factor*/ f64,
	),
}

/// The state of a [`gamepad`](crate::source::Button) or [`mouse`](crate::source::MouseButton) button.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonState {
	Pressed,
	Released,
}

/// What non-gamepad device caused the event.
#[derive(Debug, Clone)]
pub enum Source {
	Mouse,
	Keyboard,
}

/// The data for [`Event`].
/// Can provide a mouse button/keyboard key state, mouse move, mouse scroll, or mouse button/keyboard key value.
#[derive(Debug, Clone, Copy)]
pub enum State {
	ButtonState(ButtonState),
	MouseMove(/*delta pixels*/ f64),
	MouseScroll(f32),
	ValueChanged(f32),
}
