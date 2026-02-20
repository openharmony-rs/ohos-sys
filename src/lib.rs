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

#[cfg(feature = "inputmethod")]
#[cfg_attr(docsrs, doc(cfg(feature = "inputmethod")))]
pub use ohos_ime_sys as inputmethod;

#[cfg(feature = "ipckit")]
#[cfg_attr(docsrs, doc(cfg(feature = "ipckit")))]
pub use ohos_ipckit_sys as ipckit;

#[cfg(feature = "multimodal-input")]
#[cfg_attr(docsrs, doc(cfg(feature = "multimodal-input")))]
pub use ohos_input_sys as multimodal_input;

pub mod multimedia;

#[cfg(feature = "napi")]
#[cfg_attr(docsrs, doc(cfg(feature = "napi")))]
pub mod napi;

#[cfg(feature = "abilitykit")]
#[cfg_attr(docsrs, doc(cfg(feature = "abilitykit")))]
pub use ohos_abilitykit_sys as abilitykit;

#[cfg(feature = "pasteboard")]
#[cfg_attr(docsrs, doc(cfg(feature = "pasteboard")))]
pub use ohos_pasteboard_sys as pasteboard;

#[cfg(feature = "rawfile")]
#[cfg_attr(docsrs, doc(cfg(feature = "rawfile")))]
pub use ohos_rawfile_sys as rawfile;

#[cfg(feature = "rdb")]
#[cfg_attr(docsrs, doc(cfg(feature = "rdb")))]
pub use ohos_rdb_sys as rdb;

#[cfg(feature = "sensors")]
#[cfg_attr(docsrs, doc(cfg(feature = "sensors")))]
pub use ohos_sensors_sys as sensors;

#[cfg(feature = "udmf")]
#[cfg_attr(docsrs, doc(cfg(feature = "udmf")))]
pub use udmf_sys as udmf;

#[cfg(feature = "native_buffer")]
#[cfg_attr(docsrs, doc(cfg(feature = "native_buffer")))]
pub use ohos_window_sys::native_buffer;

#[cfg(feature = "native_image")]
#[cfg_attr(docsrs, doc(cfg(feature = "native_image")))]
pub use ohos_window_sys::native_image;

#[cfg(feature = "native_window")]
#[cfg_attr(docsrs, doc(cfg(feature = "native_window")))]
pub use ohos_window_sys::native_window;

// It's just one function, so we don't feature guard this.
pub mod syscap;

#[cfg(feature = "vsync")]
#[cfg_attr(docsrs, doc(cfg(feature = "vsync")))]
pub use ohos_vsync_sys as vsync;

#[cfg(feature = "xcomponent")]
#[cfg_attr(docsrs, doc(cfg(feature = "xcomponent")))]
pub use xcomponent_sys as xcomponent;

#[cfg(feature = "window_manager")]
#[cfg_attr(docsrs, doc(cfg(feature = "window_manager")))]
pub use ohos_window_manager_sys as window_manager;
