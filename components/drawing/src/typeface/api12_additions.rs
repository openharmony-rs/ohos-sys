#![allow(non_snake_case)]

use crate::types::{OH_Drawing_MemoryStream, OH_Drawing_Typeface};

extern "C" {
    /** @brief Creates a <b>OH_Drawing_Typeface</b> object by file.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param path  file path.
    @param index  file index.
    @return Returns the pointer to the <b>OH_Drawing_Typeface</b> object created.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_TypefaceCreateFromFile(
        path: *const ::core::ffi::c_char,
        index: ::core::ffi::c_int,
    ) -> *mut OH_Drawing_Typeface;
    /** @brief Creates a <b>OH_Drawing_Typeface</b> object by given a stream. If the stream is not a valid
    font file, returns nullptr. Ownership of the stream is transferred, so the caller must not reference
    it or free it again.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_MemoryStream Indicates the pointer to an <b>OH_Drawing_MemoryStream</b> object.
    @param index  memory stream index.
    @return Returns the pointer to the <b>OH_Drawing_Typeface</b> object created.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_TypefaceCreateFromStream(
        arg1: *mut OH_Drawing_MemoryStream,
        index: i32,
    ) -> *mut OH_Drawing_Typeface;
}
