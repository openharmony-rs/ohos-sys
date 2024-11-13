//! Bindings to the native vsync APIs

#[link(name = "native_vsync")]
extern "C" {}

mod vsync_ffi;
pub use vsync_ffi::*;
