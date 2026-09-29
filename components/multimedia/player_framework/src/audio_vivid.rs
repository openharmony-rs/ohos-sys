#[link(name = "native_media_core")]
extern "C" {}

mod audio_vivid_ffi;
pub use audio_vivid_ffi::*;
