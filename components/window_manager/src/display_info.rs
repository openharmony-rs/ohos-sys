//! Defines data structures for the C APIs of the display module.
//!
//! SystemCapability: SystemCapability.WindowManager.WindowManager.Core

#[link(name = "native_display_manager")]
extern "C" {}

mod display_info_ffi;
pub use display_info_ffi::*;
