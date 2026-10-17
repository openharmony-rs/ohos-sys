//! Raw bindings to the OpenHarmony USB Serial DDK, available since API level 18.
//!
//! See the [USB Serial DDK development guide](https://gitcode.com/openharmony/docs/blob/master/zh-cn/application-dev/device/driver/usb-serial-ddk-guidelines.md).
//!
//! Calling these APIs requires `ohos.permission.ACCESS_DDK_USB_SERIAL`.
//! `USB_SERIAL_DDK_SUCCESS` is nonzero (`31600000`).
//!
//! ## Feature flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "api-18")]
#[link(name = "usb_serial_ndk.z")]
unsafe extern "C" {}

#[cfg(feature = "api-18")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-18")))]
// SDK Doxygen return-code lists are not indented as Markdown lists.
#[allow(clippy::doc_lazy_continuation)]
mod usb_serial_ffi;
#[cfg(feature = "api-18")]
pub use usb_serial_ffi::*;
