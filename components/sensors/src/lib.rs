//! Rust bindings for the `sensors` module of OpenHarmony.
//!
//! ## Feature flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg(feature = "api-11")]
#![cfg_attr(docsrs, doc(cfg(feature = "api-11")))]

#[link(name = "ohsensor")]
unsafe extern "C" {}

#[link(name = "ohvibrator.z")]
unsafe extern "C" {}

pub mod sensor;
pub mod sensor_type;
pub mod vibrator;
pub mod vibrator_type;
