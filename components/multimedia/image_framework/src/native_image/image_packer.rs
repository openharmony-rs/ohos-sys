//! Declares APIs for image encoding
//!
//! <https://docs.openharmony.cn/pages/v5.0/en/application-dev/reference/apis-image-kit/image__packer__native_8h.md>

#[link(name = "image_packer")]
extern "C" {}

mod image_packer_ffi;
pub use image_packer_ffi::*;
