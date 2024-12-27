//! Bindings to the native inputmethod API
//!
//! This can be used to interact with the inputmethod on OpenHarmony from native code.
//! Available with API-level 12 and newer
//!
//! Official documentation: https://developer.huawei.com/consumer/cn/doc/harmonyos-references-V5/_input_method-V5
//!
//! ## Feature flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg(feature = "api-12")]
#![cfg_attr(docsrs, doc(cfg(feature = "api-12")))]


#[link(name = "ohinputmethod")]
extern "C" {}

pub mod attach_options;
pub mod controller;
pub mod cursor_info;
pub mod inputmethod_proxy;
pub mod private_command;
pub mod text_avoid_info;
pub mod text_config;
pub mod text_editor_proxy;
pub mod types;
