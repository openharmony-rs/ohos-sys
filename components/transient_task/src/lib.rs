//! Bindings to the OpenHarmony transient task API (`BackgroundTasksKit`).
//!
//! Transient tasks let an application request a short delay before it is suspended
//! after moving to the background, e.g. to finish saving state.
//!
//! See also the [transient task development guide](https://gitcode.com/openharmony/docs/blob/master/en/application-dev/task-management/transient-task.md).
//!
//! ## Feature flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "api-13")]
#[link(name = "transient_task")]
unsafe extern "C" {}

#[cfg(feature = "api-13")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-13")))]
pub mod transient_task_api;

#[cfg(feature = "api-13")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-13")))]
pub mod transient_task_type;
