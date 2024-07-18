/* automatically generated by rust-bindgen 0.69.4 */

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use crate::types::*;

extern "C" {
    /** @brief Creates an <b>OH_Drawing_Rect</b> object.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param left Indicates the left position of the rect.
    @param top Indicates the top position of the rect.
    @param right Indicates the right position of the rect.
    @param bottom Indicates the bottom position of the rect.
    @return Returns the pointer to the <b>OH_Drawing_Rect</b> object created.
    @since 11
    @version 1.0*/
    pub fn OH_Drawing_RectCreate(
        left: f32,
        top: f32,
        right: f32,
        bottom: f32,
    ) -> *mut OH_Drawing_Rect;
    /** @brief Destroys an <b>OH_Drawing_Rect</b> object and reclaims the memory occupied by the object.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Rect Indicates the pointer to an <b>OH_Drawing_Rect</b> object.
    @since 11
    @version 1.0*/
    pub fn OH_Drawing_RectDestroy(arg1: *mut OH_Drawing_Rect);
}