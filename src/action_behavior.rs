pub static ACTION_BEHAVIOR_DEFAULT_BUTTON: ActionBehavior = ActionBehavior {};

#[derive(Clone)]
pub struct ActionBehavior {}

impl Default for ActionBehavior {
	fn default() -> Self {
		Self {}
	}
}
