//! hilog-sys
//!
//! Rust bindings for the `HiLog` logging framework of OpenHarmony.
//! This crate should only be used on OpenHarmony (`target_env = "ohos"`).
//! More information on hilog in native applications is available in the [hilog native guidelines].
//!
//! ## Safety
//!
//! When using `OH_LOG_Print` from Rust you **must** ensure that the `fmt` parameter either
//! - Does not contain any `printf` style format specifiers (like `%s`, `%d`) OR
//! - `fmt` is `"${public}s\0"` and the actual string is passed as the following parameter.
//!
//! ## Crate Features
//!
//! * `log`: When the log feature is enabled, a `From<log::Level>` implementation is added
//!          to easily convert from `log`s log level to HiLogs log level.
//!
//! [hilog native guidelines]: https://gitee.com/openharmony/docs/blob/master/en/application-dev/dfx/hilog-guidelines.md

#[allow(non_snake_case)]
mod ffi;

pub use ffi::*;

#[cfg(feature = "log")]
impl From<log::Level> for LogLevel {
    fn from(level: log::Level) -> Self {
        // Note: log:::Level::Trace has no corresponding HiLog level.
        match level {
            log::Level::Error => LogLevel::LOG_ERROR,
            log::Level::Warn => LogLevel::LOG_WARN,
            log::Level::Info => LogLevel::LOG_INFO,
            log::Level::Debug => LogLevel::LOG_DEBUG,
            log::Level::Trace => LogLevel::LOG_DEBUG,
        }
    }
}
