//! Ohos-sys
//!
//! This crate provides Raw FFI bindings to the native API of OpenHarmonyOS (`target_env = "ohos"`).
//! Each module corresponds to one OpenHarmony API feature, and is gated behind a cargo feature.
//! If you are an application developer, you probably do not want to use this crate directly,
//! and instead want to use a higher-level API built on top of this crate.
//!
//! Note: There are currently still quite a few missing bindings, which will slowly be added.
//!
//! ## Feature flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "arkui")]
#[cfg_attr(docsrs, doc(cfg(feature = "arkui")))]
pub use arkui_sys as arkui;

#[cfg(feature = "deviceinfo")]
#[cfg_attr(docsrs, doc(cfg(feature = "deviceinfo")))]
pub use ohos_deviceinfo_sys as deviceinfo;

#[cfg(feature = "drawing")]
#[cfg_attr(docsrs, doc(cfg(feature = "drawing")))]
pub use ohos_drawing_sys as drawing;

#[cfg(feature = "hilog")]
#[cfg_attr(docsrs, doc(cfg(feature = "hilog")))]
pub use hilog_sys as hilog;

#[cfg(feature = "hitrace")]
#[cfg_attr(docsrs, doc(cfg(feature = "hitrace")))]
pub use hitrace_sys as hitrace;

#[cfg(feature = "napi")]
#[cfg_attr(docsrs, doc(cfg(feature = "napi")))]
pub mod napi;

#[cfg(feature = "native_buffer")]
#[cfg_attr(docsrs, doc(cfg(feature = "native_buffer")))]
pub mod native_buffer;

#[cfg(feature = "native_image")]
#[cfg_attr(docsrs, doc(cfg(feature = "native_image")))]
pub mod native_image;

#[cfg(feature = "native_window")]
#[cfg_attr(docsrs, doc(cfg(feature = "native_window")))]
pub mod native_window;

// It's just one function, so we don't feature guard this.
pub mod syscap;

#[cfg(feature = "vsync")]
#[cfg_attr(docsrs, doc(cfg(feature = "vsync")))]
pub mod vsync;

#[cfg(feature = "xcomponent")]
#[cfg_attr(docsrs, doc(cfg(feature = "xcomponent")))]
pub use xcomponent_sys as xcomponent;
