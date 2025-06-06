//! Defines the data structures for the C APIs of the display module.
//!
//! SystemCapability: SystemCapability.WindowManager.WindowManager.Core

#[link(name = "native_display_manager")]
extern "C" {}

mod display_capture_ffi;
pub use display_capture_ffi::*;
