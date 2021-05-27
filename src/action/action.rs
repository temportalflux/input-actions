use crate::{action, source};

pub type Id = &'static str;

/// As in Rewired, an Action is a application/consumer facing event which a
/// [`User`](crate::UserId) can trigger via a [`Device`](crate::device::Kind).
///
/// To configure an action:
/// - Call [`System::add_action`](crate::System::add_action)
/// - Add the [`Action Id`](Id) used in `add_action` to add the action to a [`category`](crate::binding::LayoutBindings).
/// - Add the category via [`System::add_map_category`](crate::System::add_map_category).
/// - Enable the category for a given user via [`System::set_category_enabled`](crate::System::set_category_enabled).
///
/// Once configured, [`System::get_user_action`](crate::System::get_user_action) can be called to get the [`action state`](action::State).
#[derive(Clone)]
pub struct Action {
	pub(crate) kind: source::Kind,
	pub(crate) behavior: action::Behavior,
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
}
