use crate::binding::Source;
use std::time::Instant;

pub enum Kind {
	Map,
	Fold,
}

pub trait Behavior {
	fn cloned(&self) -> Box<dyn Behavior + Send + Sync>;
	fn debug_string(&self) -> String {
		std::any::type_name::<Self>().to_owned()
	}
	fn kind(&self) -> Kind {
		Kind::Map
	}
	fn map(&self, _source: Source, _value: f64, _time: &Instant, _screen_size: &(f64, f64)) -> f64 {
		unimplemented!()
	}
	fn fold(&self, _values: &[f64]) -> f64 {
		unimplemented!()
	}
}

impl Clone for Box<dyn Behavior + Send + Sync> {
	fn clone(&self) -> Box<dyn Behavior + Send + Sync> {
		self.cloned()
	}
}

mod average;
pub use average::*;
mod multiplier;
pub use multiplier::*;
mod screen_position_delta;
pub use screen_position_delta::*;
mod virtual_axis;
pub use virtual_axis::*;
