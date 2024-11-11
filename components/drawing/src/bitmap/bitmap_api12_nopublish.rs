/* automatically generated by rust-bindgen 0.70.1 */

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use crate::types::*;

/** @brief Defines the pixel format of a bitmap, including the color type and alpha type.

@since 8
@version 1.0*/
#[repr(C)]
pub struct OH_Drawing_BitmapFormat {
    /// Storage format of bitmap pixels
    pub colorFormat: OH_Drawing_ColorFormat,
    /// Alpha format of bitmap pixels
    pub alphaFormat: OH_Drawing_AlphaFormat,
}
extern "C" {
    /** @brief Creates an <b>OH_Drawing_Bitmap</b> object.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @return Returns the pointer to the <b>OH_Drawing_Bitmap</b> object created.
    @since 8
    @version 1.0*/
    pub fn OH_Drawing_BitmapCreate() -> *mut OH_Drawing_Bitmap;
    /** @brief Destroys an <b>OH_Drawing_Bitmap</b> object and reclaims the memory occupied by the object.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Bitmap Indicates the pointer to an <b>OH_Drawing_Bitmap</b> object.
    @since 8
    @version 1.0*/
    pub fn OH_Drawing_BitmapDestroy(arg1: *mut OH_Drawing_Bitmap);
    /** @brief Creates an <b>OH_Drawing_Bitmap</b> object with <b>OH_Drawing_Image_Info</b> object
    and sets the mem address or pixel storage.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Image_Info Indicates the pointer to an <b>OH_Drawing_Image_Info</b> object.
    @param pixels the pointer to memory address or pixel storage.
    @param rowBytes size of pixel row or larger.
    @return Returns the pointer to the <b>OH_Drawing_Bitmap</b> object created.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_BitmapCreateFromPixels(
        arg1: *mut OH_Drawing_Image_Info,
        pixels: *mut ::core::ffi::c_void,
        rowBytes: u32,
    ) -> *mut OH_Drawing_Bitmap;
    /** @brief Initializes the width and height of an <b>OH_Drawing_Bitmap</b> object
    and sets the pixel format for the bitmap.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Bitmap Indicates the pointer to an <b>OH_Drawing_Bitmap</b> object.
    @param width Indicates the width of the bitmap to be initialized.
    @param height Indicates the height of the bitmap to be initialized.
    @param OH_Drawing_BitmapFormat Indicates the pixel format of the bitmap to be initialized,
                                   including the pixel color type and alpha type.
    @since 8
    @version 1.0*/
    pub fn OH_Drawing_BitmapBuild(
        arg1: *mut OH_Drawing_Bitmap,
        width: u32,
        height: u32,
        arg2: *const OH_Drawing_BitmapFormat,
    );
    /** @brief Obtains the width of a bitmap.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Bitmap Indicates the pointer to an <b>OH_Drawing_Bitmap</b> object.
    @return Returns the width.
    @since 8
    @version 1.0*/
    pub fn OH_Drawing_BitmapGetWidth(arg1: *mut OH_Drawing_Bitmap) -> u32;
    /** @brief Obtains the height of a bitmap.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Bitmap Indicates the pointer to an <b>OH_Drawing_Bitmap</b> object.
    @return Returns the height.
    @since 8
    @version 1.0*/
    pub fn OH_Drawing_BitmapGetHeight(arg1: *mut OH_Drawing_Bitmap) -> u32;
    /** @brief Obtains the color format of a bitmap.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Bitmap Indicates the pointer to an <b>OH_Drawing_Bitmap</b> object.
    @return Returns the bitmap color format.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_BitmapGetColorFormat(arg1: *mut OH_Drawing_Bitmap) -> OH_Drawing_ColorFormat;
    /** @brief Obtains the alpha format of a bitmap.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Bitmap Indicates the pointer to an <b>OH_Drawing_Bitmap</b> object.
    @return Returns the bitmap alpha format.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_BitmapGetAlphaFormat(arg1: *mut OH_Drawing_Bitmap) -> OH_Drawing_AlphaFormat;
    /** @brief Obtains the pixel address of a bitmap. You can use this address to obtain the pixel data of the bitmap.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Bitmap Indicates the pointer to an <b>OH_Drawing_Bitmap</b> object.
    @return Returns the pixel address.
    @since 8
    @version 1.0*/
    pub fn OH_Drawing_BitmapGetPixels(arg1: *mut OH_Drawing_Bitmap) -> *mut ::core::ffi::c_void;
    /** @brief Gets the image info.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Bitmap Indicates the pointer to an <b>OH_Drawing_Bitmap</b> object.
    @param OH_Drawing_Image_Info Indicates the pointer to an <b>OH_Drawing_Image_Info</b> object.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_BitmapGetImageInfo(
        arg1: *mut OH_Drawing_Bitmap,
        arg2: *mut OH_Drawing_Image_Info,
    );
    /** @brief Copies a rect of pixels from bitmap to dstPixels. Copy starts at (srcX, srcY),
    and does not exceed bitmap width and height.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Bitmap Indicates the pointer to an <b>OH_Drawing_Bitmap</b> object.
    @param dstInfo Indicates the pointer to an <b>OH_Drawing_Image_Info</b> object.
    @param dstPixels Destination pixel storage.
    @param dstRowBytes Destination row length.
    @param srcX Column index whose absolute value is less than width.
    @param srcY Row index whose absolute value is less than height.
    @return Returns true if pixels are copied to dstPixels.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_BitmapReadPixels(
        arg1: *mut OH_Drawing_Bitmap,
        dstInfo: *const OH_Drawing_Image_Info,
        dstPixels: *mut ::core::ffi::c_void,
        dstRowBytes: usize,
        srcX: i32,
        srcY: i32,
    ) -> bool;
}
