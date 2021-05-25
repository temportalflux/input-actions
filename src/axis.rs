#[derive(Debug, Clone)]
pub enum Axis {
	/// The x-axis of the left thumbstick.
	LThumbstickX,
	/// The y-axis of the left thumbstick.
	LThumbstickY,

	/// The x-axis of the right thumbstick.
	RThumbstickX,
	/// The y-axis of the right thumbstick.
	RThumbstickY,

	/// - PS4 & Xbox
	/// - Switch uses [`Button::LTrigger`](crate::Button::LTrigger)
	LTrigger,
	/// - PS4 & Xbox
	/// - Switch uses [`Button::RTrigger`](crate::Button::RTrigger)
	RTrigger,
}
