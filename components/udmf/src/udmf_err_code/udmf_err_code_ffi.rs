// automatically generated by rust-bindgen 0.71.1

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
impl Udmf_ErrCode {
    /// The error code in the correct case.
    pub const E_OK: Udmf_ErrCode = Udmf_ErrCode(0);
    /// The error code for common exceptions.
    pub const ERR: Udmf_ErrCode = Udmf_ErrCode(20400000);
    /// The error code for common invalid args.
    pub const E_INVALID_PARAM: Udmf_ErrCode = Udmf_ErrCode(20400001);
}
#[repr(transparent)]
/// Indicates the error code information.
///
///
/// Available since API-level: 12
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Udmf_ErrCode(pub ::core::ffi::c_uint);
#[cfg(feature = "api-15")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-15")))]
impl Udmf_ListenerStatus {
    /// brief Indicates the finished status.
    pub const UDMF_FINISHED: Udmf_ListenerStatus = Udmf_ListenerStatus(0);
    /// Indicates that processing is still in progress.
    pub const UDMF_PROCESSING: Udmf_ListenerStatus = Udmf_ListenerStatus(1);
    /// Indicates that the process has been canceled.
    pub const UDMF_CANCELED: Udmf_ListenerStatus = Udmf_ListenerStatus(2);
    /// Indicates that an internal error has occurred.
    pub const UDMF_INNER_ERROR: Udmf_ListenerStatus = Udmf_ListenerStatus(200);
    /// Indicates that the GetDataParams contains invalid parameters.
    pub const UDMF_INVALID_PARAMETERS: Udmf_ListenerStatus = Udmf_ListenerStatus(201);
    /// Indicates that no data is obtained.
    pub const UDMF_DATA_NOT_FOUND: Udmf_ListenerStatus = Udmf_ListenerStatus(202);
    /// Indicates that an error occurred in the synchronization process.
    pub const UDMF_SYNC_FAILED: Udmf_ListenerStatus = Udmf_ListenerStatus(203);
    /// Indicates that an error occurred during file copying.
    pub const UDMF_COPY_FILE_FAILED: Udmf_ListenerStatus = Udmf_ListenerStatus(204);
}
#[repr(transparent)]
/// Indicates the error code information.
///
///
/// Available since API-level: 15
#[cfg(feature = "api-15")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-15")))]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Udmf_ListenerStatus(pub ::core::ffi::c_uint);
