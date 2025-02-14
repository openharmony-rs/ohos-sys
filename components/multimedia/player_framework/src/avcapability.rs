//! The AVCapability module provides functions for querying encoding and decoding capabilities.

#[link(name = "native_media_codecbase")]
extern "C" {}

mod avcapability_ffi;
pub use avcapability_ffi::*;
