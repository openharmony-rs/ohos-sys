//! ArkUI Native
//!
//! A module to interact with ArkUI from native Rust code. It provides:
//!
//! - drag and drop APIs of ArkUI
//! - APIs of ArkUI to register gesture callbacks.
//! - animation callbacks of ArkUI
//! - UI capabilities such as UI component creation and destruction, tree node operations,
//!   attribute setting, and event listening.
//!
//! See the official upstream documentation on [ArkUI Native] for more information.
//!
//! [ArkUI Native]: https://docs.openharmony.cn/pages/v5.0/zh-cn/application-dev/reference/apis-arkui/_ark_u_i___native_module.md
//!
//! ## Feature flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[link(name = "ace_ndk.z")]
extern "C" {}

#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
pub mod drag_and_drop;

#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
pub mod drawable_descriptor;

#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
pub mod native_animate;

#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
pub mod native_dialog;

#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
pub mod native_gesture;

#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
pub mod native_interface;

#[cfg(feature = "api-13")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-13")))]
pub mod native_interface_accessibility;

#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
pub mod native_node;

#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
pub mod native_node_napi;

#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
pub mod native_type;

#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
pub mod styled_string;

#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
pub mod ui_input_event;
