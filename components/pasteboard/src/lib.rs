//! Clipboard / Pasteboard bindings for OpenHarmony
//!
//!
//! ## Feature flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[link(name = "pasteboard")]
extern "C" {}

mod pasteboard;
pub use pasteboard::*;

mod pasteboard_err_code;
pub use pasteboard_err_code::*;
