//! Media framework bindings for OpenHarmony
//!
//!
//! ## Feature flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "api-11")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-11")))]
pub mod avbuffer;

pub mod avbuffer_info;
pub mod avmemory;

pub mod avcapability;

#[cfg(feature = "api-11")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-11")))]
pub mod avplayer;
#[cfg(feature = "api-11")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-11")))]
pub mod avplayer_base;

pub mod averrors;

pub mod avformat;

pub mod avcodec_base;

pub mod avdemuxer;
pub mod avsource;
