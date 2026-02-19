//! Defines data types for [crate::avplayer]
#[link(name = "avplayer")]
extern "C" {}

mod avplayer_base_ffi;
pub use avplayer_base_ffi::*;
