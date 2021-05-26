#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Axis {
	/// The x-axis of the left thumbstick.
	LThumbstickX,
	/// The y-axis of the left thumbstick.
	LThumbstickY,

	/// The x-axis of the right thumbstick.
	RThumbstickX,
	/// The y-axis of the right thumbstick.
	RThumbstickY,

	LTrigger,
	RTrigger,
}

impl std::convert::TryFrom<gilrs::Axis> for Axis {
	type Error = ();
	fn try_from(other: gilrs::Axis) -> Result<Self, Self::Error> {
		match other {
			gilrs::Axis::LeftStickX => Ok(Axis::LThumbstickX),
			gilrs::Axis::LeftStickY => Ok(Axis::LThumbstickY),
			gilrs::Axis::RightStickX => Ok(Axis::RThumbstickX),
			gilrs::Axis::RightStickY => Ok(Axis::RThumbstickY),

			gilrs::Axis::LeftZ => Ok(Axis::LTrigger),
			gilrs::Axis::RightZ => Ok(Axis::RTrigger),

			gilrs::Axis::DPadX => Err(()),
			gilrs::Axis::DPadY => Err(()),
			gilrs::Axis::Unknown => Err(()),
		}
	}
}
