//! Raw bindings to the OpenHarmony NativeDisplaySoloist API, available since API level 12.
//!
//! See the [NativeDisplaySoloist C API reference](https://gitcode.com/openharmony/docs/blob/master/en/application-dev/reference/apis-arkgraphics2d/capi-nativedisplaysoloist.md).
//!
//! ## Feature flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "api-12")]
#[link(name = "native_display_soloist")]
unsafe extern "C" {}

#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
mod display_soloist_ffi;
#[cfg(feature = "api-12")]
pub use display_soloist_ffi::*;
