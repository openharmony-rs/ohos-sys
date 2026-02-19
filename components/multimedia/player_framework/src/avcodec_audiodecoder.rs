#[link(name = "native_media_adec")]
extern "C" {}

mod avcodec_audiodecoder_ffi;
pub use avcodec_audiodecoder_ffi::*;
