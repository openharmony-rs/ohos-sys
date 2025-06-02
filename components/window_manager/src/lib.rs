//! Window Manager bindings for OpenHarmony
//!
//! Documentation: <https://docs.openharmony.cn/pages/v5.1/en/application-dev/reference/apis-arkui/_window_manager___native_module.md>
//!
//! ## Feature flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg(feature = "api-12")]

#[cfg(feature = "api-14")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-14")))]
pub mod display_capture;

pub mod display_info;

pub mod display_manager;

#[cfg(feature = "api-15")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-15")))]
pub mod window;

pub mod window_comm;

pub mod window_event_filter;
