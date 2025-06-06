//! Provides the comm type definitions of windowManager on the native side.
//!
//! SystemCapability: SystemCapability.Window.SessionManager

#[link(name = "native_window_manager")]
extern "C" {}
mod window_comm_ffi;
pub use window_comm_ffi::*;
