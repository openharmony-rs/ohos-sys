/* automatically generated by rust-bindgen 0.70.1 */

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use crate::types::*;

/** @brief Defines a run, supplies storage for glyphs and positions.

@since 11
@version 1.0*/
#[repr(C)]
pub struct OH_Drawing_RunBuffer {
    /// storage for glyph indexes in run
    pub glyphs: *mut u16,
    /// storage for glyph positions in run
    pub pos: *mut f32,
    /// storage for text UTF-8 code units in run
    pub utf8text: *mut ::core::ffi::c_char,
    /// storage for glyph clusters (index of UTF-8 code unit)
    pub clusters: *mut u32,
}
extern "C" {
    /** @brief Creates an <b>OH_Drawing_TextBlobBuilder</b> object.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @return Returns the pointer to the <b>OH_Drawing_TextBlobBuilder</b> object created.
    @since 11
    @version 1.0*/
    pub fn OH_Drawing_TextBlobBuilderCreate() -> *mut OH_Drawing_TextBlobBuilder;
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
    /** @brief Alloc run with storage for glyphs and positions. The returned pointer does not need to be managed
    by the caller and is forbidden to be used after OH_Drawing_TextBlobBuilderMake is called.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TextBlobBuilder Indicates the pointer to an <b>OH_Drawing_TextBlobBuilder</b> object.
    @param OH_Drawing_Font Indicates the pointer to an <b>OH_Drawing_Font</b> object.
    @param count Indicates the number of glyphs.
    @param OH_Drawing_Rect Indicates the optional run bounding box.
    @since 11
    @version 1.0*/
    pub fn OH_Drawing_TextBlobBuilderAllocRunPos(
        arg1: *mut OH_Drawing_TextBlobBuilder,
        arg2: *const OH_Drawing_Font,
        count: i32,
        arg3: *const OH_Drawing_Rect,
    ) -> *const OH_Drawing_RunBuffer;
    /** @brief Make an <b>OH_Drawing_TextBlob</b> from <b>OH_Drawing_TextBlobBuilder</b>.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TextBlobBuilder Indicates the pointer to an <b>OH_Drawing_TextBlobBuilder</b> object.
    @return Returns the pointer to the <b>OH_Drawing_TextBlob</b> object.
    @since 11
    @version 1.0*/
    pub fn OH_Drawing_TextBlobBuilderMake(
        arg1: *mut OH_Drawing_TextBlobBuilder,
    ) -> *mut OH_Drawing_TextBlob;
    /** @brief Destroys an <b>OH_Drawing_TextBlob</b> object and reclaims the memory occupied by the object.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TextBlob Indicates the pointer to an <b>OH_Drawing_TextBlob</b> object.
    @since 11
    @version 1.0*/
    pub fn OH_Drawing_TextBlobDestroy(arg1: *mut OH_Drawing_TextBlob);
    /** @brief Destroys an <b>OH_Drawing_TextBlobBuilder</b> object and reclaims the memory occupied by the object.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TextBlobBuilder Indicates the pointer to an <b>OH_Drawing_TextBlobBuilder</b> object.
    @since 11
    @version 1.0*/
    pub fn OH_Drawing_TextBlobBuilderDestroy(arg1: *mut OH_Drawing_TextBlobBuilder);
}
