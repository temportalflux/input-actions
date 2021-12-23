use crate::device::GamepadKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Id {
	Mouse,
	Keyboard,
	Gamepad(GamepadKind, /*gilrs::GamepadId*/ usize),
}

impl std::fmt::Display for Id {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match *self {
			Id::Mouse => write!(f, "Mouse"),
			Id::Keyboard => write!(f, "Keyboard"),
			Id::Gamepad(kind, id) => write!(f, "Gamepad({:?}, {})", kind, id),
		}
	}
}
