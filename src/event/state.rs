use crate::event::ButtonState;

#[derive(Debug, Clone, Copy)]
pub enum State {
	ButtonState(ButtonState),
	MouseMove(/*delta pixels x*/ f64, /*delta pixels y*/ f64),
	MouseScroll(f32, f32),
	ValueChanged(f32),
}
