use crate::{Axis, Button, Key};

#[derive(Debug, PartialEq, Eq)]
pub enum EventButtonState {
	Pressed,
	Released,
}

#[derive(Debug)]
pub enum Event {
	MouseMove(/*delta pixels x*/ f64, /*delta pixels y*/ f64),
	MouseScroll(f32, f32),
	Axis(Axis, /*value*/ f64),
	Button(Button, EventButtonState),
	Key(Key, EventButtonState),
}
