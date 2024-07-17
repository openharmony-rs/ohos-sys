#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::types::{OH_Drawing_Font, OH_Drawing_TextEncoding, OH_Drawing_Typeface};

impl OH_Drawing_FontHinting {
    /// glyph outlines unchanged
    pub const FONT_HINTING_NONE: OH_Drawing_FontHinting = OH_Drawing_FontHinting(0);
}
impl OH_Drawing_FontHinting {
    /// minimal modification to improve contrast
    pub const FONT_HINTING_SLIGHT: OH_Drawing_FontHinting = OH_Drawing_FontHinting(1);
}
impl OH_Drawing_FontHinting {
    /// glyph outlines modified to improve contrast
    pub const FONT_HINTING_NORMAL: OH_Drawing_FontHinting = OH_Drawing_FontHinting(2);
}
impl OH_Drawing_FontHinting {
    /// modifies glyph outlines for maximum contrast
    pub const FONT_HINTING_FULL: OH_Drawing_FontHinting = OH_Drawing_FontHinting(3);
}
#[repr(transparent)]
/** @brief Enumerates font hinting pattern.

@since 12
@version 1.0*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct OH_Drawing_FontHinting(pub ::core::ffi::c_uint);
impl OH_Drawing_FontEdging {
    /// no transparent pixels on glyph edges
    pub const FONT_EDGING_ALIAS: OH_Drawing_FontEdging = OH_Drawing_FontEdging(0);
}
impl OH_Drawing_FontEdging {
    /// may have transparent pixels on glyph edges
    pub const FONT_EDGING_ANTI_ALIAS: OH_Drawing_FontEdging = OH_Drawing_FontEdging(1);
}
impl OH_Drawing_FontEdging {
    /// glyph positioned in pixel using transparency
    pub const FONT_EDGING_SUBPIXEL_ANTI_ALIAS: OH_Drawing_FontEdging = OH_Drawing_FontEdging(2);
}
#[repr(transparent)]
/** @brief Enumerates font edging effect.

@since 12
@version 1.0*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct OH_Drawing_FontEdging(pub ::core::ffi::c_uint);
extern "C" {
    /** @brief Sets whether the font baselines and pixels alignment when the transformation matrix is ​​axis aligned.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Font Indicates the pointer to an <b>OH_Drawing_Font</b> object.
    @param baselineSnap Indicates whether the font baselines and pixels alignment.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_FontSetBaselineSnap(arg1: *mut OH_Drawing_Font, baselineSnap: bool);

    /** @brief Gets whether the font baselines and pixels alignment when the transformation matrix is ​​axis aligned.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Font Indicates the pointer to an <b>OH_Drawing_Font</b> object.
    @return Returns <b>true</b> if the font baselines and pixels alignment; returns <b>false</b> otherwise.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_FontIsBaselineSnap(arg1: *const OH_Drawing_Font) -> bool;

    /** @brief Sets whether the font uses sub-pixel rendering.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Font Indicates the pointer to an <b>OH_Drawing_Font</b> object.
    @param isSubpixel Indicates whether the font uses sub-pixel rendering.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_FontSetSubpixel(arg1: *mut OH_Drawing_Font, isSubpixel: bool);

    /** @brief Gets whether the font uses sub-pixel rendering.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Font Indicates the pointer to an <b>OH_Drawing_Font</b> object.
    @return Returns <b>true</b> if the font uses sub-pixel rendering; returns <b>false</b> otherwise.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_FontIsSubpixel(arg1: *const OH_Drawing_Font) -> bool;

    /** @brief Sets whether the font outline is automatically adjusted.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Font Indicates the pointer to an <b>OH_Drawing_Font</b> object.
    @param isForceAutoHinting Indicates whether the font outline is automatically adjusted.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_FontSetForceAutoHinting(arg1: *mut OH_Drawing_Font, isForceAutoHinting: bool);

    /** @brief Gets whether the font outline is automatically adjusted.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Font Indicates the pointer to an <b>OH_Drawing_Font</b> object.
    @return Returns <b>true</b> if the font outline is automatically adjusted; returns <b>false</b> otherwise.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_FontIsForceAutoHinting(arg1: *const OH_Drawing_Font) -> bool;

    /** @brief Gets an <b>OH_Drawing_Typeface</b> object from the <b>OH_Drawing_Typeface</b> object.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Font Indicates the pointer to an <b>OH_Drawing_Font</b> object.
    @return OH_Drawing_Typeface Indicates the pointer to an <b>OH_Drawing_Typeface</b> object.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_FontGetTypeface(arg1: *mut OH_Drawing_Font) -> *mut OH_Drawing_Typeface;
}

extern "C" {
    /** @brief Gets text size for an <b>OH_Drawing_Font</b> object.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Font Indicates the pointer to an <b>OH_Drawing_Font</b> object.
    @return Returns the size of text.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_FontGetTextSize(arg1: *const OH_Drawing_Font) -> f32;

    /** @brief Calculate number of glyphs represented by text.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Font Indicates the pointer to an <b>OH_Drawing_Font</b> object.
    @param text Indicates the character storage encoded with text encoding.
    @param byteLength Indicates the text length in bytes.
    @param encoding Indicates the text encoding.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_FontCountText(
        arg1: *mut OH_Drawing_Font,
        text: *const ::core::ffi::c_void,
        byteLength: usize,
        encoding: OH_Drawing_TextEncoding,
    ) -> ::core::ffi::c_int;

    /** @brief Converts text into glyph indices.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Font Indicates the pointer to an <b>OH_Drawing_Font</b> object.
    @param text Indicates the character storage encoded with text encoding.
    @param byteLength Indicates the text length in bytes.
    @param encoding Indicates the text encoding.
    @param glyphs Indicates the storage for glyph indices.
    @param maxGlyphCount Indicates the storage capacity.
    @return Returns the number of glyph indices represented by text.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_FontTextToGlyphs(
        arg1: *const OH_Drawing_Font,
        text: *const ::core::ffi::c_void,
        byteLength: u32,
        encoding: OH_Drawing_TextEncoding,
        glyphs: *mut u16,
        maxGlyphCount: ::core::ffi::c_int,
    ) -> u32;

    /** @brief Retrieves the advance for each glyph in glyphs.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Font Indicates the pointer to an <b>OH_Drawing_Font</b> object.
    @param glyphs Indicates the array of glyph indices to be measured.
    @param count Indicates the number of glyphs.
    @param widths Indicates the text advances for each glyph returned to the caller.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_FontGetWidths(
        arg1: *const OH_Drawing_Font,
        glyphs: *const u16,
        count: ::core::ffi::c_int,
        widths: *mut f32,
    );
}

extern "C" {
    /** @brief Gets whether the font is linearly scalable.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Font Indicates the pointer to an <b>OH_Drawing_Font</b> object.
    @return Returns <b>true</b> if the font is linearly scalable; returns <b>false</b> otherwise.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_FontIsLinearText(arg1: *const OH_Drawing_Font) -> bool;

    /** @brief Gets text skew on x-axis for an <b>OH_Drawing_Font</b> object.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Font Indicates the pointer to an <b>OH_Drawing_Font</b> object.
    @return Returns additional skew on x-axis relative to y-axis.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_FontGetTextSkewX(arg1: *const OH_Drawing_Font) -> f32;
}

extern "C" {
    /** @brief Gets whether to increase the stroke width to approximate bold fonts.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Font Indicates the pointer to an <b>OH_Drawing_Font</b> object.
    @return Returns <b>true</b> to increase the stroke width to approximate bold fonts; returns <b>false</b> otherwise.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_FontIsFakeBoldText(arg1: *const OH_Drawing_Font) -> bool;
}

extern "C" {
    /** @brief Sets text scale on x-axis for an <b>OH_Drawing_Font</b> object.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Font Indicates the pointer to an <b>OH_Drawing_Font</b> object.
    @param scaleX Indicates the text horizontal scale.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_FontSetScaleX(arg1: *mut OH_Drawing_Font, scaleX: f32);

    /** @brief Gets text scale on x-axis from an <b>OH_Drawing_Font</b> object.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Font Indicates the pointer to an <b>OH_Drawing_Font</b> object.
    @return Returns text horizontal scale on x-axis.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_FontGetScaleX(arg1: *const OH_Drawing_Font) -> f32;

    /** @brief Sets hinting pattern for an <b>OH_Drawing_Font</b> object.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Font Indicates the pointer to an <b>OH_Drawing_Font</b> object.
    @param OH_Drawing_FontHinting Indicates the font hinting pattern.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_FontSetHinting(arg1: *mut OH_Drawing_Font, arg2: OH_Drawing_FontHinting);

    /** @brief Gets hinting pattern from an <b>OH_Drawing_Font</b> object.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Font Indicates the pointer to an <b>OH_Drawing_Font</b> object.
    @return Returns the font hinting pattern.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_FontGetHinting(arg1: *const OH_Drawing_Font) -> OH_Drawing_FontHinting;

    /** @brief Sets whether to use bitmaps instead of outlines in the <b>OH_Drawing_Font</b> object.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Font Indicates the pointer to an <b>OH_Drawing_Font</b> object.
    @param isEmbeddedBitmaps Indicates whether to use bitmaps instead of outlines.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_FontSetEmbeddedBitmaps(arg1: *mut OH_Drawing_Font, isEmbeddedBitmaps: bool);

    /** @brief Gets whether to use bitmaps instead of outlines in the <b>OH_Drawing_Font</b> object.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Font Indicates the pointer to an <b>OH_Drawing_Font</b> object.
    @return Returns <b>true</b> if using bitmaps instead of outlines; returns <b>false</b> otherwise.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_FontIsEmbeddedBitmaps(arg1: *const OH_Drawing_Font) -> bool;

    /** @brief Sets the font edging effect for an <b>OH_Drawing_Font</b> object.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Font Indicates the pointer to an <b>OH_Drawing_Font</b> object.
    @param OH_Drawing_FontEdging Indicates the font edging effect.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_FontSetEdging(arg1: *mut OH_Drawing_Font, arg2: OH_Drawing_FontEdging);

    /** @brief Gets the font edging effect from an <b>OH_Drawing_Font</b> object.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Font Indicates the pointer to an <b>OH_Drawing_Font</b> object.
    @return Returns the font edging effect.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_FontGetEdging(arg1: *const OH_Drawing_Font) -> OH_Drawing_FontEdging;
}

/** @brief Defines a run, supplies storage for the metrics of an <b>OH_Drawing_Font</b>.

@since 12
@version 1.0*/
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OH_Drawing_Font_Metrics {
    /// Indicating which metrics are valid
    pub flags: u32,
    /// storage for top in font metrics
    pub top: f32,
    /// storage for ascent in font metrics
    pub ascent: f32,
    /// storage for descent in font metrics
    pub descent: f32,
    /// storage for bottom in font metrics
    pub bottom: f32,
    /// storage for leading in font metrics
    pub leading: f32,
    ///  Average character width, zero if unknown
    pub avgCharWidth: f32,
    /// Maximum character width, zero if unknown
    pub maxCharWidth: f32,
    /// Greatest extent to left of origin of any glyph bounding box, typically negative; deprecated with variable fonts
    pub xMin: f32,
    /// Greatest extent to right of origin of any glyph bounding box, typically positive; deprecated with variable fonts
    pub xMax: f32,
    /// Height of lower-case letter, zero if unknown, typically negative
    pub xHeight: f32,
    /// Height of an upper-case letter, zero if unknown, typically negative
    pub capHeight: f32,
    /// @brief Underline thickness
    pub underlineThickness: f32,
    /// Distance from baseline to top of stroke, typically positive
    pub underlinePosition: f32,
    /// Strikeout thickness
    pub strikeoutThickness: f32,
    /// Distance from baseline to bottom of stroke, typically negative
    pub strikeoutPosition: f32,
}
extern "C" {
    /** @brief Obtains the metrics of a font.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Font Indicates the pointer to an <b>OH_Drawing_Font</b> object.
    @param OH_Drawing_Font_Metrics Indicates the pointer to an <b>OH_Drawing_Font_Metrics</b> object.
    @return Returns a float variable that recommended spacing between lines.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_FontGetMetrics(
        arg1: *mut OH_Drawing_Font,
        arg2: *mut OH_Drawing_Font_Metrics,
    ) -> f32;
}
