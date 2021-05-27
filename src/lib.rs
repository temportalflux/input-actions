//! ReBound is a Rust crate heavily inspired by Unity library [Rewired](https://assetstore.unity.com/packages/tools/utilities/rewired-21676) ([Website](https://guavaman.com/projects/rewired/)).
//! 

#[cfg(feature = "log")]
extern crate log;

/// Submodule for handling integration with `winit` when the feature is enabled.
#[cfg(feature = "winit")]
pub mod winit;

/// Configuration and state data for handling an action set by consuemrs of ReBound.
pub mod action;

/// Data for telling ReBound how a device input is mapped to an action.
pub mod binding;

mod category;
pub use category::*;

/// Data pertaining to physical devices (like mice, keyboards, and gamepads) which send input to ReBound.
pub mod device;

/// Enumerations for differentiating between device inputs (buttons, axes, keys, mouse buttons).
pub mod source;

/// Data sent to ReBound when device inputs are detected.
pub mod event;

mod system;
pub use system::*;

mod user;
pub(crate) use user::*;
