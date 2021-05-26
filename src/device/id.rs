use crate::device::GamepadKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum Id {
	Mouse,
	Keyboard,
	Gamepad(GamepadKind, gilrs::GamepadId),
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
