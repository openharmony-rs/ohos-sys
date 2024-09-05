//! Raw bindings for the [`HiTrace`] tracing system on OpenHarmony
//!
//! The official [Hitrace NDK guidelines] describe the available API.
//! Rust users should likely not use this crate directly, but instead use an abstraction on
//! top of this crate.
//!
//! [`HiTrace`]: https://gitee.com/openharmony/hiviewdfx_hitrace
//! [Hitrace NDK guidelines]: https://gitee.com/openharmony/docs/blob/master/zh-cn/application-dev/dfx/hitracemeter-guidelines-ndk.md

#[link(name = "hitrace_ndk.z")]
extern "C" {}

mod hitrace_api10;
pub use hitrace_api10::*;

#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
mod api12_additions;
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
pub use api12_additions::*;
