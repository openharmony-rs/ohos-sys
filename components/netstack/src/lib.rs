//! Rust bindings to OpenHarmony NetworkKit (netstack) native APIs.
//!
//! ## Feature flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg(feature = "api-11")]
#![cfg_attr(docsrs, doc(cfg(feature = "api-11")))]

#[cfg(feature = "api-20")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-20")))]
pub mod net_http;

#[cfg(feature = "api-20")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-20")))]
pub mod net_http_type;

#[cfg(feature = "api-11")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-11")))]
pub mod net_websocket;

#[cfg(feature = "api-11")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-11")))]
pub mod net_websocket_type;
