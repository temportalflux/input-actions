#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GamepadKind {
	DualAxisGamepad,
	PS4Dualshock,
	Xbox360,
	Joycon,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum ControllerId {
	Mouse,
	Keyboard,
	Gamepad(GamepadKind, gilrs::GamepadId),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ControllerKind {
	Mouse,
	Keyboard,
	Gamepad(GamepadKind),
}

impl From<ControllerId> for ControllerKind {
	fn from(id: ControllerId) -> ControllerKind {
		match id {
			ControllerId::Mouse => ControllerKind::Mouse,
			ControllerId::Keyboard => ControllerKind::Keyboard,
			ControllerId::Gamepad(kind, _) => ControllerKind::Gamepad(kind),
		}
	}
}

impl std::fmt::Display for ControllerId {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match *self {
			ControllerId::Mouse => write!(f, "Mouse"),
			ControllerId::Keyboard => write!(f, "Keyboard"),
			ControllerId::Gamepad(kind, id) => write!(f, "Gamepad({:?}, {})", kind, id),
		}
	}
}
