//! The OH_DisplayManager module provides the display management capability.
//!
//! SystemCapability: SystemCapability.WindowManager.WindowManager.Core
//!
//! Documentation: <https://docs.openharmony.cn/pages/v5.1/en/application-dev/reference/apis-arkui/_o_h___display_manager.md>

#[link(name = "native_display_manager")]
extern "C" {}

mod display_manager_ffi;
pub use display_manager_ffi::*;
