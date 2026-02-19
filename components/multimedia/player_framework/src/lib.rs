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

#[cfg(feature = "api-10")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-10")))]
pub mod avcapability;

#[cfg(feature = "api-11")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-11")))]
pub mod avplayer;
#[cfg(feature = "api-11")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-11")))]
pub mod avplayer_base;

pub mod averrors;

pub mod avformat;

#[cfg(feature = "api-10")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-10")))]
pub mod avcodec_audio_channel_layout;
#[cfg(feature = "api-11")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-11")))]
pub mod avcodec_audiocodec;
pub mod avcodec_audiodecoder;
pub mod avcodec_audioencoder;
pub mod avcodec_base;
pub mod avcodec_videodecoder;
pub mod avcodec_videoencoder;

#[cfg(feature = "api-10")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-10")))]
pub mod avdemuxer;
#[cfg(feature = "api-18")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-18")))]
pub mod avimage_generator;
#[cfg(feature = "api-18")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-18")))]
pub mod avimage_generator_base;
#[cfg(feature = "api-18")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-18")))]
pub mod avmetadata_extractor;
#[cfg(feature = "api-18")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-18")))]
pub mod avmetadata_extractor_base;
#[cfg(feature = "api-10")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-10")))]
pub mod avmuxer;
#[cfg(feature = "api-18")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-18")))]
pub mod avrecorder;
#[cfg(feature = "api-18")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-18")))]
pub mod avrecorder_base;
#[cfg(feature = "api-10")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-10")))]
pub mod avscreen_capture;
#[cfg(feature = "api-10")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-10")))]
pub mod avscreen_capture_base;
#[cfg(feature = "api-10")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-10")))]
pub mod avscreen_capture_errors;
#[cfg(feature = "api-10")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-10")))]
pub mod avsource;
#[cfg(feature = "api-20")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-20")))]
pub mod avtranscoder;
#[cfg(feature = "api-20")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-20")))]
pub mod avtranscoder_base;
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
pub mod cencinfo;
#[cfg(feature = "api-20")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-20")))]
pub mod lowpower_audio_sink;
#[cfg(feature = "api-20")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-20")))]
pub mod lowpower_audio_sink_base;
#[cfg(feature = "api-20")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-20")))]
pub mod lowpower_avsink_base;
#[cfg(feature = "api-20")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-20")))]
pub mod lowpower_video_sink;
#[cfg(feature = "api-20")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-20")))]
pub mod lowpower_video_sink_base;
#[cfg(feature = "api-18")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-18")))]
pub mod media_types;
