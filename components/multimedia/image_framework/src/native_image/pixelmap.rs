//! Declares APIs for accessing a PixelMap
//!
//! <https://docs.openharmony.cn/pages/v5.0/en/application-dev/reference/apis-image-kit/pixelmap__native_8h.md>

#[link(name = "pixelmap")]
extern "C" {}

mod pixelmap_ffi;
pub use pixelmap_ffi::*;

pub use ohos_sys_opaque_types::OH_PixelmapNative;
