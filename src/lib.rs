//! input-actions is a Rust crate heavily inspired by Unity library [Rewired](https://assetstore.unity.com/packages/tools/utilities/rewired-21676) ([Website](https://guavaman.com/projects/rewired/)).
//!
//! This crate utilizes "gilrs" (which uses "rusty-xinput") to handle gamepad input,
//! and both of these crates expose potentially spammy levels of logging when devices are connected.
//! It is recommended you ignore or limit the logging levels of "gilrs" and "rusty_xinput" log targets/modules.
//! This is being tracked by https://gitlab.com/gilrs-project/gilrs/-/issues/105.
//!

#[cfg(feature = "log")]
extern crate log;

/// Submodule for handling integration with `winit` when the feature is enabled.
#[cfg(feature = "winit")]
pub mod winit;

pub static LOG: &'static str = "input-actions";
pub static DEPENDENCY_LOG_TARGETS: [&'static str; 2] = ["gilrs", "rusty_xinput"];

/// Configuration and state data for handling an action set by consuemrs of input-actions.
pub mod action;

/// Data for telling input-actions how a device input is mapped to an action.
pub mod binding;

/// Data pertaining to physical devices (like mice, keyboards, and gamepads) which send input to input-actions.
pub mod device;

/// Enumerations for differentiating between device inputs (buttons, axes, keys, mouse buttons).
pub mod source;

/// Data sent to input-actions when device inputs are detected.
pub mod event;

mod system;
pub use system::*;

mod user;
pub(crate) use user::*;
