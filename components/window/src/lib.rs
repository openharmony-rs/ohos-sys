//! OpenHarmony native window, buffer and image bindings
//!
//! There are a lot of interdependencies in the APIs for native window, native buffer
//! and native images, so we release these APIs in one crate.
//!
//! See the module level documentation for more information on the individual modules.
//!
//! ## Feature flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod native_buffer;
#[cfg(feature = "api-20")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-20")))]
pub mod native_fence;
pub mod native_image;
pub mod native_window;

pub use native_window::graphic_error_code::*;
