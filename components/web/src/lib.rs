//! Bindings to the Web module of OpenHarmony.
//!
//! Official documentation: https://docs.openharmony.cn/pages/v5.0/en/application-dev/reference/apis-arkweb/native__interface__arkweb_8h.md
//!
//! ## Feature flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg(feature = "api-11")]
#![cfg_attr(docsrs, doc(cfg(feature = "api-11")))]

#[link(name = "ohweb")]
unsafe extern "C" {}

pub mod native_interface_arkweb;

#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
pub mod arkweb_error_code;
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
pub mod arkweb_interface;
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
pub mod arkweb_net_error_list;
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
pub mod arkweb_scheme_handler;
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
pub mod arkweb_type;
