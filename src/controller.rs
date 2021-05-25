#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub enum ControllerKind {
	Keyboard,
	Mouse,
	DualAxisGamepad,
	PS4Dualshock,
	Xbox360,
	Joycon,
}
