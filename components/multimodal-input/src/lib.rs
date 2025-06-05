//! Bindings to the Multimodal Input-kit API of OpenHarmony
//!
//! Available with API-level 12 and newer
//!
//! Official documentation: <https://developer.huawei.com/consumer/cn/doc/harmonyos-references-V14/input-V14>
//!
//! ## Feature flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg(feature = "api-12")]
#![cfg_attr(docsrs, doc(cfg(feature = "api-12")))]

#[link(name = "ohinput")]
extern "C" {}

pub mod axis_type;
pub mod input_manager;
pub mod key_code;
