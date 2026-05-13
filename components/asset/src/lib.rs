//! Bindings to the OpenHarmony AssetStoreKit (Asset NDK).
//!
//! ## Overview
//!
//! AssetStoreKit provides a system-managed store for **short sensitive data**
//! (≤ 1024 bytes) — typical contents include account credentials, OAuth /
//! refresh tokens and other secrets that should not be persisted in plaintext
//! by the application. The store is encrypted at rest and access can be gated
//! on screen-lock state or user authentication.
//!
//! Available since OpenHarmony API-level 11. One function
//! (`OH_Asset_QuerySyncResult`) and the `Asset_SyncResult` struct were added in
//! API-level 20.
//!
//! See also the [official Asset documentation](https://gitcode.com/openharmony/docs/blob/master/en/application-dev/reference/apis-asset-store-kit/_asset_api.md).
//!
//! ## Feature flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "api-11")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-11")))]
pub mod asset_api;

#[cfg(feature = "api-11")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-11")))]
pub mod asset_type;

#[cfg(feature = "api-11")]
#[link(name = "asset_ndk.z")]
unsafe extern "C" {}
