// automatically generated by rust-bindgen 0.71.1

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use crate::types::*;

#[allow(unused_imports)]
#[cfg(feature = "api-12")]
use crate::error_code::OH_Drawing_ErrorCode;

#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
impl OH_Drawing_FilterMode {
    /// single sample point (nearest neighbor)
    pub const FILTER_MODE_NEAREST: OH_Drawing_FilterMode = OH_Drawing_FilterMode(0);
}
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
impl OH_Drawing_FilterMode {
    /// interporate between 2x2 sample points (bilinear interpolation)
    pub const FILTER_MODE_LINEAR: OH_Drawing_FilterMode = OH_Drawing_FilterMode(1);
}
#[repr(transparent)]
/// Enumerates storage filter mode.
///
///
/// Available since API-level: 12
///
/// Version: 1.0
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct OH_Drawing_FilterMode(pub ::core::ffi::c_uint);
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
impl OH_Drawing_MipmapMode {
    /// ignore mipmap levels, sample from the "base"
    pub const MIPMAP_MODE_NONE: OH_Drawing_MipmapMode = OH_Drawing_MipmapMode(0);
}
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
impl OH_Drawing_MipmapMode {
    /// sample from the nearest level
    pub const MIPMAP_MODE_NEAREST: OH_Drawing_MipmapMode = OH_Drawing_MipmapMode(1);
}
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
impl OH_Drawing_MipmapMode {
    /// interpolate between the two nearest levels
    pub const MIPMAP_MODE_LINEAR: OH_Drawing_MipmapMode = OH_Drawing_MipmapMode(2);
}
#[repr(transparent)]
/// Enumerates storage formats mipmap mode.
///
///
/// Available since API-level: 12
///
/// Version: 1.0
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct OH_Drawing_MipmapMode(pub ::core::ffi::c_uint);
extern "C" {
    /// Creates an <b>OH_Drawing_SamplingOptions</b> object.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `filterMode` - sampling filter mode.
    ///
    /// * `mipmapMode` - sampling mipmap mode..
    ///
    /// # Returns
    ///
    /// * Returns the pointer to the <b>OH_Drawing_SamplingOptions</b> object created.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_SamplingOptionsCreate(
        filterMode: OH_Drawing_FilterMode,
        mipmapMode: OH_Drawing_MipmapMode,
    ) -> *mut OH_Drawing_SamplingOptions;
    /// Destroys an <b>OH_Drawing_SamplingOptions</b> object and reclaims the memory occupied by the object.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `samplingOptions` - Indicates the pointer to an <b>OH_Drawing_SamplingOptions</b> object.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_SamplingOptionsDestroy(samplingOptions: *mut OH_Drawing_SamplingOptions);
}
