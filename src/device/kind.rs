use crate::device::{GamepadKind, Id};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Kind {
	Window,
	Mouse,
	Keyboard,
	Gamepad(GamepadKind),
}

impl From<Id> for Kind {
	fn from(id: Id) -> Kind {
		match id {
			Id::Window => Kind::Window,
			Id::Mouse => Kind::Mouse,
			Id::Keyboard => Kind::Keyboard,
			Id::Gamepad(kind, _) => Kind::Gamepad(kind),
		}
	}
}
