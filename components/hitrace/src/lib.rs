//! Raw bindings for the [`HiTrace`] tracing system on OpenHarmony
//!
//! The official [Hitrace NDK guidelines] describe the available API.
//! Rust users should likely not use this crate directly, but instead use an abstraction on
//! top of this crate.
//!
//! [`HiTrace`]: https://gitee.com/openharmony/hiviewdfx_hitrace
//! [Hitrace NDK guidelines]: https://gitee.com/openharmony/docs/blob/master/zh-cn/application-dev/dfx/hitracemeter-guidelines-ndk.md
#![cfg_attr(docsrs, feature(doc_cfg))]

#[link(name = "hitrace_ndk.z")]
extern "C" {}

mod hitrace_ffi;
pub use hitrace_ffi::*;
