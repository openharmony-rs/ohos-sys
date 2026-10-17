//! Raw bindings to the OpenHarmony USB DDK, available since API level 10.
//!
//! See the [USB DDK C API reference](https://gitcode.com/openharmony/docs/blob/master/zh-cn/application-dev/reference/apis-driverdevelopment-kit/capi-usb-ddk-api-h.md).
//!
//! Calling these APIs requires `ohos.permission.ACCESS_DDK_USB`.
//! [`UsbDdkErrCode`] uses the values for the selected API level; several codes changed in API 16.
//!
//! ## Feature flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[link(name = "usb_ndk.z")]
unsafe extern "C" {}

mod error_code;
// The SDK's Doxygen return-code lists are not indented as Markdown lists.
#[allow(clippy::doc_lazy_continuation)]
mod usb_ffi;
pub use error_code::UsbDdkErrCode;
pub use usb_ffi::*;
