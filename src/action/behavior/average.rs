use crate::action::behavior::{Behavior, Kind};

#[derive(Debug, Clone, Copy)]
pub struct Average;
impl Behavior for Average {
	fn cloned(&self) -> Box<dyn Behavior + Send + Sync> {
		Box::new(self.clone())
	}

	fn debug_string(&self) -> String {
		format!("{:?}", self)
	}

	fn kind(&self) -> Kind {
		Kind::Fold
	}

	fn fold(&self, values: &[f64]) -> f64 {
		values.iter().fold(0.0f64, |out, v| out + v) / (values.len() as f64)
	}
}
