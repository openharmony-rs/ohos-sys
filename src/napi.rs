#[link(name = "ace_napi.z")]
extern "C" {}

mod napi_ffi;
pub use napi_ffi::*;
