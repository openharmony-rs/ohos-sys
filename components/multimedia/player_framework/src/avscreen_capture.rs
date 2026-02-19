#[link(name = "native_avscreen_capture")]
extern "C" {}

mod avscreen_capture_ffi;
pub use avscreen_capture_ffi::*;
