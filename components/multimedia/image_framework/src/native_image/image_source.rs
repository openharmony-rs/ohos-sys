//! Declares APIs for image decoding
//!
//! <https://docs.openharmony.cn/pages/v5.0/en/application-dev/reference/apis-image-kit/image__source__native_8h.md>

#[link(name = "image_source")]
extern "C" {}

mod image_source_ffi;
pub use image_source_ffi::*;
