// automatically generated by rust-bindgen 0.71.1

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use crate::types::*;

#[allow(unused_imports)]
#[cfg(feature = "api-12")]
use crate::error_code::OH_Drawing_ErrorCode;

/// Defines the pixel format of a bitmap, including the color type and alpha type.
///
///
/// Available since API-level: 8
///
/// Version: 1.0
#[repr(C)]
pub struct OH_Drawing_BitmapFormat {
    /// Storage format of bitmap pixels
    pub colorFormat: OH_Drawing_ColorFormat,
    /// Alpha format of bitmap pixels
    pub alphaFormat: OH_Drawing_AlphaFormat,
}
extern "C" {
    /// Creates an <b>OH_Drawing_Bitmap</b> object.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    ///
    /// # Returns
    ///
    /// Returns the pointer to the <b>OH_Drawing_Bitmap</b> object created.
    ///
    /// Available since API-level: 8
    ///
    /// Version: 1.0
    pub fn OH_Drawing_BitmapCreate() -> *mut OH_Drawing_Bitmap;
    /// Destroys an <b>OH_Drawing_Bitmap</b> object and reclaims the memory occupied by the object.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// `OH_Drawing_Bitmap` - Indicates the pointer to an <b>OH_Drawing_Bitmap</b> object.
    ///
    /// Available since API-level: 8
    ///
    /// Version: 1.0
    pub fn OH_Drawing_BitmapDestroy(arg1: *mut OH_Drawing_Bitmap);
    /// Creates an <b>OH_Drawing_Bitmap</b> object with <b>OH_Drawing_Image_Info</b> object
    /// and sets the mem address or pixel storage.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// `OH_Drawing_Image_Info` - Indicates the pointer to an <b>OH_Drawing_Image_Info</b> object.
    ///
    /// `pixels` - the pointer to memory address or pixel storage.
    ///
    /// `rowBytes` - size of pixel row or larger.
    ///
    /// # Returns
    ///
    /// Returns the pointer to the <b>OH_Drawing_Bitmap</b> object created.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_BitmapCreateFromPixels(
        arg1: *mut OH_Drawing_Image_Info,
        pixels: *mut ::core::ffi::c_void,
        rowBytes: u32,
    ) -> *mut OH_Drawing_Bitmap;
    /// Initializes the width and height of an <b>OH_Drawing_Bitmap</b> object
    /// and sets the pixel format for the bitmap.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// `OH_Drawing_Bitmap` - Indicates the pointer to an <b>OH_Drawing_Bitmap</b> object.
    ///
    /// `width` - Indicates the width of the bitmap to be initialized.
    ///
    /// `height` - Indicates the height of the bitmap to be initialized.
    ///
    /// `OH_Drawing_BitmapFormat` - Indicates the pixel format of the bitmap to be initialized,
    /// including the pixel color type and alpha type.
    ///
    /// Available since API-level: 8
    ///
    /// Version: 1.0
    pub fn OH_Drawing_BitmapBuild(
        arg1: *mut OH_Drawing_Bitmap,
        width: u32,
        height: u32,
        arg2: *const OH_Drawing_BitmapFormat,
    );
    /// Obtains the width of a bitmap.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// `OH_Drawing_Bitmap` - Indicates the pointer to an <b>OH_Drawing_Bitmap</b> object.
    ///
    /// # Returns
    ///
    /// Returns the width.
    ///
    /// Available since API-level: 8
    ///
    /// Version: 1.0
    pub fn OH_Drawing_BitmapGetWidth(arg1: *mut OH_Drawing_Bitmap) -> u32;
    /// Obtains the height of a bitmap.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// `OH_Drawing_Bitmap` - Indicates the pointer to an <b>OH_Drawing_Bitmap</b> object.
    ///
    /// # Returns
    ///
    /// Returns the height.
    ///
    /// Available since API-level: 8
    ///
    /// Version: 1.0
    pub fn OH_Drawing_BitmapGetHeight(arg1: *mut OH_Drawing_Bitmap) -> u32;
    /// Obtains the color format of a bitmap.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// `OH_Drawing_Bitmap` - Indicates the pointer to an <b>OH_Drawing_Bitmap</b> object.
    ///
    /// # Returns
    ///
    /// Returns the bitmap color format.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_BitmapGetColorFormat(arg1: *mut OH_Drawing_Bitmap) -> OH_Drawing_ColorFormat;
    /// Obtains the alpha format of a bitmap.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// `OH_Drawing_Bitmap` - Indicates the pointer to an <b>OH_Drawing_Bitmap</b> object.
    ///
    /// # Returns
    ///
    /// Returns the bitmap alpha format.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_BitmapGetAlphaFormat(arg1: *mut OH_Drawing_Bitmap) -> OH_Drawing_AlphaFormat;
    /// Obtains the pixel address of a bitmap. You can use this address to obtain the pixel data of the bitmap.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// `OH_Drawing_Bitmap` - Indicates the pointer to an <b>OH_Drawing_Bitmap</b> object.
    ///
    /// # Returns
    ///
    /// Returns the pixel address.
    ///
    /// Available since API-level: 8
    ///
    /// Version: 1.0
    pub fn OH_Drawing_BitmapGetPixels(arg1: *mut OH_Drawing_Bitmap) -> *mut ::core::ffi::c_void;
    /// Gets the image info.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// `OH_Drawing_Bitmap` - Indicates the pointer to an <b>OH_Drawing_Bitmap</b> object.
    ///
    /// `OH_Drawing_Image_Info` - Indicates the pointer to an <b>OH_Drawing_Image_Info</b> object.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_BitmapGetImageInfo(
        arg1: *mut OH_Drawing_Bitmap,
        arg2: *mut OH_Drawing_Image_Info,
    );
    /// Copies a rect of pixels from bitmap to dstPixels. Copy starts at (srcX, srcY),
    /// and does not exceed bitmap width and height.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// `OH_Drawing_Bitmap` - Indicates the pointer to an <b>OH_Drawing_Bitmap</b> object.
    ///
    /// `dstInfo` - Indicates the pointer to an <b>OH_Drawing_Image_Info</b> object.
    ///
    /// `dstPixels` - Destination pixel storage.
    ///
    /// `dstRowBytes` - Destination row length.
    ///
    /// `srcX` - Column index whose absolute value is less than width.
    ///
    /// `srcY` - Row index whose absolute value is less than height.
    ///
    /// # Returns
    ///
    /// Returns true if pixels are copied to dstPixels.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_BitmapReadPixels(
        arg1: *mut OH_Drawing_Bitmap,
        dstInfo: *const OH_Drawing_Image_Info,
        dstPixels: *mut ::core::ffi::c_void,
        dstRowBytes: usize,
        srcX: i32,
        srcY: i32,
    ) -> bool;
}
