use crate::types::{
    OH_Drawing_AlphaFormat, OH_Drawing_Bitmap, OH_Drawing_ColorFormat, OH_Drawing_Image_Info,
};

extern "C" {
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
