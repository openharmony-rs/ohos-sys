// automatically generated by rust-bindgen 0.71.1

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use crate::types::*;

#[allow(unused_imports)]
#[cfg(feature = "api-12")]
use crate::error_code::OH_Drawing_ErrorCode;

extern "C" {
    /// Creates an <b>OH_Drawing_Surface</b> object on GPU indicated by context.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `OH_Drawing_GpuContext` - Indicates the pointer to an <b>OH_Drawing_GpuContext</b> object.
    ///
    /// * `bool` - Indicates whether an allocation should count against a cache budget.
    ///
    /// * `OH_Drawing_Image_Info` - Indicates the image info.
    ///
    /// # Returns
    ///
    /// * Returns the pointer to the <b>OH_Drawing_Surface</b> object created.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_SurfaceCreateFromGpuContext(
        arg1: *mut OH_Drawing_GpuContext,
        arg2: bool,
        arg3: OH_Drawing_Image_Info,
    ) -> *mut OH_Drawing_Surface;
    /// Gets the canvas that draws into surface.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `OH_Drawing_Surface` - Indicates the pointer to an <b>OH_Drawing_Surface</b> object.
    ///
    /// # Returns
    ///
    /// * Returns the pointer to the <b>OH_Drawing_Canvas</b> object. The returned pointer does not need to be managed
    /// by the caller.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_SurfaceGetCanvas(arg1: *mut OH_Drawing_Surface) -> *mut OH_Drawing_Canvas;
    /// Destroys an <b>OH_Drawing_Surface</b> object and reclaims the memory occupied by the object.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `OH_Drawing_Surface` - Indicates the pointer to an <b>OH_Drawing_Surface</b> object.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_SurfaceDestroy(arg1: *mut OH_Drawing_Surface);
}
