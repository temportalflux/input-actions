pub type Id = &'static str;

pub mod behavior;
mod behavior_binding;
pub use behavior_binding::*;
mod state;
pub use state::*;
