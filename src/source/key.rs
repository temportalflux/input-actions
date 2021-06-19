#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Key {
	// Alphabet
	A,
	B,
	C,
	D,
	E,
	F,
	G,
	H,
	I,
	J,
	K,
	L,
	M,
	N,
	O,
	P,
	Q,
	R,
	S,
	T,
	U,
	V,
	W,
	X,
	Y,
	Z,

	// Function Keys
	Escape,
	F1,
	F2,
	F3,
	F4,
	F5,
	F6,
	F7,
	F8,
	F9,
	F10,
	F11,
	F12,
	F13,
	F14,
	F15,
	F16,
	F17,
	F18,
	F19,
	F20,
	F21,
	F22,
	F23,
	F24,

	// Number Keys (Not Numpad)
	Key1,
	Key2,
	Key3,
	Key4,
	Key5,
	Key6,
	Key7,
	Key8,
	Key9,
	Key0,

	// Numpad Keys
	Numlock,
	Numpad0,
	Numpad1,
	Numpad2,
	Numpad3,
	Numpad4,
	Numpad5,
	Numpad6,
	Numpad7,
	Numpad8,
	Numpad9,
	NumpadPlus,
	NumpadMinus,
	NumpadAsterisk,
	NumpadSlash,
	NumpadDecimal,
	NumpadEnter,

	// Control Keys
	Snapshot,
	ScrollLock,
	Pause,

	// Home Keys
	Insert,
	Home,
	Delete,
	End,
	PageUp,
	PageDown,

	// Arrow Keys
	Left,
	Right,
	Up,
	Down,

	// Keyboard Controls
	Grave,
	Back,
	Tab,
	CapitalLock,
	Return,
	Space,
	// Modifiers
	LAlt,
	RAlt,
	LShift,
	RShift,
	LControl,
	RControl,
	LWin,
	RWin,

	// Alpha-adjacent
	Minus,
	Equals,
	LBracket,
	RBracket,
	Backslash,
	Semicolon,
	Apostrophe,
	Comma,
	Period,
	Slash,
}

#[derive(Debug, Hash, enumset::EnumSetType)]
pub enum KeyModifier {
	Shift,
	Control,
	Alt,
	/// This is the “windows” key on PC and “command” key on Mac.
	Platform,
}

impl Key {
	pub fn to_string(&self, modifiers: &enumset::EnumSet<KeyModifier>) -> Option<String> {
		use Key::*;
		let uppercase = modifiers.contains(KeyModifier::Shift);
		match *self {
			// Alphabet
			A => Some(if uppercase { "A" } else { "a" }),
			B => Some(if uppercase { "B" } else { "b" }),
			C => Some(if uppercase { "C" } else { "c" }),
			D => Some(if uppercase { "D" } else { "d" }),
			E => Some(if uppercase { "E" } else { "e" }),
			F => Some(if uppercase { "F" } else { "f" }),
			G => Some(if uppercase { "G" } else { "g" }),
			H => Some(if uppercase { "H" } else { "h" }),
			I => Some(if uppercase { "I" } else { "i" }),
			J => Some(if uppercase { "J" } else { "j" }),
			K => Some(if uppercase { "K" } else { "k" }),
			L => Some(if uppercase { "L" } else { "l" }),
			M => Some(if uppercase { "M" } else { "m" }),
			N => Some(if uppercase { "N" } else { "n" }),
			O => Some(if uppercase { "O" } else { "o" }),
			P => Some(if uppercase { "P" } else { "p" }),
			Q => Some(if uppercase { "Q" } else { "q" }),
			R => Some(if uppercase { "R" } else { "r" }),
			S => Some(if uppercase { "S" } else { "s" }),
			T => Some(if uppercase { "T" } else { "t" }),
			U => Some(if uppercase { "U" } else { "u" }),
			V => Some(if uppercase { "V" } else { "v" }),
			W => Some(if uppercase { "W" } else { "w" }),
			X => Some(if uppercase { "X" } else { "x" }),
			Y => Some(if uppercase { "Y" } else { "y" }),
			Z => Some(if uppercase { "Z" } else { "z" }),

			// Number Keys (Not Numpad)
			Key1 => Some(if uppercase { "!" } else { "1" }),
			Key2 => Some(if uppercase { "@" } else { "2" }),
			Key3 => Some(if uppercase { "#" } else { "3" }),
			Key4 => Some(if uppercase { "$" } else { "4" }),
			Key5 => Some(if uppercase { "%" } else { "5" }),
			Key6 => Some(if uppercase { "^" } else { "6" }),
			Key7 => Some(if uppercase { "&" } else { "7" }),
			Key8 => Some(if uppercase { "*" } else { "8" }),
			Key9 => Some(if uppercase { "(" } else { "9" }),
			Key0 => Some(if uppercase { ")" } else { "0" }),

			// Numpad Keys
			Numpad0 => Some("0"),
			Numpad1 => Some("1"),
			Numpad2 => Some("2"),
			Numpad3 => Some("3"),
			Numpad4 => Some("4"),
			Numpad5 => Some("5"),
			Numpad6 => Some("6"),
			Numpad7 => Some("7"),
			Numpad8 => Some("8"),
			Numpad9 => Some("9"),
			NumpadPlus => Some("+"),
			NumpadMinus => Some("-"),
			NumpadAsterisk => Some("*"),
			NumpadSlash => Some("/"),
			NumpadDecimal => Some("."),

			// Keyboard Controls
			Grave => Some(if uppercase { "~" } else { "`" }),
			Space => Some(" "),

			// Alpha-adjacent
			Minus => Some(if uppercase { "_" } else { "-" }),
			Equals => Some(if uppercase { "+" } else { "=" }),
			LBracket => Some(if uppercase { "{" } else { "[" }),
			RBracket => Some(if uppercase { "}" } else { "]" }),
			Backslash => Some(if uppercase { "|" } else { "\\" }),
			Semicolon => Some(if uppercase { ":" } else { ";" }),
			Apostrophe => Some(if uppercase { "\"" } else { "'" }),
			Comma => Some(if uppercase { "<" } else { "," }),
			Period => Some(if uppercase { ">" } else { "." }),
			Slash => Some(if uppercase { "?" } else { "/" }),

			_ => None,
		}
		.map(|s| s.to_owned())
	}
}
