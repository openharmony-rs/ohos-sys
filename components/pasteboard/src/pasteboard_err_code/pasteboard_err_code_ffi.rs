// automatically generated by rust-bindgen 0.71.1

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[cfg(feature = "api-13")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-13")))]
impl PASTEBOARD_ErrCode {
    /// The operation is successful.
    pub const ERR_OK: PASTEBOARD_ErrCode = PASTEBOARD_ErrCode(0);
    /// Permission verification failed.
    pub const ERR_PERMISSION_ERROR: PASTEBOARD_ErrCode = PASTEBOARD_ErrCode(201);
    /// Invalid parameter is detected.
    pub const ERR_INVALID_PARAMETER: PASTEBOARD_ErrCode = PASTEBOARD_ErrCode(401);
    /// The capability is not supported.
    pub const ERR_DEVICE_NOT_SUPPORTED: PASTEBOARD_ErrCode = PASTEBOARD_ErrCode(801);
    /// Inner error.
    pub const ERR_INNER_ERROR: PASTEBOARD_ErrCode = PASTEBOARD_ErrCode(12900000);
    /// Another copy is in progress.
    pub const ERR_BUSY: PASTEBOARD_ErrCode = PASTEBOARD_ErrCode(12900003);
}
#[repr(transparent)]
/// Enumerates the error codes.
///
///
/// Available since API-level: 13
#[cfg(feature = "api-13")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-13")))]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct PASTEBOARD_ErrCode(pub ::core::ffi::c_uint);
