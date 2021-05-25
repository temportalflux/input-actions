use crate::{Axis, Button, Key};

#[derive(Debug, Clone)]
pub enum Binding {
	Button(Button),
	Key(Key),
	Axis(Axis),
}
