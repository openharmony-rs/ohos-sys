//! Bindings for image_native.h
//!
//! <https://docs.openharmony.cn/pages/v5.0/en/application-dev/reference/apis-image-kit/image__native_8h.md>

#[link(name = "ohimage")]
extern "C" {}

mod image_ffi;
pub use image_ffi::*;
