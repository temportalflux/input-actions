#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
	/// - PS4 case 1: [`X (Bottom)`](Button::FaceBottom)
	/// - Xbox: [`A (Bottom)`](Button::FaceBottom)
	/// - PS4 case 2: [`Circle (Right)`](Button::FaceRight)
	/// - Switch: [`A (Right)`](Button::FaceRight)
	VirtualConfirm,
	/// The button used for confirmation/approval.
	/// This is a virtual wrapper based on the console.
	/// - PS4 case 1: [`Circle (Right)`](Button::FaceRight)
	/// - Xbox: [`B (Right)`](Button::FaceRight)
	/// - PS4 case 2: [`X (Bottom)`](Button::FaceBottom)
	/// - Switch: [`B (Bottom)`](Button::FaceBottom)
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

	LTrigger,
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

impl std::convert::TryFrom<gilrs::Button> for Button {
	type Error = ();
	fn try_from(other: gilrs::Button) -> Result<Self, Self::Error> {
		match other {
			// Action Pad
			gilrs::Button::South => Ok(Button::FaceBottom),
			gilrs::Button::East => Ok(Button::FaceLeft),
			gilrs::Button::North => Ok(Button::FaceTop),
			gilrs::Button::West => Ok(Button::FaceRight),
			gilrs::Button::C => Err(()),
			gilrs::Button::Z => Err(()),
			// Triggers
			gilrs::Button::LeftTrigger => Ok(Button::LTrigger),
			gilrs::Button::LeftTrigger2 => Ok(Button::LTrigger),
			gilrs::Button::RightTrigger => Ok(Button::RTrigger),
			gilrs::Button::RightTrigger2 => Ok(Button::RTrigger),
			// Menu Pad
			gilrs::Button::Select => Ok(Button::LSpecial),
			gilrs::Button::Start => Ok(Button::RSpecial),
			gilrs::Button::Mode => Ok(Button::Special),
			// Sticks
			gilrs::Button::LeftThumb => Ok(Button::LThumbstick),
			gilrs::Button::RightThumb => Ok(Button::RThumbstick),
			// D-Pad
			gilrs::Button::DPadUp => Ok(Button::DPadUp),
			gilrs::Button::DPadDown => Ok(Button::DPadDown),
			gilrs::Button::DPadLeft => Ok(Button::DPadLeft),
			gilrs::Button::DPadRight => Ok(Button::DPadRight),

			gilrs::Button::Unknown => Err(()),
		}
	}
}
