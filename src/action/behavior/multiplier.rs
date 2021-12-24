use crate::{action::behavior::Behavior, binding::Source};
use std::time::Instant;

#[derive(Debug, Clone, Copy)]
pub struct Multiplier(pub f32);
impl Behavior for Multiplier {
	fn cloned(&self) -> Box<dyn Behavior + Send + Sync> {
		Box::new(self.clone())
	}

	fn debug_string(&self) -> String {
		format!("{:?}", self)
	}

	fn map(&self, _source: Source, value: f64, _time: &Instant, _screen_size: &(f64, f64)) -> f64 {
		value * (self.0 as f64)
	}
}
