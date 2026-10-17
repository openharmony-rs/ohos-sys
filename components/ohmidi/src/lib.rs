//! Raw bindings to the OpenHarmony MIDI NDK, available since API level 24.
//!
//! See the [OH_MIDI development guide](https://gitcode.com/openharmony/docs/blob/master/zh-cn/application-dev/media/audio/using-ohmidi.md).
//!
//! ## Feature flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "api-24")]
#[link(name = "ohmidi")]
unsafe extern "C" {}

#[cfg(feature = "api-24")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-24")))]
mod ohmidi_ffi;
#[cfg(feature = "api-24")]
pub use ohmidi_ffi::*;
