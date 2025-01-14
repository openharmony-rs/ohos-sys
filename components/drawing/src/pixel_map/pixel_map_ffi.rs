// automatically generated by rust-bindgen 0.71.1

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use crate::types::*;

#[allow(unused_imports)]
#[cfg(feature = "api-12")]
use crate::error_code::OH_Drawing_ErrorCode;
use ohos_sys_opaque_types::{NativePixelMap_, OH_PixelmapNative};

extern "C" {
    /// Gets an <b>OH_Drawing_PixelMap</b> object.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `NativePixelMap_` - Indicates a pointer to an native pixelmap supported by image framework.
    ///
    /// # Returns
    ///
    /// * Returns the pointer to the <b>OH_Drawing_PixelMap</b> object.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_PixelMapGetFromNativePixelMap(
        arg1: *mut NativePixelMap_,
    ) -> *mut OH_Drawing_PixelMap;
    /// Gets an <b>OH_Drawing_PixelMap</b> object.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `OH_PixelmapNative` - Indicates a pointer to the <b>OH_PixelmapNative</b> object supported by image framework.
    ///
    /// # Returns
    ///
    /// * Returns the pointer to the <b>OH_Drawing_PixelMap</b> object.
    /// If nullptr is returned, the get operation fails.
    /// The possible cause of the failure is that a nullptr is passed.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_PixelMapGetFromOhPixelMapNative(
        arg1: *mut OH_PixelmapNative,
    ) -> *mut OH_Drawing_PixelMap;
    /// Dissolves the relationship between <b>OH_Drawing_PixelMap</b> object and <b>NativePixelMap_</b> or
    /// <b>OH_PixelmapNative</b> which is build by 'GetFrom' function.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `OH_Drawing_PixelMap` - Indicates a pointer to the <b>OH_Drawing_PixelMap</b>.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_PixelMapDissolve(arg1: *mut OH_Drawing_PixelMap);
}
