pub static ACTION_BEHAVIOR_DEFAULT_BUTTON: Behavior = Behavior { digital_axis: None };

#[derive(Debug, Clone)]
pub struct Behavior {
	digital_axis: Option<DigitalAxis>,
}

impl Behavior {
	pub(crate) fn digital_axis(&self) -> &Option<DigitalAxis> {
		&self.digital_axis
	}
}

/// For [`Button`](crate::source::Kind::Button) events bound to [`Axis`](crate::source::Kind::Axis) actions.
#[derive(Debug, Clone)]
pub struct DigitalAxis {
	reverse: Option<DigitalAxisReverse>,
	/// Speed (units/sec) that the axis value falls toward 0.
	gravity: f32,
	/// Speed to move toward an axis value of 1.0 in units/sec.
	sensitivity: f32,
}

/// Modifier applied when input is received in the opposite direction of the current flow.
#[derive(Debug, Clone)]
pub enum DigitalAxisReverse {
	/// Snap axis value to 0 and continue from there.
	Snap,
	/// Reverse the current value to the opposite sign and continue from there.
	InstantReverse,
}

impl Default for Behavior {
	fn default() -> Self {
		Self { digital_axis: None }
	}
}

fn _interp_to(current: f32, target: f32, delta_time: f32, speed: f32) -> f32 {
	if speed <= 0.0 {
		return target;
	}
	let dist = target - current;
	if dist.powi(2) < f32::EPSILON {
		return target;
	}
	return current + (dist * (delta_time * speed).max(0.0).min(1.0));
}
