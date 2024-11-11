#![allow(non_snake_case)]

use crate::text_declaration::OH_Drawing_FontCollection;

extern "C" {
    /** @brief Disable the font collection fallback.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_FontCollection Indicates the pointer to an <b>OH_Drawing_FontCollection</b> object.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_DisableFontCollectionFallback(arg1: *mut OH_Drawing_FontCollection);
    /** @brief Disable the font collection systemfont.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_FontCollection Indicates the pointer to an <b>OH_Drawing_FontCollection</b> object.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_DisableFontCollectionSystemFont(arg1: *mut OH_Drawing_FontCollection);
    /** @brief Creates an <b>OH_Drawing_FontCollection</b> object with shared usage between
    <b>OH_Drawing_TypographyCreate</b>.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @return Returns the pointer to the <b>OH_Drawing_FontCollection</b> object created.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_CreateSharedFontCollection() -> *mut OH_Drawing_FontCollection;
    /** @brief Clear font caches.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_FontCollection Indicates the pointer to an <b>OH_Drawing_FontCollection</b> object.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_ClearFontCaches(arg1: *mut OH_Drawing_FontCollection);
}
