use crate::{action, source};

pub type Id = &'static str;

/// As in Rewired, an Action is a application/consumer facing event which a
/// [`User`](crate::UserId) can trigger via a [`Device`](crate::device::Kind).
///
/// To configure an action:
/// - Call [`System::add_action`](crate::System::add_action)
/// - Add the [`Action Id`](Id) used in `add_action` to add the action to an [`action set`](crate::binding::ActionSet).
/// - Add the action set via [`System::add_action_set`](crate::System::add_action_set).
/// - Enable the action set for a given user via [`System::mark_action_set_enabled`](crate::System::mark_action_set_enabled).
///
/// Once configured, [`System::get_user_action`](crate::System::get_user_action) can be called to get the [`action state`](action::State).
#[derive(Clone)]
pub struct Action {
	kind: source::Kind,
	behavior: action::Behavior,
}

impl Action {
	pub fn new(kind: source::Kind) -> Self {
		Self {
			kind,
			behavior: action::Behavior::default(),
		}
	}

	pub fn with_behavior(mut self, behavior: action::Behavior) -> Self {
		self.behavior = behavior;
		self
	}

	pub(crate) fn kind(&self) -> source::Kind {
		self.kind
	}

	pub(crate) fn behavior(&self) -> &action::Behavior {
		&self.behavior
	}
}
