//! Bindings to the OpenHarmony hiappevent API (`HiAppEvent`).
//!
//! HiAppEvent is an event subscription and event recording mechanism
//! provided by the system for app developers.
//!
//! See also the [HiAppEvent development guide](https://gitcode.com/openharmony/docs/blob/master/en/application-dev/dfx/hiappevent-intro.md).
//!
//! ## Feature flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[link(name = "hiappevent_ndk.z")]
unsafe extern "C" {}

pub mod hiappevent;

pub mod hiappevent_cfg;

pub mod hiappevent_param;

pub mod hiappevent_event;
