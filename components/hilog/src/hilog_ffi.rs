// automatically generated by rust-bindgen 0.71.1

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

impl LogType {
    /// Third-party application logs
    pub const LOG_APP: LogType = LogType(0);
}
#[repr(transparent)]
/// Enumerates log types.
///
/// Currently, <b>LOG_APP</b> is available.
///
///
///
/// Available since API-level: 8
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct LogType(pub ::core::ffi::c_uint);
impl LogLevel {
    /// Debug level to be used by [`OH_LOG_DEBUG`]
    pub const LOG_DEBUG: LogLevel = LogLevel(3);
    /// Informational level to be used by [`OH_LOG_INFO`]
    pub const LOG_INFO: LogLevel = LogLevel(4);
    /// Warning level to be used by [`OH_LOG_WARN`]
    pub const LOG_WARN: LogLevel = LogLevel(5);
    /// Error level to be used by [`OH_LOG_ERROR`]
    pub const LOG_ERROR: LogLevel = LogLevel(6);
    /// Fatal level to be used by [`OH_LOG_FATAL`]
    pub const LOG_FATAL: LogLevel = LogLevel(7);
}
#[repr(transparent)]
/// Enumerates log levels.
///
/// You are advised to select log levels based on their respective usage scenarios:
///
/// <ul><li><b>DEBUG</b>: used for debugging and disabled from commercial releases</li>
///
/// <li><b>INFO</b>: used for logging important system running status and steps in key processes</li>
///
/// <li><b>WARN</b>: used for logging unexpected exceptions that have little impact on user experience and can
/// automatically recover. Logs at this level are generally output when such exceptions are detected and
/// captured.</li>
///
/// <li><b>ERROR</b>: used for logging malfunction that affects user experience and cannot automatically
/// recover</li>
///
/// <li><b>FATAL</b>: used for logging major exceptions that have severely affected user experience and should
/// not occur.</li></ul>
///
///
///
/// Available since API-level: 8
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct LogLevel(pub ::core::ffi::c_uint);
/// Defines the function pointer type for the user-defined log processing function.
///
/// # Arguments
///
/// * `type` - Indicates the log type. The type for third-party applications is defined by [`LOG_APP`].
///
/// * `level` - Indicates the log level, which can be <b>LOG_DEBUG</b>, <b>LOG_INFO</b>, <b>LOG_WARN</b>,
/// <b>LOG_ERROR</b>, and <b>LOG_FATAL</b>.
///
/// * `domain` - Indicates the service domain of logs. Its value is a hexadecimal integer ranging from 0x0 to 0xFFFF.
///
/// * `tag` - Indicates the log tag, which is a string used to identify the class, file, or service behavior.
///
/// * `msg` - Indicates the log message itself, which is a formatted log string.
///
/// Available since API-level: 11
#[cfg(feature = "api-11")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-11")))]
pub type LogCallback = ::core::option::Option<
    unsafe extern "C" fn(
        type_: LogType,
        level: LogLevel,
        domain: ::core::ffi::c_uint,
        tag: *const ::core::ffi::c_char,
        msg: *const ::core::ffi::c_char,
    ),
>;
extern "C" {
    /// Outputs logs.
    ///
    /// You can use this function to output logs based on the specified log type, log level, service domain, log tag,
    /// and variable parameters determined by the format specifier and privacy identifier in the printf format.
    ///
    /// # Arguments
    ///
    /// * `type` - Indicates the log type. The type for third-party applications is defined by [`LOG_APP`].
    ///
    /// * `level` - Indicates the log level, which can be <b>LOG_DEBUG</b>, <b>LOG_INFO</b>, <b>LOG_WARN</b>,
    /// <b>LOG_ERROR</b>, and <b>LOG_FATAL</b>.
    ///
    /// * `domain` - Indicates the service domain of logs. Its value is a hexadecimal integer ranging from 0x0 to 0xFFFF.
    ///
    /// * `tag` - Indicates the log tag, which is a string used to identify the class, file, or service behavior.
    ///
    /// * `fmt` - Indicates the format string, which is an enhancement of a printf format string and supports the privacy
    /// identifier. Specifically, {public} or {private} is added between the % character and the format specifier
    /// in each parameter.
    ///
    ///
    /// * `...` - Indicates a list of parameters. The number and type of parameters must map onto the format specifiers
    /// in the format string.
    ///
    /// # Returns
    ///
    /// * Returns <b>0</b> or a larger value if the operation is successful; returns a value smaller
    /// than <b>0</b> otherwise.
    ///
    /// Available since API-level: 8
    pub fn OH_LOG_Print(
        type_: LogType,
        level: LogLevel,
        domain: ::core::ffi::c_uint,
        tag: *const ::core::ffi::c_char,
        fmt: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    /// Checks whether logs of the specified service domain, log tag, and log level can be output.
    ///
    /// # Arguments
    ///
    /// * `domain` - Indicates the service domain of logs.
    ///
    /// * `tag` - Indicates the log tag.
    ///
    /// * `level` - Indicates the log level.
    ///
    /// # Returns
    ///
    /// * Returns <b>true</b> if the specified logs can be output; returns <b>false</b> otherwise.
    ///
    /// Available since API-level: 8
    pub fn OH_LOG_IsLoggable(
        domain: ::core::ffi::c_uint,
        tag: *const ::core::ffi::c_char,
        level: LogLevel,
    ) -> bool;
    /// Set the user-defined log processing function.
    ///
    /// After calling this function, the callback function implemented by the user can receive all hilogs of the
    /// current process.
    /// Note that it will not change the default behavior of hilog logs of the current process, no matter whether this
    /// interface is called or not.
    ///
    ///
    /// # Arguments
    ///
    /// * `callback` - Indicates the callback function implemented by the user. If you do not need to process hilog logs,
    /// you can transfer a null pointer.
    ///
    /// Available since API-level: 11
    #[cfg(feature = "api-11")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-11")))]
    pub fn OH_LOG_SetCallback(callback: LogCallback);
}
