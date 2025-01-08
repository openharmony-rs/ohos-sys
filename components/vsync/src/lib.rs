//! Bindings to the native vsync APIs
//!
//! See also [Native Vsync Development](https://docs.openharmony.cn/pages/v5.0/en/application-dev/graphics/native-vsync-guidelines.md).
//!
//!
//! ## Feature flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[link(name = "native_vsync")]
extern "C" {}

mod vsync_ffi;
pub use vsync_ffi::*;
