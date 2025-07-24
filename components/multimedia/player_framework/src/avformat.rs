#[link(name = "native_media_core")]
extern "C" {}

mod avformat_ffi;
pub use avformat_ffi::*;
