use crate::{action::behavior::Behavior, binding::Source};
use std::time::Instant;

/// A value from -1.0 to 1.0 that represents the distance moved across
/// the screen over the update cycle. The value is normalized based on
/// the width and height of the screen. A movement across the entire screen
/// distance in one update cycle will return +/- 1.0.
#[derive(Debug, Clone, Copy)]
pub struct ScreenPositionDelta;
impl Behavior for ScreenPositionDelta {
	fn cloned(&self) -> Box<dyn Behavior + Send + Sync> {
		Box::new(self.clone())
	}

	fn debug_string(&self) -> String {
		format!("{:?}", self)
	}

	fn map(&self, source: Source, value: f64, _time: &Instant, screen_size: &(f64, f64)) -> f64 {
		use crate::binding::{Mouse, MouseAxis::*};
		let axis = match source {
			Source::Mouse(Mouse::Move(axis)) => axis,
			_ => unimplemented!(),
		};
		let size = match axis {
			MouseX => screen_size.0,
			MouseY => screen_size.1,
		};
		value / size
	}
}
