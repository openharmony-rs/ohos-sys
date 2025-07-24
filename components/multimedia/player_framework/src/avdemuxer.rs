#[link(name = "native_media_avdemuxer")]
extern "C" {}

mod avdemuxer_ffi;
pub use avdemuxer_ffi::*;
