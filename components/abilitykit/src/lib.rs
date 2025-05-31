//! Bindings to the OpenHarmony AbilityKit.
//!
//! Requires at least API-12.
//! ## Feature flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg(feature = "api-12")]

#[cfg(feature = "api-15")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-15")))]
pub mod base;

#[cfg(feature = "api-13")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-13")))]
pub mod runtime;

#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
pub mod childprocess;
