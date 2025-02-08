//! Unified Data Management Framework (UDMF) bindings for OpenHarmony
//!
//! The Unified Data Management Framework(UDMF) aims to define various standards
//! for data across applications, devices, and platforms, providing a unified OpenHarmony
//! data language and standardized data access and reading paths.
//!
//! ## Feature flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg(feature = "api-12")]

#[link(name = "udmf")]
extern "C" {}

pub mod data_management_framework;
mod udmf_err_code;

pub use udmf_err_code::Udmf_ErrCode;

pub mod data_struct;
pub mod meta;
pub mod type_descriptor;
