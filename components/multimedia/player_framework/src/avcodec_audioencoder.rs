#[link(name = "native_media_aenc")]
extern "C" {}

mod avcodec_audioencoder_ffi;
pub use avcodec_audioencoder_ffi::*;
