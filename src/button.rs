#[derive(Debug, Clone)]
pub enum Button {
	/// The bottom button in the face-diamond.
	/// - PS4: X
	/// - Xbox: A
	/// - Switch: B
	FaceBottom,
	/// The left button in the face-diamond.
	/// - PS4: Square
	/// - Xbox: X
	/// - Switch: Y
	FaceLeft,
	/// The right button in the face-diamond.
	/// - PS4: Circle
	/// - Xbox: B
	/// - Switch: A
	FaceRight,
	/// The top button in the face-diamond.
	/// - PS4: Triangle
	/// - Xbox: Y
	/// - Switch: X
	FaceTop,

	/// The button used for confirmation/approval.
	/// This is a virtual wrapper based on the console.
	/// - PS4 case 1: [`X`](Button::FaceBottom)
	/// - PS4 case 2: [`Circle`](Button::FaceRight)
	/// - Xbox: [`A`](Button::FaceBottom)
	/// - Switch: [`A`](Button::FaceRight)
	VirtualConfirmPrimary,
	/// The button used for confirmation/approval.
	/// This is a virtual wrapper based on the console.
	/// - PS4 case 1: [`X`](Button::FaceRight)
	/// - PS4 case 2: [`Circle`](Button::FaceBottom)
	/// - Xbox: [`B`](Button::FaceRight)
	/// - Switch: [`A`](Button::FaceBottom)
	VirtualDeny,

	/// Pressing in on the left thumbstick
	LThumbstick,
	/// Pressing in on the right thumbstick
	RThumbstick,

	DPadUp,
	DPadDown,
	DPadLeft,
	DPadRight,

	LShoulder,
	RShoulder,

	/// - Switch: the trigger is a button not an axis
	/// - PS4 & XBox use [`Axis::LTrigger`](crate::Axis::LTrigger)
	LTrigger,
	/// - Switch: the trigger is a button not an axis
	/// - PS4 & XBox use [`Axis::RTrigger`](crate::Axis::RTrigger)
	RTrigger,

	/// - PS4: Trackpad
	/// - Xbox: Home/Xbox
	/// - Switch: Home
	Special,
	/// - PS4: Share
	/// - Xbox: Share/Windows
	/// - Switch: Minus
	LSpecial,
	/// - PS4: Options
	/// - Xbox: Hamburder
	/// - Switch: Plus
	RSpecial,
}
