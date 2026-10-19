//! Raw bindings to the OpenHarmony Purgeable Memory API, available since API level 10.
//!
//! See the [Purgeable Memory development guide](https://gitcode.com/openharmony/docs/blob/master/en/application-dev/kernel-enhance/purgeable-memory-guidelines.md).
//!
//! ## Feature flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[link(name = "purgeable_memory_ndk.z")]
unsafe extern "C" {}

mod purgeable_memory_ffi;
pub use purgeable_memory_ffi::*;
