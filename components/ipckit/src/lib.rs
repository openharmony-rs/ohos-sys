//! Bindings to the IPCKit framework of OpenHarmony.
//!
//! Official documentation: https://docs.openharmony.cn/pages/v5.0/en/application-dev/reference/apis-ipc-kit/ipc__kit_8h.md
//!
//! ## Feature flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg(feature = "api-12")]
#![cfg_attr(docsrs, doc(cfg(feature = "api-12")))]

#[link(name = "ipc_capi")]
unsafe extern "C" {}

pub mod cparcel;
pub mod cremote_object;
pub mod cskeleton;
pub mod error_code;
