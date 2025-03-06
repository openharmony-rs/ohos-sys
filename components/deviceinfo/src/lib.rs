//! OpenHarmony functions providing device information.
//!
//! See also the official [DeviceInfo documentation].
//!
//! [DeviceInfo documentation]: https://docs.openharmony.cn/pages/v5.0/en/application-dev/reference/apis-basic-services-kit/_device_info.md

#[link(name = "deviceinfo_ndk.z")]
extern "C" {}

mod deviceinfo_ffi;
pub use deviceinfo_ffi::*;
