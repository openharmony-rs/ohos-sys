//! Rust bindings to OpenHarmony NetworkKit net_ssl native APIs.
//!
//! ## Feature flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg(feature = "api-11")]
#![cfg_attr(docsrs, doc(cfg(feature = "api-11")))]

#[cfg(feature = "api-11")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-11")))]
pub mod net_ssl_c;

#[cfg(feature = "api-11")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-11")))]
pub mod net_ssl_c_type;
