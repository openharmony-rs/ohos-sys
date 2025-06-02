//! Declares APIs for window
//!
//! SystemCapability: SystemCapability.Window.SessionManager

#[link(name = "native_window_manager")]
extern "C" {}

mod window_ffi;
pub use window_ffi::*;
