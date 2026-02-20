//! Bindings to the OHAudio module of OpenHarmony.
//!
//! Official documentation: https://gitcode.com/openharmony/docs/blob/master/en/application-dev/reference/apis-audio-kit/capi-ohaudio.md
//!
//! ## Feature flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[link(name = "ohaudio")]
unsafe extern "C" {}

pub mod audiocapturer;
pub mod audiorenderer;
pub mod audiostream_base;
pub mod audiostreambuilder;

#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
pub mod audio_common;
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
pub mod audio_device_base;
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
pub mod audio_manager;
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
pub mod audio_routing_manager;
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
pub mod audio_session_manager;

#[cfg(feature = "api-19")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-19")))]
pub mod audio_stream_manager;

#[cfg(feature = "api-20")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-20")))]
pub mod audio_resource_manager;
#[cfg(feature = "api-20")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-20")))]
pub mod audio_volume_manager;
