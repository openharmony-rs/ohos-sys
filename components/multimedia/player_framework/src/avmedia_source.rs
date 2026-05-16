mod avmedia_source_ffi;
pub use avmedia_source_ffi::*;

#[link(name = "avmedia_source")]
extern "C" {}
