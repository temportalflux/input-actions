use crate::{binding, event};

#[derive(Debug, Clone)]
pub struct Event {
	pub source: binding::Source,
	pub state: event::State,
}
