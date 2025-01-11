//! Bindings to the rawfile APIs
//!
//! ## Overview
//!
//! The OpenHarmony rawfile API allows the user to access so-called `rawfile`s that are bundled
//! into the `hap` bundle.
//!
//! You can use the APIs to traverse, open, search for, read, and close raw files.
//! The rawfile APIs are non-thread-safe. Only the close and open APIs are thread-safe.
//!
//! See also the [official rawfile documentation](https://docs.openharmony.cn/pages/v5.0/en/application-dev/reference/apis-localization-kit/rawfile.md).
//!
//!
//! ## Feature flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![cfg_attr(docsrs, feature(doc_cfg))]

extern "C" {}

mod raw_file_types_ffi;
pub use raw_file_types_ffi::*;

#[cfg(feature = "_functions")]
pub mod raw_dir;
#[cfg(feature = "_functions")]
pub mod raw_file;
#[cfg(feature = "_functions")]
pub mod raw_file_manager;
