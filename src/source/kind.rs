/// Enum to differentiate between the two types of inputs: single state (button) and range (axis).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Kind {
	Axis,
	Button,
}
