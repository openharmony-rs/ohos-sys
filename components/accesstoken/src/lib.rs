//! Bindings to the OpenHarmony AbilityAccessControl (AccessToken) NDK.
//!
//! ## Overview
//!
//! AbilityAccessControl provides a single C API for checking at runtime whether
//! the calling application has been granted a named permission.  The primary
//! entry point is `OH_AT_CheckSelfPermission`, which returns `true` when the
//! application holds the requested permission and `false` otherwise.
//!
//! Available since OpenHarmony API-level 12.
//!
//! See also the [official AbilityAccessControl documentation](https://gitcode.com/openharmony/docs/blob/master/en/application-dev/reference/apis-ability-kit/_ability_access_control.md).
//!
//! ## Feature flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
pub mod ability_access_control;

#[cfg(feature = "api-12")]
#[link(name = "ability_access_control")]
unsafe extern "C" {}
