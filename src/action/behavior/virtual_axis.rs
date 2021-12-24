use crate::action::behavior::Behavior;

/// For [`Button`](crate::source::Kind::Button) events bound to [`Axis`](crate::source::Kind::Axis) actions.
#[derive(Debug, Clone, Copy)]
pub struct VirtualAxis {
	/// Speed (units/sec) that the axis value falls toward 0.
	pub gravity: f32,
	/// Speed to move toward an axis value of 1.0 in units/sec.
	pub acceleration: f32,
	pub on_reverse: Option<VirtualAxisReverse>,
}

/// Modifier applied when input is received in the opposite direction of the current flow.
#[derive(Debug, Clone, Copy)]
pub enum VirtualAxisReverse {
	/// Snap axis value to 0 and continue from there.
	Snap,
	/// Reverse the current value to the opposite sign and continue from there.
	InstantReverse,
}

impl Behavior for VirtualAxis {
	fn cloned(&self) -> Box<dyn Behavior + Send + Sync> {
		Box::new(self.clone())
	}

	fn debug_string(&self) -> String {
		format!("{:?}", self)
	}
}
