#[link(name = "native_media_vdec")]
extern "C" {}

mod avcodec_videodecoder_ffi;
pub use avcodec_videodecoder_ffi::*;
