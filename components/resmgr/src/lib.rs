//! Bindings to the OpenHarmony resourcemanager API (`resourcemanager`).
//!
//! Resourcemanager is a module that provides access to application
//! resources or system resources by resource ID or resource name at
//! the Native layer.
//!
//! See also the [official resourcemanager API reference](https://gitcode.com/openharmony/docs/blob/master/en/application-dev/reference/apis-localization-kit/capi-resourcemanager.md).
//!
//! ## Feature flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "api-12")]
#[link(name = "ohresmgr")]
unsafe extern "C" {}

#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
pub mod ohresmgr;

#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
pub mod resmgr_common;
