//! USB DDK error codes changed numerical values in API level 16.

/// USB DDK error code values for the selected OpenHarmony API level.
///
/// The C functions return `i32`; compare their result with a constant's `.0` value.
#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct UsbDdkErrCode(pub i32);

#[allow(non_upper_case_globals)]
impl UsbDdkErrCode {
    /// Operation succeeded.
    pub const USB_DDK_SUCCESS: Self = Self(0);
    /// Operation failed; deprecated since API 16.
    pub const USB_DDK_FAILED: Self = Self(-1);
    /// Invalid parameter.
    #[cfg(not(feature = "api-16"))]
    pub const USB_DDK_INVALID_PARAMETER: Self = Self(-2);
    /// Invalid parameter.
    #[cfg(feature = "api-16")]
    pub const USB_DDK_INVALID_PARAMETER: Self = Self(401);
    /// Memory error.
    #[cfg(not(feature = "api-16"))]
    pub const USB_DDK_MEMORY_ERROR: Self = Self(-3);
    /// Memory error.
    #[cfg(feature = "api-16")]
    pub const USB_DDK_MEMORY_ERROR: Self = Self(27400001);
    /// Invalid operation.
    #[cfg(not(feature = "api-16"))]
    pub const USB_DDK_INVALID_OPERATION: Self = Self(-4);
    /// Invalid operation.
    #[cfg(feature = "api-16")]
    pub const USB_DDK_INVALID_OPERATION: Self = Self(27400002);
    /// Null pointer; deprecated since API 16.
    pub const USB_DDK_NULL_PTR: Self = Self(-5);
    /// Device busy; deprecated since API 16.
    pub const USB_DDK_DEVICE_BUSY: Self = Self(-6);
    /// Timeout.
    #[cfg(not(feature = "api-16"))]
    pub const USB_DDK_TIMEOUT: Self = Self(-7);
    /// Timeout.
    #[cfg(feature = "api-16")]
    pub const USB_DDK_TIMEOUT: Self = Self(27400004);
    /// Permission check failed; documented since API 14.
    #[cfg(any(feature = "api-14", doc))]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-14")))]
    pub const USB_DDK_NO_PERM: Self = Self(201);
    /// I/O failure; documented since API 14.
    #[cfg(any(feature = "api-14", doc))]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-14")))]
    pub const USB_DDK_IO_FAILED: Self = Self(27400003);
}
