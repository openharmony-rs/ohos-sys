//! Declares APIs for window event filter
//!
//! SystemCapability: SystemCapability.Window.SessionManager

#[link(name = "native_window_manager")]
extern "C" {}
mod window_event_filter_ffi;
pub use window_event_filter_ffi::*;
