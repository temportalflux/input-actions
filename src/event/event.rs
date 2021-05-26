use crate::{binding::Binding, event};

#[derive(Debug, Clone)]
pub struct Event {
	pub binding: Binding,
	pub state: event::State,
}
