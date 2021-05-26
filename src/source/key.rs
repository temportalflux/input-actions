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