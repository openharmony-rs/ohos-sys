#![allow(non_snake_case)]

use crate::drawing::types::{
    OH_Drawing_Font, OH_Drawing_Point2D, OH_Drawing_Rect, OH_Drawing_TextBlob,
    OH_Drawing_TextEncoding,
};

extern "C" {
    /** @brief Creates an <b>OH_Drawing_TextBlob</b> object from text.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param text Indicates the the pointer to text.
    @param byteLength Indicates the text length.
    @param OH_Drawing_Font Indicates the pointer to an <b>OH_Drawing_Font</b> object.
    @param OH_Drawing_TextEncoding Indicates the pointer to an <b>OH_Drawing_TextEncoding</b> object.
    @return Returns the pointer to the <b>OH_Drawing_TextBlob</b> object created.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_TextBlobCreateFromText(
        text: *const ::core::ffi::c_void,
        byteLength: usize,
        arg1: *const OH_Drawing_Font,
        arg2: OH_Drawing_TextEncoding,
    ) -> *mut OH_Drawing_TextBlob;
    /** @brief Creates an <b>OH_Drawing_TextBlob</b> object from pos text.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param text Indicates the the pointer to text.
    @param byteLength Indicates the text length.
    @param OH_Drawing_Point2D Indicates the pointer to an <b>OH_Drawing_Point2D</b> array object.
    @param OH_Drawing_Font Indicates the pointer to an <b>OH_Drawing_Font</b> object.
    @param OH_Drawing_TextEncoding Indicates the pointer to an <b>OH_Drawing_TextEncoding</b> object.
    @return Returns the pointer to the <b>OH_Drawing_TextBlob</b> object created.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_TextBlobCreateFromPosText(
        text: *const ::core::ffi::c_void,
        byteLength: usize,
        arg1: *mut OH_Drawing_Point2D,
        arg2: *const OH_Drawing_Font,
        arg3: OH_Drawing_TextEncoding,
    ) -> *mut OH_Drawing_TextBlob;
    /** @brief Creates an <b>OH_Drawing_TextBlob</b> object from pos text.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param str Indicates the the pointer to text.
    @param OH_Drawing_Font Indicates the pointer to an <b>OH_Drawing_Font</b> object.
    @param OH_Drawing_TextEncoding Indicates the pointer to an <b>OH_Drawing_TextEncoding</b> object.
    @return Returns the pointer to the <b>OH_Drawing_TextBlob</b> object created.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_TextBlobCreateFromString(
        str_: *const ::core::ffi::c_char,
        arg1: *const OH_Drawing_Font,
        arg2: OH_Drawing_TextEncoding,
    ) -> *mut OH_Drawing_TextBlob;
    /** @brief Gets the bounds of textblob, assigned to the pointer to an <b>OH_Drawing_Rect</b> object.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TextBlob Indicates the pointer to an <b>OH_Drawing_TextBlob</b> object.
    @param OH_Drawing_Rect Indicates the pointer to an <b>OH_Drawing_Rect</b> object.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_TextBlobGetBounds(arg1: *mut OH_Drawing_TextBlob, arg2: *mut OH_Drawing_Rect);
    /** @brief Gets a non-zero value unique among all <b>OH_Drawing_TextBlob</b> objects.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TextBlob Indicates the pointer to an <b>OH_Drawing_TextBlob</b> object.
    @return Returns identifier for the <b>OH_Drawing_TextBlob</b> object.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_TextBlobUniqueID(arg1: *const OH_Drawing_TextBlob) -> u32;
}
