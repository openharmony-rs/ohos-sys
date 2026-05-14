//! Bindings to the OpenHarmony CryptoArchitectureKit (`libohcrypto.so`).
//!
//! ## Overview
//!
//! CryptoArchitectureKit is the OpenHarmony cryptographic framework. It
//! provides APIs for message digests, MAC, symmetric and asymmetric key
//! generation, symmetric and asymmetric ciphers, signatures, key agreement,
//! KDFs and random number generation.
//!
//! Available since OpenHarmony API-level 12. The asymmetric cipher, KDF, key
//! agreement, MAC and random number generator modules were added in API-level
//! 20.
//!
//! See also the [official CryptoArchitectureKit documentation](https://gitcode.com/openharmony/docs/blob/master/en/application-dev/reference/apis-crypto-architecture-kit/_crypto_common_api.md).
//!
//! ## Feature flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
pub mod common;

#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
pub mod asym_key;

#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
pub mod digest;

#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
pub mod signature;

#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
pub mod sym_cipher;

#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
pub mod sym_key;

#[cfg(feature = "api-20")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-20")))]
pub mod asym_cipher;

#[cfg(feature = "api-20")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-20")))]
pub mod kdf;

#[cfg(feature = "api-20")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-20")))]
pub mod key_agreement;

#[cfg(feature = "api-20")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-20")))]
pub mod mac;

#[cfg(feature = "api-20")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-20")))]
pub mod rand;

#[cfg(feature = "api-12")]
#[link(name = "ohcrypto")]
unsafe extern "C" {}
