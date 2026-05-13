//! Bindings to the OpenHarmony Universal Keystore Kit (HUKS).
//!
//! ## Overview
//!
//! HUKS is the OpenHarmony system service for hardware-backed key management
//! and cryptographic operations. Keys can be generated inside HUKS or imported
//! from the application; private key material never leaves the keystore.
//! Typical workflows use a parameter set (`OH_Huks_ParamSet`) to describe the
//! key purpose, algorithm, and access constraints, then a session
//! (`InitSession` / `UpdateSession` / `FinishSession`) to perform a single
//! cryptographic operation referencing the key by alias.
//!
//! Available since OpenHarmony API-level 9. Crate minimum is API-level 10 to
//! match the rest of `ohos-sys`. Later API levels add error codes, additional
//! algorithms (SM2/SM3/SM4 at API-18), and `WrapKey`/`UnwrapKey` (API-20).
//!
//! See also the [official HUKS documentation](https://gitcode.com/openharmony/docs/blob/master/en/application-dev/reference/apis-universal-keystore-kit/_huks_key_api.md).
//!
//! ## Feature flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod native_huks_api;
pub mod native_huks_param;
pub mod native_huks_type;

#[link(name = "huks_ndk.z")]
unsafe extern "C" {}
