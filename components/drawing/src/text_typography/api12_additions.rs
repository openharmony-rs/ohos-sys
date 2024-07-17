#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::font::OH_Drawing_Font_Metrics;
use crate::text_declaration::{
    OH_Drawing_FontParser, OH_Drawing_Range, OH_Drawing_TextBox, OH_Drawing_TextShadow,
    OH_Drawing_TextStyle, OH_Drawing_Typography, OH_Drawing_TypographyCreate,
    OH_Drawing_TypographyStyle,
};
use crate::text_typography::{
    OH_Drawing_FontStyle, OH_Drawing_FontWeight, OH_Drawing_TextAlign, OH_Drawing_TextBaseline,
    OH_Drawing_TextDecorationStyle, OH_Drawing_TextDirection,
};
use crate::types::{OH_Drawing_Brush, OH_Drawing_Pen, OH_Drawing_Point, OH_Drawing_RectStyle_Info};

impl OH_Drawing_FontStyle {
    /// Oblique style
    pub const FONT_STYLE_OBLIQUE: OH_Drawing_FontStyle = OH_Drawing_FontStyle(2);
}
impl OH_Drawing_FontConfigInfoErrorCode {
    /// The list of system font configuration information was successfully obtained
    pub const SUCCESS_FONT_CONFIG_INFO: OH_Drawing_FontConfigInfoErrorCode =
        OH_Drawing_FontConfigInfoErrorCode(0);
}
impl OH_Drawing_FontConfigInfoErrorCode {
    /// Unknown error
    pub const ERROR_FONT_CONFIG_INFO_UNKNOWN: OH_Drawing_FontConfigInfoErrorCode =
        OH_Drawing_FontConfigInfoErrorCode(1);
}
impl OH_Drawing_FontConfigInfoErrorCode {
    /// Parse system config file error
    pub const ERROR_FONT_CONFIG_INFO_PARSE_FILE: OH_Drawing_FontConfigInfoErrorCode =
        OH_Drawing_FontConfigInfoErrorCode(2);
}
impl OH_Drawing_FontConfigInfoErrorCode {
    /// Alloc memory error
    pub const ERROR_FONT_CONFIG_INFO_ALLOC_MEMORY: OH_Drawing_FontConfigInfoErrorCode =
        OH_Drawing_FontConfigInfoErrorCode(3);
}
impl OH_Drawing_FontConfigInfoErrorCode {
    /// Copy string data error
    pub const ERROR_FONT_CONFIG_INFO_COPY_STRING_DATA: OH_Drawing_FontConfigInfoErrorCode =
        OH_Drawing_FontConfigInfoErrorCode(4);
}
#[repr(transparent)]
/** @brief Gets system font configuration information list result enum.

@since 12
@version 1.0*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct OH_Drawing_FontConfigInfoErrorCode(pub ::core::ffi::c_uint);
/** @brief Fallback font information.

@since 12
@version 1.0*/
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OH_Drawing_FontFallbackInfo {
    /// The type of language supported by the font set. The language format is bcp47
    pub language: *mut ::core::ffi::c_char,
    /// Font family name
    pub familyName: *mut ::core::ffi::c_char,
}
/** @brief Fallback font group.

@since 12
@version 1.0*/
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OH_Drawing_FontFallbackGroup {
    /** The name of the font set corresponding to the fallback font set. If the value is null,
    all fonts can be set using the fallback font set list.*/
    pub groupName: *mut ::core::ffi::c_char,
    /// Fallback font Info Size
    pub fallbackInfoSize: usize,
    /// A list of font sets for fallback fonts
    pub fallbackInfoSet: *mut OH_Drawing_FontFallbackInfo,
}
/** @brief Font weight mapping information.

@since 12
@version 1.0*/
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OH_Drawing_FontAdjustInfo {
    /// The font's original weight value
    pub weight: ::core::ffi::c_int,
    /// The font weight displayed in the application
    pub to: ::core::ffi::c_int,
}
/** @brief Alias font information.

@since 12
@version 1.0*/
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OH_Drawing_FontAliasInfo {
    /// Font family name
    pub familyName: *mut ::core::ffi::c_char,
    /** Font weight value. When the weight value is greater than 0,
    the font set contains only fonts with the specified weight.
    When the weight value is equal to 0, the font set contains all fonts.*/
    pub weight: ::core::ffi::c_int,
}
/** @brief General font set information supported by the system.

@since 12
@version 1.0*/
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OH_Drawing_FontGenericInfo {
    /// Font family name
    pub familyName: *mut ::core::ffi::c_char,
    /// The size of alias font lists
    pub aliasInfoSize: usize,
    /// The size of font weight mapping information lists
    pub adjustInfoSize: usize,
    /// List of alias fonts
    pub aliasInfoSet: *mut OH_Drawing_FontAliasInfo,
    /// Font weight mapping information lists
    pub adjustInfoSet: *mut OH_Drawing_FontAdjustInfo,
}
/** @brief System font configuration information.

@since 12
@version 1.0*/
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OH_Drawing_FontConfigInfo {
    /// Count of system font file paths
    pub fontDirSize: usize,
    /// List size of generic font sets
    pub fontGenericInfoSize: usize,
    /// Count of fallback font set lists
    pub fallbackGroupSize: usize,
    /// List of system font file paths
    pub fontDirSet: *mut *mut ::core::ffi::c_char,
    /// List of generic font sets
    pub fontGenericInfoSet: *mut OH_Drawing_FontGenericInfo,
    /// List of fallback font sets
    pub fallbackGroupSet: *mut OH_Drawing_FontFallbackGroup,
}
/** @brief Describes the font information.

@since 12
@version 1.0*/
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OH_Drawing_FontDescriptor {
    /// The file path of System font
    pub path: *mut ::core::ffi::c_char,
    /// A name that uniquely identifies the font
    pub postScriptName: *mut ::core::ffi::c_char,
    /// The name of System font
    pub fullName: *mut ::core::ffi::c_char,
    /// The family of System font
    pub fontFamily: *mut ::core::ffi::c_char,
    /// The subfont family of the system font
    pub fontSubfamily: *mut ::core::ffi::c_char,
    /// The weight of System font
    pub weight: ::core::ffi::c_int,
    /// The width of System font
    pub width: ::core::ffi::c_int,
    /// Whether the system font is tilted
    pub italic: ::core::ffi::c_int,
    /// Whether the system font is compact
    pub monoSpace: bool,
    /// whether symbolic fonts are supported
    pub symbolic: bool,
}
/** @brief The metrics of line.

@since 12
@version 1.0*/
#[repr(C)]
pub struct OH_Drawing_LineMetrics {
    /// Text ascender height
    pub ascender: f64,
    /// Tex descender height
    pub descender: f64,
    /// The height of a capital letter
    pub capHeight: f64,
    /// The height of a lowercase letter
    pub xHeight: f64,
    /// Text width
    pub width: f64,
    /// Line height
    pub height: f64,
    /** The distance from the left end of the text to the left end of the container,
    aligned to 0, is the width of the container minus the width of the line of text*/
    pub x: f64,
    /** The height from the top of the text to the top of the container, the first line is 0,
    and the second line is the height of the first line*/
    pub y: f64,
    /// Start Index
    pub startIndex: usize,
    /// End Index
    pub endIndex: usize,
    /// The metrics information of the first character
    pub firstCharMetrics: OH_Drawing_Font_Metrics,
}
impl OH_Drawing_TextHeightBehavior {
    /// both ascend of first row and last row style
    pub const TEXT_HEIGHT_ALL: OH_Drawing_TextHeightBehavior = OH_Drawing_TextHeightBehavior(0);
}
impl OH_Drawing_TextHeightBehavior {
    /// forbidding ascend of first row style
    pub const TEXT_HEIGHT_DISABLE_FIRST_ASCENT: OH_Drawing_TextHeightBehavior =
        OH_Drawing_TextHeightBehavior(1);
}
impl OH_Drawing_TextHeightBehavior {
    /// forbidding ascend of last row style
    pub const TEXT_HEIGHT_DISABLE_LAST_ASCENT: OH_Drawing_TextHeightBehavior =
        OH_Drawing_TextHeightBehavior(2);
}
impl OH_Drawing_TextHeightBehavior {
    /// neither ascend of first row nor last row style
    pub const TEXT_HEIGHT_DISABLE_ALL: OH_Drawing_TextHeightBehavior =
        OH_Drawing_TextHeightBehavior(3);
}
#[repr(transparent)]
/** @brief Enumerates of heightmode of text.

@since 12
@version 1.0*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct OH_Drawing_TextHeightBehavior(pub ::core::ffi::c_uint);
impl OH_Drawing_TextStyleType {
    /// None style
    pub const TEXT_STYLE_NONE: OH_Drawing_TextStyleType = OH_Drawing_TextStyleType(0);
}
impl OH_Drawing_TextStyleType {
    /// All attributes style
    pub const TEXT_STYLE_ALL_ATTRIBUTES: OH_Drawing_TextStyleType = OH_Drawing_TextStyleType(1);
}
impl OH_Drawing_TextStyleType {
    /// Font style
    pub const TEXT_STYLE_FONT: OH_Drawing_TextStyleType = OH_Drawing_TextStyleType(2);
}
impl OH_Drawing_TextStyleType {
    /// Foreground style
    pub const TEXT_STYLE_FOREGROUND: OH_Drawing_TextStyleType = OH_Drawing_TextStyleType(3);
}
impl OH_Drawing_TextStyleType {
    /// Background style
    pub const TEXT_STYLE_BACKGROUND: OH_Drawing_TextStyleType = OH_Drawing_TextStyleType(4);
}
impl OH_Drawing_TextStyleType {
    /// Shadow style
    pub const TEXT_STYLE_SHADOW: OH_Drawing_TextStyleType = OH_Drawing_TextStyleType(5);
}
impl OH_Drawing_TextStyleType {
    /// Decorations style
    pub const TEXT_STYLE_DECORATIONS: OH_Drawing_TextStyleType = OH_Drawing_TextStyleType(6);
}
impl OH_Drawing_TextStyleType {
    /// Letter spacing style
    pub const TEXT_STYLE_LETTER_SPACING: OH_Drawing_TextStyleType = OH_Drawing_TextStyleType(7);
}
impl OH_Drawing_TextStyleType {
    /// Word spacing style
    pub const TEXT_STYLE_WORD_SPACING: OH_Drawing_TextStyleType = OH_Drawing_TextStyleType(8);
}
#[repr(transparent)]
/** @brief Enumerates text style type.

@since 12
@version 1.0*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct OH_Drawing_TextStyleType(pub ::core::ffi::c_uint);
impl OH_Drawing_FontWidth {
    pub const FONT_WIDTH_ULTRA_CONDENSED: OH_Drawing_FontWidth = OH_Drawing_FontWidth(1);
}
impl OH_Drawing_FontWidth {
    pub const FONT_WIDTH_EXTRA_CONDENSED: OH_Drawing_FontWidth = OH_Drawing_FontWidth(2);
}
impl OH_Drawing_FontWidth {
    pub const FONT_WIDTH_CONDENSED: OH_Drawing_FontWidth = OH_Drawing_FontWidth(3);
}
impl OH_Drawing_FontWidth {
    pub const FONT_WIDTH_SEMI_CONDENSED: OH_Drawing_FontWidth = OH_Drawing_FontWidth(4);
}
impl OH_Drawing_FontWidth {
    pub const FONT_WIDTH_NORMAL: OH_Drawing_FontWidth = OH_Drawing_FontWidth(5);
}
impl OH_Drawing_FontWidth {
    pub const FONT_WIDTH_SEMI_EXPANDED: OH_Drawing_FontWidth = OH_Drawing_FontWidth(6);
}
impl OH_Drawing_FontWidth {
    pub const FONT_WIDTH_EXPANDED: OH_Drawing_FontWidth = OH_Drawing_FontWidth(7);
}
impl OH_Drawing_FontWidth {
    pub const FONT_WIDTH_EXTRA_EXPANDED: OH_Drawing_FontWidth = OH_Drawing_FontWidth(8);
}
impl OH_Drawing_FontWidth {
    pub const FONT_WIDTH_ULTRA_EXPANDED: OH_Drawing_FontWidth = OH_Drawing_FontWidth(9);
}
#[repr(transparent)]
/** @brief Enumerates font width.

@since 12
@version 1.0*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct OH_Drawing_FontWidth(pub ::core::ffi::c_uint);
/** @brief Defines the font style struct.

@since 12
@version 1.0*/
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OH_Drawing_FontStyleStruct {
    /// Font weight
    pub weight: OH_Drawing_FontWeight,
    /// Font width
    pub width: OH_Drawing_FontWidth,
    /// Font slant
    pub slant: OH_Drawing_FontStyle,
}
/** @brief Defines the fontfeature.

@since 12
@version 1.0*/
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OH_Drawing_FontFeature {
    /// key of fontfeature
    pub tag: *mut ::core::ffi::c_char,
    /// value of fontfeature
    pub value: ::core::ffi::c_int,
}
/** @brief Defines StrutStyle info struct.

@since 12
@version 1.0*/
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OH_Drawing_StrutStyle {
    /// The font weight to use when calculating the strut
    pub weight: OH_Drawing_FontWeight,
    /// The font style to use when calculating the strut
    pub style: OH_Drawing_FontStyle,
    /// The size of the ascent plus descent in logical pixels
    pub size: f64,
    /// The minimum height of the strut, as a multiple of fontSize
    pub heightScale: f64,
    /// Whether the height is override
    pub heightOverride: bool,
    /// Whether the halfleading is enable
    pub halfLeading: bool,
    /// The additional leading to apply to the strut as a multiple of Size
    pub leading: f64,
    /// Whether the strut height should be forced
    pub forceStrutHeight: bool,
    /// The size of font families
    pub familiesSize: usize,
    /// The families of the font to use when calculating the strut
    pub families: *mut *mut ::core::ffi::c_char,
}

extern "C" {
    /** @brief Sets the foreground brush style.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TextStyle Indicates the pointer to a text style object <b>OH_Drawing_TextStyle</b>.
    @param OH_Drawing_Brush Indicates the pointer to a brush object <b>OH_Drawing_Brush</b>.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_SetTextStyleForegroundBrush(
        arg1: *mut OH_Drawing_TextStyle,
        arg2: *mut OH_Drawing_Brush,
    );
    /** @brief Gets the foreground brush style.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TextStyle Indicates the pointer to a text style object <b>OH_Drawing_TextStyle</b>.
    @param OH_Drawing_Brush Indicates the pointer to a brush object <b>OH_Drawing_Brush</b>.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_TextStyleGetForegroundBrush(
        arg1: *mut OH_Drawing_TextStyle,
        arg2: *mut OH_Drawing_Brush,
    );
    /** @brief Sets the foreground pen style.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TextStyle Indicates the pointer to a text style object <b>OH_Drawing_TextStyle</b>.
    @param OH_Drawing_Pen Indicates the pointer to a pen object <b>OH_Drawing_Pen</b>.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_SetTextStyleForegroundPen(
        arg1: *mut OH_Drawing_TextStyle,
        arg2: *mut OH_Drawing_Pen,
    );
    /** @brief Gets the foreground pen style.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TextStyle Indicates the pointer to a text style object <b>OH_Drawing_TextStyle</b>.
    @param OH_Drawing_Pen Indicates the pointer to a pen object <b>OH_Drawing_Pen</b>.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_TextStyleGetForegroundPen(
        arg1: *mut OH_Drawing_TextStyle,
        arg2: *mut OH_Drawing_Pen,
    );
    /** @brief Sets the background brush style.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TextStyle Indicates the pointer to a text style object <b>OH_Drawing_TextStyle</b>.
    @param OH_Drawing_Brush Indicates the pointer to a brush object <b>OH_Drawing_Brush</b>.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_SetTextStyleBackgroundBrush(
        arg1: *mut OH_Drawing_TextStyle,
        arg2: *mut OH_Drawing_Brush,
    );
    /** @brief Gets the background brush style.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TextStyle Indicates the pointer to a text style object <b>OH_Drawing_TextStyle</b>.
    @param OH_Drawing_Brush Indicates the pointer to a brush object <b>OH_Drawing_Brush</b>.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_TextStyleGetBackgroundBrush(
        arg1: *mut OH_Drawing_TextStyle,
        arg2: *mut OH_Drawing_Brush,
    );
    /** @brief Sets the background pen style.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TextStyle Indicates the pointer to a text style object <b>OH_Drawing_TextStyle</b>.
    @param OH_Drawing_Pen Indicates the pointer to a pen object <b>OH_Drawing_Pen</b>.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_SetTextStyleBackgroundPen(
        arg1: *mut OH_Drawing_TextStyle,
        arg2: *mut OH_Drawing_Pen,
    );
    /** @brief Gets the background pen style.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TextStyle Indicates the pointer to a text style object <b>OH_Drawing_TextStyle</b>.
    @param OH_Drawing_Pen Indicates the pointer to a pen object <b>OH_Drawing_Pen</b>.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_TextStyleGetBackgroundPen(
        arg1: *mut OH_Drawing_TextStyle,
        arg2: *mut OH_Drawing_Pen,
    );
    // Note: Minor documentation changes to OH_Drawing_TypographyGetLongestLine

    /** @brief get line text range.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Typography Indicates the pointer to an <b>OH_Drawing_Typography</b> object.
    @param int Indicates the line number.
    @param bool Indicates whether spaces are contained.
    @return Returns line text range.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_TypographyGetLineTextRange(
        arg1: *mut OH_Drawing_Typography,
        arg2: ::core::ffi::c_int,
        arg3: bool,
    ) -> *mut OH_Drawing_Range;
    /** @brief Creates an <b>OH_Drawing_FontDescriptor</b> object.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @return Returns the pointer to the font descriptor object <b>OH_Drawing_FontDescriptor</b> created.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_CreateFontDescriptor() -> *mut OH_Drawing_FontDescriptor;
    /** @brief Releases the memory occupied by an <b>OH_Drawing_FontDescriptor</b> object.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_FontDescriptor the pointer to the font descriptor object <b>OH_Drawing_FontDescriptor</b>.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_DestroyFontDescriptor(arg1: *mut OH_Drawing_FontDescriptor);
    /** @brief Creates an <b>OH_Drawing_FontParser</b> object.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @return Returns the pointer to the font parser object <b>OH_Drawing_FontParser</b>.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_CreateFontParser() -> *mut OH_Drawing_FontParser;
    /** @brief Releases the memory occupied by an <b>OH_Drawing_FontParser</b> object.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_FontParser Indicates the pointer to the font parser object <b>OH_Drawing_FontParser</b>.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_DestroyFontParser(arg1: *mut OH_Drawing_FontParser);
    /** @brief Gets a list of system font names.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_FontParser Indicates the pointer to the font parser object <b>OH_Drawing_FontParser</b>.
    @param size_t Returns the number of obtained system font names.
    @return Returns a list of obtained system fonts.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_FontParserGetSystemFontList(
        arg1: *mut OH_Drawing_FontParser,
        arg2: *mut usize,
    ) -> *mut *mut ::core::ffi::c_char;
    /** @brief Releases the memory occupied by a list of system font names.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param char** Indicates the pointer to a list of system font names.
    @param size_t The number of obtained system font names.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_DestroySystemFontList(arg1: *mut *mut ::core::ffi::c_char, arg2: usize);
    /** @brief Gets information about the system font by font name.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_FontParser Indicates the pointer to the font parser object <b>OH_Drawing_FontParser</b>.
    @param char** font name.
    @return Returns system fonts information.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_FontParserGetFontByName(
        arg1: *mut OH_Drawing_FontParser,
        arg2: *const ::core::ffi::c_char,
    ) -> *mut OH_Drawing_FontDescriptor;
    /** @brief Get line metrics information.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Typography Indicates the pointer to a typography object <b>OH_Drawing_Typography</b>.
    @return Indicates the pointer to a line metrics object <b>OH_Drawing_LineMetrics</b>.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_TypographyGetLineMetrics(
        arg1: *mut OH_Drawing_Typography,
    ) -> *mut OH_Drawing_LineMetrics;
    /** @brief Get the number of lines.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_LineMetrics Indicates the pointer to a line metrics object <b>OH_Drawing_LineMetrics</b>.
    @return Returns the number of lines.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_LineMetricsGetSize(arg1: *mut OH_Drawing_LineMetrics) -> usize;
    /** @brief Releases the memory occupied by line metrics.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_LineMetrics Indicates the pointer to a line metrics object <b>OH_Drawing_LineMetrics</b>.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_DestroyLineMetrics(arg1: *mut OH_Drawing_LineMetrics);
    /** @brief Gets the specified line by line number.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Typography Indicates the pointer to a typography object <b>OH_Drawing_Typography</b>.
    @param int Line number.
    @param OH_Drawing_LineMetrics Indicates the pointer to a line metrics object <b>OH_Drawing_LineMetrics</b>.
    @return Whether the line metrics was obtained.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_TypographyGetLineMetricsAt(
        arg1: *mut OH_Drawing_Typography,
        arg2: ::core::ffi::c_int,
        arg3: *mut OH_Drawing_LineMetrics,
    ) -> bool;
    /** @brief  Sets the ellipsis of lines in a text file.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Typography Indicates the pointer to a typography object <b>OH_Drawing_Typography</b>.
    @param char Indicates the line textellipsis.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_SetTypographyTextEllipsis(
        arg1: *mut OH_Drawing_TypographyStyle,
        arg2: *const ::core::ffi::c_char,
    );
    /** @brief Sets the locale of lines in a text file.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TypographyStyle Indicates the pointer to a typography style object
    <b>OH_Drawing_TypographyStyle</b>.
    @param char Indicates the pointer to the locale to set.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_SetTypographyTextLocale(
        arg1: *mut OH_Drawing_TypographyStyle,
        arg2: *const ::core::ffi::c_char,
    );
    /** @brief Sets the textSplitRatio of lines in a text file.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TypographyStyle Indicates the pointer to a typography style object
    <b>OH_Drawing_TypographyStyle</b>.
    @param float Indicates the textSplitRatio of lines to set.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_SetTypographyTextSplitRatio(arg1: *mut OH_Drawing_TypographyStyle, arg2: f32);
    /** @brief Gets the TextStyle of lines in a text file.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TypographyStyle Indicates the pointer to a typography style object
    <b>OH_Drawing_TypographyStyle</b>.
    @return Returns line text textstyle.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_TypographyGetTextStyle(
        arg1: *mut OH_Drawing_TypographyStyle,
    ) -> *mut OH_Drawing_TextStyle;
    /** @brief Gets the EffectiveAlign of lines in a text file.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TypographyStyle Indicates the pointer to a typography style object
    <b>OH_Drawing_TypographyStyle</b>.
    @return Returns line text align.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_TypographyGetEffectiveAlignment(
        arg1: *mut OH_Drawing_TypographyStyle,
    ) -> ::core::ffi::c_int;
    /** @brief Gets the UnlimitedLines of lines in a text file.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TypographyStyle Indicates the pointer to a typography style object
    <b>OH_Drawing_TypographyStyle</b>.
    @return Returns whether the text has a maximum line limit,
    with true indicating a maximum line limit and false indicating no maximum line limit.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_TypographyIsLineUnlimited(arg1: *mut OH_Drawing_TypographyStyle) -> bool;
    /** @brief Gets the IsEllipsized of lines in a text file.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TypographyStyle Indicates the pointer to a typography style object
    <b>OH_Drawing_TypographyStyle</b>.
    @return Returns whether the text has ellipsis,
    true meaning there is an ellipsis and false meaning there is no ellipsis.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_TypographyIsEllipsized(arg1: *mut OH_Drawing_TypographyStyle) -> bool;
    /** @brief set line textstyle.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TypographyStyle Indicates the pointer to a typography style object
    <b>OH_Drawing_TypographyStyle</b>.
    @param OH_Drawing_TextStyle Indicates the pointer to a text style object <b>OH_Drawing_TextStyle</b>.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_SetTypographyTextStyle(
        arg1: *mut OH_Drawing_TypographyStyle,
        arg2: *mut OH_Drawing_TextStyle,
    );
    /** @brief get line fontmetrics.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Typography Indicates the pointer to a typography object <b>OH_Drawing_Typography</b>.
    @param OH_Drawing_TextStyle Indicates the pointer to a text style object <b>OH_Drawing_TextStyle</b>.
    @param OH_Drawing_Font_Metrics Indicates the pointer to a font metrics object <b>OH_Drawing_Font_Metrics</b>.
    @return Whether the font metrics was obtained.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_TextStyleGetFontMetrics(
        arg1: *mut OH_Drawing_Typography,
        arg2: *mut OH_Drawing_TextStyle,
        arg3: *mut OH_Drawing_Font_Metrics,
    ) -> bool;
    /** @brief Gets the position of the specified line or the first text of the specified line.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Typography Indicates the pointer to a typography object <b>OH_Drawing_Typography</b>.
    @param int Line number.
    @param bool True is the information for the whole line, and false is the information to get the first character
    @param bool Whether the text width contains whitespace.
    @param OH_Drawing_LineMetrics Indicates the pointer to a line metrics object <b>OH_Drawing_LineMetrics</b>.
    @return return whether the information was successfully fetched.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_TypographyGetLineInfo(
        arg1: *mut OH_Drawing_Typography,
        arg2: ::core::ffi::c_int,
        arg3: bool,
        arg4: bool,
        arg5: *mut OH_Drawing_LineMetrics,
    ) -> bool;
    /** @brief Sets the font weight of text typography.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TypographyStyle Indicates the pointer to a typography style object
    <b>OH_Drawing_TypographyStyle</b>.
    @param int Indicates the font weight of text typography to set. For details,
    see the enum <b>OH_Drawing_FontWeight</b>.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_SetTypographyTextFontWeight(
        arg1: *mut OH_Drawing_TypographyStyle,
        arg2: ::core::ffi::c_int,
    );
    /** @brief Sets the font style of text typography.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TypographyStyle Indicates the pointer to a typography style object
    <b>OH_Drawing_TypographyStyle</b>.
    @param int Indicates the font style of text typography to set. For details,
    see the enum <b>OH_Drawing_FontStyle</b>.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_SetTypographyTextFontStyle(
        arg1: *mut OH_Drawing_TypographyStyle,
        arg2: ::core::ffi::c_int,
    );
    /** @brief Sets the font family of text typography.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TypographyStyle Indicates the pointer to a typography style object
    <b>OH_Drawing_TypographyStyle</b>.
    @param char Indicates the pointer to the font family of text typography to set.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_SetTypographyTextFontFamily(
        arg1: *mut OH_Drawing_TypographyStyle,
        arg2: *const ::core::ffi::c_char,
    );
    /** @brief Sets the font size of text typography.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TypographyStyle Indicates the pointer to a typography style object
    <b>OH_Drawing_TypographyStyle</b>.
    @param double Indicates the font size of text typography to set.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_SetTypographyTextFontSize(arg1: *mut OH_Drawing_TypographyStyle, arg2: f64);
    /** @brief Sets the font height of text typography.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TypographyStyle Indicates the pointer to a typography style object
    <b>OH_Drawing_TypographyStyle</b>.
    @param double Indicates the font height of text typography to set.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_SetTypographyTextFontHeight(arg1: *mut OH_Drawing_TypographyStyle, arg2: f64);
    /** @brief Sets the half leading of text typography.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TypographyStyle Indicates the pointer to a typography style object
    <b>OH_Drawing_TypographyStyle</b>.
    @param bool Indicates the half leading of text typography to set.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_SetTypographyTextHalfLeading(
        arg1: *mut OH_Drawing_TypographyStyle,
        arg2: bool,
    );
    /** @brief Sets whether to enable line style for text typography.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TypographyStyle Indicates the pointer to a typography style object
    <b>OH_Drawing_TypographyStyle</b>.
    @param bool Indicates whether the line style for text typography is used.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_SetTypographyTextUseLineStyle(
        arg1: *mut OH_Drawing_TypographyStyle,
        arg2: bool,
    );
    /** @brief Sets the font weight of line style for text typography.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TypographyStyle Indicates the pointer to a typography style object
    <b>OH_Drawing_TypographyStyle</b>.
    @param int Indicates the font weight of line style for text typography to set.
    For details, see the enum <b>OH_Drawing_FontWeight</b>.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_SetTypographyTextLineStyleFontWeight(
        arg1: *mut OH_Drawing_TypographyStyle,
        arg2: ::core::ffi::c_int,
    );
    /** @brief Sets the font style of line style for text typography.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TypographyStyle Indicates the pointer to a typography style object
    <b>OH_Drawing_TypographyStyle</b>.
    @param int Indicates the font style of line style for text typography to set. For details,
    see the enum <b>OH_Drawing_FontStyle</b>.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_SetTypographyTextLineStyleFontStyle(
        arg1: *mut OH_Drawing_TypographyStyle,
        arg2: ::core::ffi::c_int,
    );
    /** @brief Sets the font families of line style for text typography.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TypographyStyle Indicates the pointer to a typography style object
    <b>OH_Drawing_TypographyStyle</b>.
    @param int Indicates the number of font families to set.
    @param char Indicates the pointer to the font families of line style for text typography to set.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_SetTypographyTextLineStyleFontFamilies(
        arg1: *mut OH_Drawing_TypographyStyle,
        arg2: ::core::ffi::c_int,
        fontFamilies: *mut *const ::core::ffi::c_char,
    );
    /** @brief Sets the font size of line style for text typography.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TypographyStyle Indicates the pointer to a typography style object
    <b>OH_Drawing_TypographyStyle</b>.
    @param double Indicates the font size of line style for text typography to set.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_SetTypographyTextLineStyleFontSize(
        arg1: *mut OH_Drawing_TypographyStyle,
        arg2: f64,
    );
    /** @brief Sets the font height of line style for text typography.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TypographyStyle Indicates the pointer to a typography style object
    <b>OH_Drawing_TypographyStyle</b>.
    @param double Indicates the font height of line style for text typography to set.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_SetTypographyTextLineStyleFontHeight(
        arg1: *mut OH_Drawing_TypographyStyle,
        arg2: f64,
    );
    /** @brief Sets the half leading of line style for text typography.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TypographyStyle Indicates the pointer to a typography style object
    <b>OH_Drawing_TypographyStyle</b>.
    @param bool Indicates the half leading of line for text typography to set.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_SetTypographyTextLineStyleHalfLeading(
        arg1: *mut OH_Drawing_TypographyStyle,
        arg2: bool,
    );
    /** @brief Sets the spacing scale of line style for text typography.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TypographyStyle Indicates the pointer to a typography style object
    <b>OH_Drawing_TypographyStyle</b>.
    @param double Indicates the space scale of line for text typography to set.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_SetTypographyTextLineStyleSpacingScale(
        arg1: *mut OH_Drawing_TypographyStyle,
        arg2: f64,
    );
    /** @brief Sets whether only line style is enabled for text typography.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TypographyStyle Indicates the pointer to a typography style object
    <b>OH_Drawing_TypographyStyle</b>.
    @param bool Indicates the line style for text typography to set only.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_SetTypographyTextLineStyleOnly(
        arg1: *mut OH_Drawing_TypographyStyle,
        arg2: bool,
    );
    /** @brief Creates an <b>OH_Drawing_TextShadow</b> object.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @return Returns the pointer to the text shadow object created <b>OH_Drawing_TextShadow</b>.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_CreateTextShadow() -> *mut OH_Drawing_TextShadow;
    /** @brief Releases the memory occupied by the text shadow object <b>OH_Drawing_TextShadow</b>.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TextShadow Indicates the pointer to the text shadow object <b>OH_Drawing_TextShadow</b>.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_DestroyTextShadow(arg1: *mut OH_Drawing_TextShadow);
    /** @brief Gets the vector of TextShadow in TextStyle.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TextStyle Indicates the pointer to a text style object <b>OH_Drawing_TextStyle</b>.
    @param int Indicates the number in vector to set.
    @param OH_Drawing_TextShadow Indicates the pointer to the text shadow object <b>OH_Drawing_TextShadow</b>.
    @return Returns the vector of TextShadow.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_TextStyleGetShadows(
        arg1: *mut OH_Drawing_TextStyle,
    ) -> *mut OH_Drawing_TextShadow;
    /** @brief Gets the size of vector of TextShadow in TextStyle.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TextStyle Indicates the pointer to a text style object <b>OH_Drawing_TextStyle</b>.
    @return Returns the size of vector.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_TextStyleGetShadowCount(
        arg1: *mut OH_Drawing_TextStyle,
    ) -> ::core::ffi::c_int;
    /** @brief Adds element in vector of TextShadow in TextStyle.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TextStyle Indicates the pointer to a text style object <b>OH_Drawing_TextStyle</b>.
    @param OH_Drawing_TextShadow Indicates the pointer to the text shadow object <b>OH_Drawing_TextShadow</b>.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_TextStyleAddShadow(
        arg1: *mut OH_Drawing_TextStyle,
        arg2: *const OH_Drawing_TextShadow,
    );
    /** @brief clear elements in vector of TextShadow in TextStyle.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TextStyle Indicates the pointer to a text style object <b>OH_Drawing_TextStyle</b>.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_TextStyleClearShadows(arg1: *mut OH_Drawing_TextStyle);
    /** @brief Gets element in vector of TextShadow with index.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TextStyle Indicates the pointer to a text style object <b>OH_Drawing_TextStyle</b>.
    @param int Indicates the index to set.
    @return Returns the pointer to element with the index in vector of the text style object
    <b>OH_Drawing_TextStyle</b>.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_TextStyleGetShadowWithIndex(
        arg1: *mut OH_Drawing_TextStyle,
        arg2: ::core::ffi::c_int,
    ) -> *mut OH_Drawing_TextShadow;
    /** @brief Set indents of the typography.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Typography Indicates the pointer to a typography object <b>OH_Drawing_Typography</b>.
    @param float Indicates the pointer to the indents to set.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_TypographySetIndents(
        arg1: *mut OH_Drawing_Typography,
        arg2: ::core::ffi::c_int,
        indents: *const f32,
    );
    /** @brief Gets element with index in vector of Indents.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Typography Indicates the pointer to a typography object <b>OH_Drawing_Typography</b>.
    @param int Indicates the index to set.
    @return float Indicates the element with the index in vector of Indents.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_TypographyGetIndentsWithIndex(
        arg1: *mut OH_Drawing_Typography,
        arg2: ::core::ffi::c_int,
    ) -> f32;
    /** @brief Releases the memory occupied by vector with the text shadow object <b>OH_Drawing_TextShadow</b>.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param Indicates the pointer to the text shadow object <b>OH_Drawing_TextShadow</b>.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_DestroyTextShadows(arg1: *mut OH_Drawing_TextShadow);
    /** @brief Set mode of applying the leading over and under text.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TypographyStyle Indicates the pointer to an <b>OH_Drawing_TypographyStyle</b> object.
    @param heightMode Indicates the mode to set.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_TypographyTextSetHeightBehavior(
        arg1: *mut OH_Drawing_TypographyStyle,
        heightMode: OH_Drawing_TextHeightBehavior,
    );
    /** @brief Get mode of applying the leading over and under text.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TypographyStyle Indicates the pointer to an <b>OH_Drawing_TypographyStyle</b> object.
    @return Returns the mode.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_TypographyTextGetHeightBehavior(
        arg1: *mut OH_Drawing_TypographyStyle,
    ) -> OH_Drawing_TextHeightBehavior;
    /** @brief Set struct of background rect and styleId of text.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TextStyle Indicates the pointer to an <b>OH_Drawing_TextStyle</b> object.
    @param OH_Drawing_RectStyle_Info Indicates the pointer to an <b>OH_Drawing_RectStyle_Info</b> object.
    @param styleId Indicates the styleId of text to set.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_TextStyleSetBackgroundRect(
        arg1: *mut OH_Drawing_TextStyle,
        arg2: *const OH_Drawing_RectStyle_Info,
        styleId: ::core::ffi::c_int,
    );
    /** @brief Add symbols in creating typography.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TypographyCreate Indicates the pointer to an <b>OH_Drawing_TypographyCreate</b> object.
    @param symbol Indicates the symbol to set.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_TypographyHandlerAddSymbol(
        arg1: *mut OH_Drawing_TypographyCreate,
        symbol: u32,
    );
    /** @brief Add font feature.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TextStyle Indicates the pointer to an <b>OH_Drawing_TextStyle</b> object.
    @param tag Indicates the pointer to the tag to set.
    @param value Indicates the value to set.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_TextStyleAddFontFeature(
        arg1: *mut OH_Drawing_TextStyle,
        tag: *const ::core::ffi::c_char,
        value: ::core::ffi::c_int,
    );
    /** @brief Get all font features.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TextStyle Indicates the pointer to an <b>OH_Drawing_TextStyle</b> object.
    @return OH_Drawing_FontFeature Indicates the pointer to an array of structures of OH_Drawing_FontFeature.
    Get size of font feature by OH_Drawing_TextStyleGetFontFeatureSize.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_TextStyleGetFontFeatures(
        arg1: *mut OH_Drawing_TextStyle,
    ) -> *mut OH_Drawing_FontFeature;
    /** @brief Release the memory occupied by array of structures of font features.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_FontFeature Indicates the pointer to an array of structures of OH_Drawing_FontFeature.
    @param fontFeatureSize Indicates the size of array of structures of OH_Drawing_FontFeature.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_TextStyleDestroyFontFeatures(
        arg1: *mut OH_Drawing_FontFeature,
        fontFeatureSize: usize,
    );
    /** @brief Get size of font features.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TextStyle Indicates the pointer to an <b>OH_Drawing_TextStyle</b> object.
    @return Returns the size of fontfeatures map.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_TextStyleGetFontFeatureSize(arg1: *mut OH_Drawing_TextStyle) -> usize;
    /** @brief Clear font features.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TextStyle Indicates the pointer to an <b>OH_Drawing_TextStyle</b> object.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_TextStyleClearFontFeature(arg1: *mut OH_Drawing_TextStyle);
    /** @brief Set baseline shift of text.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TextStyle Indicates the pointer to an <b>OH_Drawing_TextStyle</b> object.
    @param lineShift Indicates the baseline shift to set.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_TextStyleSetBaselineShift(arg1: *mut OH_Drawing_TextStyle, lineShift: f64);
    /** @brief Get baseline shift of text.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TextStyle Indicates the pointer to an <b>OH_Drawing_TextStyle</b> object.
    @return Returns the baseline shift.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_TextStyleGetBaselineShift(arg1: *mut OH_Drawing_TextStyle) -> f64;
    /** @brief Gets the text color.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TextStyle Indicates the pointer to an <b>OH_Drawing_TextStyle</b> object.
    @return Returns the text color.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_TextStyleGetColor(arg1: *mut OH_Drawing_TextStyle) -> u32;
    /** @brief Gets text decoration style.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TextStyle Indicates the pointer to an <b>OH_Drawing_TextStyle</b> object.
    @return Returns text decoration style.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_TextStyleGetDecorationStyle(
        arg1: *mut OH_Drawing_TextStyle,
    ) -> OH_Drawing_TextDecorationStyle;
    /** @brief Gets font weight.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TextStyle Indicates the pointer to an <b>OH_Drawing_TextStyle</b> object.
    @return Returns font Weight.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_TextStyleGetFontWeight(
        arg1: *mut OH_Drawing_TextStyle,
    ) -> OH_Drawing_FontWeight;
    /** @brief Gets font style.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TextStyle Indicates the pointer to an <b>OH_Drawing_TextStyle</b> object.
    @return Returns font style.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_TextStyleGetFontStyle(
        arg1: *mut OH_Drawing_TextStyle,
    ) -> OH_Drawing_FontStyle;
    /** @brief Gets the font baseline.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TextStyle Indicates the pointer to an <b>OH_Drawing_TextStyle</b> object.
    @return Returns the font baseline.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_TextStyleGetBaseline(
        arg1: *mut OH_Drawing_TextStyle,
    ) -> OH_Drawing_TextBaseline;
    /** @brief Gets a list of font families.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TextStyle Indicates the pointer to an <b>OH_Drawing_TextStyle</b> object.
    @param num Indicates count of font families result.
    @return Returns a list of font families.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_TextStyleGetFontFamilies(
        arg1: *mut OH_Drawing_TextStyle,
        num: *mut usize,
    ) -> *mut *mut ::core::ffi::c_char;
    /** @brief Releases the memory occupied by a list of font families.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param fontFamilies Indicates the pointer to a list of font families.
    @param num Indicates the count of obtained font families.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_TextStyleDestroyFontFamilies(
        fontFamilies: *mut *mut ::core::ffi::c_char,
        num: usize,
    );
    /** @brief Gets font size.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TextStyle Indicates the pointer to an <b>OH_Drawing_TextStyle</b> object.
    @return Returns font size.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_TextStyleGetFontSize(arg1: *mut OH_Drawing_TextStyle) -> f64;
    /** @brief Gets the letter spacing of the text.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TextStyle Indicates the pointer to an <b>OH_Drawing_TextStyle</b> object.
    @return Returns the size of the letter spacing.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_TextStyleGetLetterSpacing(arg1: *mut OH_Drawing_TextStyle) -> f64;
    /** @brief Gets the word spacing of the text.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TextStyle Indicates the pointer to an <b>OH_Drawing_TextStyle</b> object.
    @return Returns word spacing size.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_TextStyleGetWordSpacing(arg1: *mut OH_Drawing_TextStyle) -> f64;
    /** @brief Gets font height.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TextStyle Indicates the pointer to an <b>OH_Drawing_TextStyle</b> object.
    @return Returns font height.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_TextStyleGetFontHeight(arg1: *mut OH_Drawing_TextStyle) -> f64;
    /** @brief Gets whether to set the text to half line spacing.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TextStyle Indicates the pointer to an <b>OH_Drawing_TextStyle</b> object.
    @return Returns true indicates that the spacing takes effect,
    false indicates that the spacing does not take effect.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_TextStyleGetHalfLeading(arg1: *mut OH_Drawing_TextStyle) -> bool;
    /** @brief Gets the locale.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TextStyle Indicates the pointer to an <b>OH_Drawing_TextStyle</b> object.
    @return Returns a locale of data type as a pointer to a char. As with the TextStyle lifecycle.
    No release is required and the return value is invalidated after the set method is called.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_TextStyleGetLocale(
        arg1: *mut OH_Drawing_TextStyle,
    ) -> *const ::core::ffi::c_char;
    /** @brief Sets the text style, including font weight, font width and font slant.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TextStyle Indicates the pointer to an <b>OH_Drawing_TextStyle</b> object.
    @param OH_Drawing_FontStyleStruct Indicates an <b>OH_Drawing_FontStyleStruct</b> object.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_SetTextStyleFontStyleStruct(
        drawingTextStyle: *mut OH_Drawing_TextStyle,
        fontStyle: OH_Drawing_FontStyleStruct,
    );
    /** @brief Gets the text style, including font weight, font width and font slant.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TextStyle Indicates the pointer to an <b>OH_Drawing_TextStyle</b> object.
    @return Returns the <b>OH_Drawing_FontStyleStruct</b> object getted.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_TextStyleGetFontStyleStruct(
        drawingTextStyle: *mut OH_Drawing_TextStyle,
    ) -> OH_Drawing_FontStyleStruct;
    /** @brief Sets the typography style, including font weight, font width and font slant.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TypographyStyle Indicates the pointer to an <b>OH_Drawing_TypographyStyle</b> object.
    @param OH_Drawing_FontStyleStruct Indicates an <b>OH_Drawing_FontStyleStruct</b> object.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_SetTypographyStyleFontStyleStruct(
        drawingStyle: *mut OH_Drawing_TypographyStyle,
        fontStyle: OH_Drawing_FontStyleStruct,
    );
    /** @brief Gets the typography style, including font weight, font width and font slant.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TypographyStyle Indicates the pointer to an <b>OH_Drawing_TypographyStyle</b> object.
    @return Returns the <b>OH_Drawing_FontStyleStruct</b> object getted.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_TypographyStyleGetFontStyleStruct(
        drawingStyle: *mut OH_Drawing_TypographyStyle,
    ) -> OH_Drawing_FontStyleStruct;
    /** @brief Gets whether the two TextStyle objects are equal.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param style Indicates source of comparison <b>OH_Drawing_TextStyle</b> object.
    @param comparedStyle Indicates comparison <b>OH_Drawing_TextStyle</b> object.
    @return Compare result.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_TextStyleIsEqual(
        style: *const OH_Drawing_TextStyle,
        comparedStyle: *const OH_Drawing_TextStyle,
    ) -> bool;
    /** @brief Gets whether the font properties of two TextStyle objects are equal.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param style Indicates source of <b>comparison OH_Drawing_TextStyle</b> object.
    @param comparedStyle Indicates comparison <b>OH_Drawing_TextStyle</b> object.
    @return Compare result.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_TextStyleIsEqualByFont(
        style: *const OH_Drawing_TextStyle,
        comparedStyle: *const OH_Drawing_TextStyle,
    ) -> bool;
    /** @brief Gets whether two TextStyle objects match attributes

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param textStyleType Indicates enumerates of text style type.
    @param style Indicates source of comparison <b>OH_Drawing_TextStyle</b> object.
    @param comparedStyle Indicates comparison <b>OH_Drawing_TextStyle</b> object.
    @return Match attributes result.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_TextStyleIsAttributeMatched(
        style: *const OH_Drawing_TextStyle,
        comparedStyle: *const OH_Drawing_TextStyle,
        textStyleType: OH_Drawing_TextStyleType,
    ) -> bool;
    /** @brief Set placeholder of TextStyle.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TextStyle Indicates the pointer to an <b>OH_Drawing_TextStyle</b> object.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_TextStyleSetPlaceholder(style: *mut OH_Drawing_TextStyle);
    /** @brief Gets whether placeholder is enable.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TextStyle Indicates the pointer to an <b>OH_Drawing_TextStyle</b> object.
    @return Whether placeholder is enable.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_TextStyleIsPlaceholder(style: *mut OH_Drawing_TextStyle) -> bool;
    /** @brief Gets text alignment mode.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TypographyStyle Indicates the pointer to an <b>OH_Drawing_TypographyStyle</b> object.
    @return Returns text alignment mode.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_TypographyStyleGetEffectiveAlignment(
        style: *mut OH_Drawing_TypographyStyle,
    ) -> OH_Drawing_TextAlign;
    /** @brief Gets whether the hinting is enabled.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TypographyStyle Indicates the pointer to an <b>OH_Drawing_TypographyStyle</b> object.
    @return True, if the hinting takes effect; False, if the hinting does not take effect.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_TypographyStyleIsHintEnabled(style: *mut OH_Drawing_TypographyStyle) -> bool;
    /** @brief Gets system font configuration information.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_FontConfigInfoErrorCode Indicates error code returned, based on the error code to
    release the memory of system font configuration information.
    For details, see the enum <b>OH_Drawing_FontConfigInfoErrorCode</b>.
    @return Returns a pointer to system font configuration information.
    Indicates the pointer to an <b>OH_Drawing_FontConfigInfo</b> object.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_GetSystemFontConfigInfo(
        arg1: *mut OH_Drawing_FontConfigInfoErrorCode,
    ) -> *mut OH_Drawing_FontConfigInfo;
    /** @brief Releases the memory occupied by system font configuration information.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_FontConfigInfo Indicates the pointer to an <b>OH_Drawing_FontConfigInfo</b> object.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_DestroySystemFontConfigInfo(arg1: *mut OH_Drawing_FontConfigInfo);
    /** @brief Sets the strut style for text typography.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TypographyStyle Indicates the pointer to an <b>OH_Drawing_TypographyStyle</b> object.
    @param OH_Drawing_StrutStyle Indicates the pointer of <b>OH_Drawing_StrutStyle</b> object.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_SetTypographyStyleTextStrutStyle(
        arg1: *mut OH_Drawing_TypographyStyle,
        arg2: *mut OH_Drawing_StrutStyle,
    );
    /** @brief Releases the memory occupied by an <b>OH_Drawing_StrutStyle</b> object.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_StrutStyle Indicates the pointer of <b>OH_Drawing_StrutStyle</b> object.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_TypographyStyleDestroyStrutStyle(arg1: *mut OH_Drawing_StrutStyle);
    /** @brief Gets the strut style for text typography.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TypographyStyle Indicates the pointer to an <b>OH_Drawing_TypographyStyle</b> object.
    @return Returns the pointer of <b>OH_Drawing_StrutStyle</b> object.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_TypographyStyleGetStrutStyle(
        arg1: *mut OH_Drawing_TypographyStyle,
    ) -> *mut OH_Drawing_StrutStyle;
    /** @brief Overriding the struct StrutStyle equals operator.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param from Indicates source of comparison object.
    @param to Indicates comparison object.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_TypographyStyleStrutStyleEquals(
        from: *mut OH_Drawing_StrutStyle,
        to: *mut OH_Drawing_StrutStyle,
    ) -> bool;
    /** @brief Sets the hinting of text typography.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TypographyStyle Indicates the pointer to an <b>OH_Drawing_TypographyStyle</b> object.
    @param hintsEnabled Indicates the hinting of text typography..
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_TypographyStyleSetHintsEnabled(
        style: *mut OH_Drawing_TypographyStyle,
        hintsEnabled: bool,
    );
    pub fn OH_Drawing_TypographyGetLineFontMetrics(
        arg1: *mut OH_Drawing_Typography,
        lineNumber: usize,
        fontMetricsSize: *mut usize,
    ) -> *mut OH_Drawing_Font_Metrics;
    /** @brief Free up all the space taken up by the lineFontMetric.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Font_Metrics Indicates the first address of the lineFontMetric gather to be destroyed.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_TypographyDestroyLineFontMetrics(arg1: *mut OH_Drawing_Font_Metrics);
    /** @brief Mark the Typography as dirty, and initially state the Typography.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Typography Indicates the pointer to the text <b>OH_Drawing_Typography</b> object.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_TypographyMarkDirty(arg1: *mut OH_Drawing_Typography);
    /** @brief Get the unresolved Glyphs count of lines in a text.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Typography Indicates the pointer to the text <b>OH_Drawing_Typography</b> object.
    @return Returns unresolved Glyphs count.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_TypographyGetUnresolvedGlyphsCount(arg1: *mut OH_Drawing_Typography) -> i32;
    /** @brief Update the font size of lines in a text.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Typography Indicates the pointer to the text <b>OH_Drawing_Typography</b> object.
    @param from Indicates the source of the original font size.
    @param to Indicates the destination of the updated font size.
    @param fontSize Indicates the size of the font.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_TypographyUpdateFontSize(
        arg1: *mut OH_Drawing_Typography,
        from: usize,
        to: usize,
        fontSize: f32,
    );
    /** @brief Get whether the text layout enables line styles.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TypographyStyle Indicates the pointer to the text <b>OH_Drawing_TypographyStyle</b> object.
    @return Whether or not to enable line styles in text layout only, true means enable, false means disable.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_TypographyTextGetLineStyle(arg1: *mut OH_Drawing_TypographyStyle) -> bool;
    /** @brief Get the font weight of line style for text typography.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TypographyStyle Indicates the pointer to a typography style object
    <b>OH_Drawing_TypographyStyle</b>.
    @return Return the font weight of line style for text typography.
    For details, see the enum <b>OH_Drawing_FontWeight</b>.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_TypographyTextlineStyleGetFontWeight(
        arg1: *mut OH_Drawing_TypographyStyle,
    ) -> OH_Drawing_FontWeight;
    /** @brief Get the font style of line style for text typography.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TypographyStyle Indicates the pointer to a typography style object
    <b>OH_Drawing_TypographyStyle</b>.
    @return Return the font style of line style for text typography.
    For details, see the enum <b>OH_Drawing_FontStyle</b>.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_TypographyTextlineStyleGetFontStyle(
        arg1: *mut OH_Drawing_TypographyStyle,
    ) -> OH_Drawing_FontStyle;
    /** @brief Get the font families of line style for text typography.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TypographyStyle Indicates the pointer to a typography style object
    <b>OH_Drawing_TypographyStyle</b>.
    @param num The number of obtained font names.
    @return Return the font families of line style for text typography.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_TypographyTextlineStyleGetFontFamilies(
        arg1: *mut OH_Drawing_TypographyStyle,
        num: *mut usize,
    ) -> *mut *mut ::core::ffi::c_char;
    /** @brief Releases the memory occupied by a list of font families names.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param fontFamilies Indicates the pointer to a list of font families names.
    @param fontFamiliesNum Indicates the number of obtained font names.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_TypographyTextlineStyleDestroyFontFamilies(
        fontFamilies: *mut *mut ::core::ffi::c_char,
        fontFamiliesNum: usize,
    );
    /** @brief Get the font size of font size for text typography.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TypographyStyle Indicates the pointer to a typography style object
    <b>OH_Drawing_TypographyStyle</b>.
    @return Return the font size of font size for text typography.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_TypographyTextlineStyleGetFontSize(
        arg1: *mut OH_Drawing_TypographyStyle,
    ) -> f64;
    /** @brief Get the font height scale in text layout.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TypographyStyle Indicates the pointer to a typography style object
    <b>OH_Drawing_TypographyStyle</b>.
    @return Retrun the font height scale in text layout.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_TypographyTextlineStyleGetHeightScale(
        arg1: *mut OH_Drawing_TypographyStyle,
    ) -> f64;
    /** @brief Get whether to enable font height for line styles in text layout only.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TypographyStyle Indicates the pointer to a typography style object
    <b>OH_Drawing_TypographyStyle</b>.
    @return Whether or not to enable the font height for line styles in text layout only,
    true means enable, false means disable.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_TypographyTextlineStyleGetHeightOnly(
        arg1: *mut OH_Drawing_TypographyStyle,
    ) -> bool;
    /** @brief Get the half leading of line style for text typography.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TypographyStyle Indicates the pointer to a typography style object
    <b>OH_Drawing_TypographyStyle</b>.
    @return Whether to enable the text line half leading style, true means enable, false means disable.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_TypographyTextlineStyleGetHalfLeading(
        arg1: *mut OH_Drawing_TypographyStyle,
    ) -> bool;
    /** @brief Get the spacing scale of line style for text typography.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TypographyStyle Indicates the pointer to a typography style object
    <b>OH_Drawing_TypographyStyle</b>.
    @return Return the spacing scale of line style for text typography.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_TypographyTextlineStyleGetSpacingScale(
        arg1: *mut OH_Drawing_TypographyStyle,
    ) -> f64;
    /** @brief Get whether only line style is enabled for text typography.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TypographyStyle Indicates the pointer to a typography style object
    <b>OH_Drawing_TypographyStyle</b>.
    @return Returns whether only line style is enabled for text layout, true means it is enabled,
    false means it is not.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_TypographyTextlineGetStyleOnly(arg1: *mut OH_Drawing_TypographyStyle)
        -> bool;
    /** @brief Get the text alignment mode.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TypographyStyle Indicates the pointer to a typography style object
    <b>OH_Drawing_TypographyStyle</b>.
    @return Return the text alignment mode. For details, see the enum <b>OH_Drawing_TextAlign</b>.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_TypographyGetTextAlign(
        arg1: *mut OH_Drawing_TypographyStyle,
    ) -> OH_Drawing_TextAlign;
    /** @brief Get the text direction.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TypographyStyle Indicates the pointer to a typography style object
    <b>OH_Drawing_TypographyStyle</b>.
    @return Return the text direction. For details, see the enum <b>OH_Drawing_TextDirection</b>.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_TypographyGetTextDirection(
        arg1: *mut OH_Drawing_TypographyStyle,
    ) -> OH_Drawing_TextDirection;
    /** @brief Sets the maximum number of lines in a text.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TypographyStyle Indicates the pointer to a typography style object
    <b>OH_Drawing_TypographyStyle</b>.
    @return Return the maximum number of lines in a text.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_TypographyGetTextMaxLines(arg1: *mut OH_Drawing_TypographyStyle) -> usize;
    /** @brief Get the ellipsis of lines in a text.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TypographyStyle Indicates the pointer to a typography style object
    <b>OH_Drawing_TypographyStyle</b>.
    @return Return the ellipsis of lines in a text.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_TypographyGetTextEllipsis(
        arg1: *mut OH_Drawing_TypographyStyle,
    ) -> *mut ::core::ffi::c_char;
    /** @brief Releases the memory occupied by a list of Ellipsis names.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param ellipsis Indicates the pointer to a list of Ellipsis names.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_TypographyDestroyEllipsis(ellipsis: *mut ::core::ffi::c_char);
    /** @brief Overriding the class ParagraphStyle equals operator.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param from Indicates source of comparison object.
    @param to Indicates comparison object.
    @return Compare result.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_TypographyStyleEquals(
        from: *mut OH_Drawing_TypographyStyle,
        to: *mut OH_Drawing_TypographyStyle,
    ) -> bool;
    /** @brief Releases the memory occupied by text box.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TextBox Indicates the pointer to a text box object <b>OH_Drawing_TextBox</b>.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_TypographyDestroyTextBox(arg1: *mut OH_Drawing_TextBox);
    /** @brief Sets the parameter of text-shadow.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TextShadow Indicates the pointer to an <b>OH_Drawing_TextShadow</b> object.
    @param color Indicates the color setting of text-shadow.
    @param OH_Drawing_Point Indicates the pointer to an <b>OH_Drawing_Point</b> object.
    @param blurRadius Indicates the radius of blur for text-shadow.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_SetTextShadow(
        shadow: *mut OH_Drawing_TextShadow,
        color: u32,
        offset: *mut OH_Drawing_Point,
        blurRadius: f64,
    );
}
