#[link(name = "native_fence")]
extern "C" {}

#[path = "native_fence/native_fence_ffi.rs"]
mod native_fence_ffi;
pub use native_fence_ffi::*;
