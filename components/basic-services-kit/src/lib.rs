//! Bindings to the OpenHarmony BasicServicesKit NDK.
//!
//! ## Overview
//!
//! [BasicServicesKit](https://gitcode.com/openharmony/docs/blob/master/en/application-dev/reference/apis-basic-services-kit/_o_h___common_event.md)
//! groups several unrelated system-service NDKs under one upstream kit. Each
//! lives in a separate shared library, so this crate exposes each behind its
//! own cargo feature:
//!
//! | Module                                    | Feature        | Library                  | Min API |
//! | ----------------------------------------- | -------------- | ------------------------ | ------- |
//! | [`commonevent`] / [`commonevent_support`] | `commonevent`  | `libohcommonevent.so`    | 12      |
//! | [`battery_info`]                          | `battery-info` | `libohbattery_info.so`   | 13      |
//! | [`print`]                                 | `print`        | `libohprint.so`          | 12      |
//! | [`scan`]                                  | `scan`         | `libohscan.so`           | 12      |
//! | [`os_account`] / [`os_account_common`]    | `os-account`   | `libos_account_ndk.so`   | 12      |
//! | [`time_service`]                          | `time-service` | `libtime_service_ndk.so` | 12      |
//!
//! A sub-API feature is a no-op unless the matching `api-XX` feature is also
//! enabled — i.e. enabling `commonevent` without `api-12` (or higher) leaves
//! the module hidden and the `.so` unlinked.
//!
//! ## Feature flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(all(feature = "commonevent", feature = "api-12"))]
#[cfg_attr(docsrs, doc(cfg(all(feature = "commonevent", feature = "api-12"))))]
pub mod commonevent;

#[cfg(all(feature = "commonevent", feature = "api-12"))]
#[cfg_attr(docsrs, doc(cfg(all(feature = "commonevent", feature = "api-12"))))]
pub mod commonevent_support;

#[cfg(all(feature = "commonevent", feature = "api-12"))]
#[link(name = "ohcommonevent")]
unsafe extern "C" {}

#[cfg(all(feature = "battery-info", feature = "api-13"))]
#[cfg_attr(docsrs, doc(cfg(all(feature = "battery-info", feature = "api-13"))))]
pub mod battery_info;

#[cfg(all(feature = "battery-info", feature = "api-13"))]
#[link(name = "ohbattery_info")]
unsafe extern "C" {}

#[cfg(all(feature = "print", feature = "api-12"))]
#[cfg_attr(docsrs, doc(cfg(all(feature = "print", feature = "api-12"))))]
pub mod print;

#[cfg(all(feature = "print", feature = "api-12"))]
#[link(name = "ohprint")]
unsafe extern "C" {}

#[cfg(all(feature = "scan", feature = "api-12"))]
#[cfg_attr(docsrs, doc(cfg(all(feature = "scan", feature = "api-12"))))]
pub mod scan;

#[cfg(all(feature = "scan", feature = "api-12"))]
#[link(name = "ohscan")]
unsafe extern "C" {}

#[cfg(all(feature = "os-account", feature = "api-12"))]
#[cfg_attr(docsrs, doc(cfg(all(feature = "os-account", feature = "api-12"))))]
pub mod os_account;

#[cfg(all(feature = "os-account", feature = "api-12"))]
#[cfg_attr(docsrs, doc(cfg(all(feature = "os-account", feature = "api-12"))))]
pub mod os_account_common;

#[cfg(all(feature = "os-account", feature = "api-12"))]
#[link(name = "os_account_ndk")]
unsafe extern "C" {}

#[cfg(all(feature = "time-service", feature = "api-12"))]
#[cfg_attr(docsrs, doc(cfg(all(feature = "time-service", feature = "api-12"))))]
pub mod time_service;

#[cfg(all(feature = "time-service", feature = "api-12"))]
#[link(name = "time_service_ndk")]
unsafe extern "C" {}
