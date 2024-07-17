#![allow(non_snake_case)]

use crate::drawing::types::{OH_Drawing_ColorFilter, OH_Drawing_Filter, OH_Drawing_ImageFilter};

extern "C" {
    /** @brief Sets an <b>OH_Drawing_ImageFilter</b> object for an <b>OH_Drawing_Filter</b> object.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Filter Indicates the pointer to an <b>OH_Drawing_Filter</b> object.
    @param OH_Drawing_ImageFilter Indicates the pointer to an <b>OH_Drawing_ImageFilter</b> object.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_FilterSetImageFilter(
        arg1: *mut OH_Drawing_Filter,
        arg2: *mut OH_Drawing_ImageFilter,
    );
    /** @brief Gets an <b>OH_Drawing_ColorFilter</b> object from an <b>OH_Drawing_Filter</b> object.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Filter Indicates the pointer to an <b>OH_Drawing_Filter</b> object.
    @param OH_Drawing_ColorFilter Indicates the pointer to an <b>OH_Drawing_ColorFilter</b> object.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_FilterGetColorFilter(
        arg1: *mut OH_Drawing_Filter,
        arg2: *mut OH_Drawing_ColorFilter,
    );
}
