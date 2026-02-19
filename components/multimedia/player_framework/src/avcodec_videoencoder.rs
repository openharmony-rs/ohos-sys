#[link(name = "native_media_venc")]
extern "C" {}

mod avcodec_videoencoder_ffi;
pub use avcodec_videoencoder_ffi::*;
