//! Common enum and struct declarations used by the image interface
//!
//! <https://docs.openharmony.cn/pages/v5.0/en/application-dev/reference/apis-image-kit/image__common_8h.md>

#[link(name = "image_common")]
extern "C" {}

#[path = "image_common/image_common_ffi.rs"]
mod image_common_ffi;
pub use image_common_ffi::*;
