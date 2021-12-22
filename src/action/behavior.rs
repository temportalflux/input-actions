use crate::binding::Source;
use std::time::Instant;

pub trait Behavior {
	fn cloned(&self) -> Box<dyn Behavior>;
	fn process(
		&self,
		_source: Source,
		value: f64,
		_time: &Instant,
		_screen_size: &(f64, f64),
	) -> f64 {
		value
	}
}

impl Clone for Box<dyn Behavior> {
	fn clone(&self) -> Box<dyn Behavior> {
		self.cloned()
	}
}

mod multiplier;
pub use multiplier::*;
mod screen_position_delta;
pub use screen_position_delta::*;
mod virtual_axis;
pub use virtual_axis::*;
