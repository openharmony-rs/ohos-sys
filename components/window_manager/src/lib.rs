//! Window Manager bindings for OpenHarmony
//!
//! Documentation: https://docs.openharmony.cn/pages/v5.1/en/application-dev/reference/apis-arkui/_window_manager___native_module.md
//!
//! ## Feature flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg(feature = "api-12")]

#[link(name = "native_display_manager")]
extern "C" {}

#[cfg(feature = "api-14")]
#[path = "display_capture/display_capture_ffi.rs"]
pub mod display_capture;

#[path = "display_manager/display_manager_ffi.rs"]
pub mod display_manager;

#[path = "display_info/display_info_ffi.rs"]
pub mod display_manager_info;

#[path = "window_comm/window_comm_ffi.rs"]
pub mod window_comm;

pub mod window_event_filter;
