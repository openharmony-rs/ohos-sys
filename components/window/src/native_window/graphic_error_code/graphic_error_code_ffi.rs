// automatically generated by rust-bindgen 0.71.1

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
impl OHNativeErrorCode {
    /// succeed
    pub const NATIVE_ERROR_OK: OHNativeErrorCode = OHNativeErrorCode(0);
    /// memory operation error
    ///
    /// Available since API-level: 15
    #[cfg(feature = "api-15")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-15")))]
    pub const NATIVE_ERROR_MEM_OPERATION_ERROR: OHNativeErrorCode = OHNativeErrorCode(30001000);
    /// input invalid parameter
    pub const NATIVE_ERROR_INVALID_ARGUMENTS: OHNativeErrorCode = OHNativeErrorCode(40001000);
    /// unauthorized operation
    pub const NATIVE_ERROR_NO_PERMISSION: OHNativeErrorCode = OHNativeErrorCode(40301000);
    /// no idle buffer is available
    pub const NATIVE_ERROR_NO_BUFFER: OHNativeErrorCode = OHNativeErrorCode(40601000);
    /// the consumer side doesn't exist
    pub const NATIVE_ERROR_NO_CONSUMER: OHNativeErrorCode = OHNativeErrorCode(41202000);
    /// uninitialized
    pub const NATIVE_ERROR_NOT_INIT: OHNativeErrorCode = OHNativeErrorCode(41203000);
    /// the consumer is connected
    pub const NATIVE_ERROR_CONSUMER_CONNECTED: OHNativeErrorCode = OHNativeErrorCode(41206000);
    /// the buffer status did not meet expectations
    pub const NATIVE_ERROR_BUFFER_STATE_INVALID: OHNativeErrorCode = OHNativeErrorCode(41207000);
    /// buffer is already in the cache queue
    pub const NATIVE_ERROR_BUFFER_IN_CACHE: OHNativeErrorCode = OHNativeErrorCode(41208000);
    /// the buffer queue is full
    pub const NATIVE_ERROR_BUFFER_QUEUE_FULL: OHNativeErrorCode = OHNativeErrorCode(41209000);
    /// buffer is not in the cache queue
    pub const NATIVE_ERROR_BUFFER_NOT_IN_CACHE: OHNativeErrorCode = OHNativeErrorCode(41210000);
    /// the consumer is disconnected
    pub const NATIVE_ERROR_CONSUMER_DISCONNECTED: OHNativeErrorCode = OHNativeErrorCode(41211000);
    /// the consumer not register listener
    pub const NATIVE_ERROR_CONSUMER_NO_LISTENER_REGISTERED: OHNativeErrorCode =
        OHNativeErrorCode(41212000);
    /// the current device or platform does not support it
    pub const NATIVE_ERROR_UNSUPPORTED: OHNativeErrorCode = OHNativeErrorCode(50102000);
    /// unknown error, please check log
    pub const NATIVE_ERROR_UNKNOWN: OHNativeErrorCode = OHNativeErrorCode(50002000);
    /// hdi interface error
    pub const NATIVE_ERROR_HDI_ERROR: OHNativeErrorCode = OHNativeErrorCode(50007000);
    /// ipc send failed
    pub const NATIVE_ERROR_BINDER_ERROR: OHNativeErrorCode = OHNativeErrorCode(50401000);
    /// the egl environment is abnormal
    pub const NATIVE_ERROR_EGL_STATE_UNKNOWN: OHNativeErrorCode = OHNativeErrorCode(60001000);
    /// egl interface invocation failed
    pub const NATIVE_ERROR_EGL_API_FAILED: OHNativeErrorCode = OHNativeErrorCode(60002000);
}
#[repr(transparent)]
/// interface error code.
///
/// Available since API-level: 12
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct OHNativeErrorCode(pub ::core::ffi::c_uint);
