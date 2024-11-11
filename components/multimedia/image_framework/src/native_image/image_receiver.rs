//! Declares APIs used to obtain image data from the native layer
//!
//! <https://docs.openharmony.cn/pages/v5.0/en/application-dev/reference/apis-image-kit/image__receiver__native_8h.md>

#[link(name = "image_receiver")]
extern "C" {}

mod image_receiver_api12;
pub use image_receiver_api12::*;
