#[link(name = "lowpower_avsink")]
extern "C" {}

mod lowpower_audio_sink_ffi;
pub use lowpower_audio_sink_ffi::*;
