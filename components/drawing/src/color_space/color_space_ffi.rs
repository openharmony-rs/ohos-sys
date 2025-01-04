// automatically generated by rust-bindgen 0.71.1

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use crate::types::*;

#[allow(unused_imports)]
#[cfg(feature = "api-12")]
use crate::error_code::OH_Drawing_ErrorCode;

extern "C" {
    /// Creates an <b>OH_Drawing_ColorSpace</b> object that represents the SRGB color space.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    ///
    /// # Returns
    ///
    /// Returns the pointer to the <b>OH_Drawing_ColorSpace</b> object created.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_ColorSpaceCreateSrgb() -> *mut OH_Drawing_ColorSpace;
    /// Creates an <b>OH_Drawing_ColorSpace</b> object with the SRGB primaries, but a linear (1.0) gamma.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    ///
    /// # Returns
    ///
    /// Returns the pointer to the <b>OH_Drawing_ColorSpace</b> object created.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_ColorSpaceCreateSrgbLinear() -> *mut OH_Drawing_ColorSpace;
    /// Destroy an <b>OH_Drawing_ColorSpace</b> object.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// `OH_Drawing_ColorSpace` - Indicates the pointer to an <b>OH_Drawing_ColorSpace</b> object.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_ColorSpaceDestroy(arg1: *mut OH_Drawing_ColorSpace);
}
