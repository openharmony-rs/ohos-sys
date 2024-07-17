#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::drawing::text_declaration::{
    OH_Drawing_PositionAndAffinity, OH_Drawing_Range, OH_Drawing_TextBox, OH_Drawing_TextStyle,
    OH_Drawing_Typography, OH_Drawing_TypographyCreate, OH_Drawing_TypographyStyle,
};
use crate::drawing::text_typography::OH_Drawing_TextBaseline;

impl OH_Drawing_PlaceholderVerticalAlignment {
    /// Offset At Baseline
    pub const ALIGNMENT_OFFSET_AT_BASELINE: OH_Drawing_PlaceholderVerticalAlignment =
        OH_Drawing_PlaceholderVerticalAlignment(0);
}
impl OH_Drawing_PlaceholderVerticalAlignment {
    /// Above Baseline
    pub const ALIGNMENT_ABOVE_BASELINE: OH_Drawing_PlaceholderVerticalAlignment =
        OH_Drawing_PlaceholderVerticalAlignment(1);
}
impl OH_Drawing_PlaceholderVerticalAlignment {
    /// Below Baseline
    pub const ALIGNMENT_BELOW_BASELINE: OH_Drawing_PlaceholderVerticalAlignment =
        OH_Drawing_PlaceholderVerticalAlignment(2);
}
impl OH_Drawing_PlaceholderVerticalAlignment {
    /// Top of Row Box
    pub const ALIGNMENT_TOP_OF_ROW_BOX: OH_Drawing_PlaceholderVerticalAlignment =
        OH_Drawing_PlaceholderVerticalAlignment(3);
}
impl OH_Drawing_PlaceholderVerticalAlignment {
    /// Bottom of Row Box
    pub const ALIGNMENT_BOTTOM_OF_ROW_BOX: OH_Drawing_PlaceholderVerticalAlignment =
        OH_Drawing_PlaceholderVerticalAlignment(4);
}
impl OH_Drawing_PlaceholderVerticalAlignment {
    /// Center of Row Box
    pub const ALIGNMENT_CENTER_OF_ROW_BOX: OH_Drawing_PlaceholderVerticalAlignment =
        OH_Drawing_PlaceholderVerticalAlignment(5);
}
#[repr(transparent)]
/** @brief Enumerates placeholder vertical alignment.

@since 11
@version 1.0*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct OH_Drawing_PlaceholderVerticalAlignment(pub ::core::ffi::c_uint);
/** @brief Defines the placeholder span.

@since 11
@version 1.0*/
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OH_Drawing_PlaceholderSpan {
    /// width of placeholder
    pub width: f64,
    /// height of placeholder
    pub height: f64,
    /// alignment of placeholder
    pub alignment: OH_Drawing_PlaceholderVerticalAlignment,
    /// baseline of placeholder
    pub baseline: OH_Drawing_TextBaseline,
    /// baselineoffset of placeholder
    pub baselineOffset: f64,
}
impl OH_Drawing_TextDecorationStyle {
    /// Solid style
    pub const TEXT_DECORATION_STYLE_SOLID: OH_Drawing_TextDecorationStyle =
        OH_Drawing_TextDecorationStyle(0);
}
impl OH_Drawing_TextDecorationStyle {
    /// Double style
    pub const TEXT_DECORATION_STYLE_DOUBLE: OH_Drawing_TextDecorationStyle =
        OH_Drawing_TextDecorationStyle(1);
}
impl OH_Drawing_TextDecorationStyle {
    /// Dotted style
    pub const TEXT_DECORATION_STYLE_DOTTED: OH_Drawing_TextDecorationStyle =
        OH_Drawing_TextDecorationStyle(2);
}
impl OH_Drawing_TextDecorationStyle {
    /// Dashed style
    pub const TEXT_DECORATION_STYLE_DASHED: OH_Drawing_TextDecorationStyle =
        OH_Drawing_TextDecorationStyle(3);
}
impl OH_Drawing_TextDecorationStyle {
    /// Wavy style
    pub const TEXT_DECORATION_STYLE_WAVY: OH_Drawing_TextDecorationStyle =
        OH_Drawing_TextDecorationStyle(4);
}
#[repr(transparent)]
/** @brief Enumerates text decoration style.

@since 11
@version 1.0*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct OH_Drawing_TextDecorationStyle(pub ::core::ffi::c_uint);
impl OH_Drawing_EllipsisModal {
    /// Head modal
    pub const ELLIPSIS_MODAL_HEAD: OH_Drawing_EllipsisModal = OH_Drawing_EllipsisModal(0);
}
impl OH_Drawing_EllipsisModal {
    /// Middle modal
    pub const ELLIPSIS_MODAL_MIDDLE: OH_Drawing_EllipsisModal = OH_Drawing_EllipsisModal(1);
}
impl OH_Drawing_EllipsisModal {
    /// Tail modal
    pub const ELLIPSIS_MODAL_TAIL: OH_Drawing_EllipsisModal = OH_Drawing_EllipsisModal(2);
}
#[repr(transparent)]
/** @brief Enumerates ellipsis modal.

@since 11
@version 1.0*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct OH_Drawing_EllipsisModal(pub ::core::ffi::c_uint);
impl OH_Drawing_BreakStrategy {
    /// Greedy strategy
    pub const BREAK_STRATEGY_GREEDY: OH_Drawing_BreakStrategy = OH_Drawing_BreakStrategy(0);
}
impl OH_Drawing_BreakStrategy {
    /// Quality strategy
    pub const BREAK_STRATEGY_HIGH_QUALITY: OH_Drawing_BreakStrategy = OH_Drawing_BreakStrategy(1);
}
impl OH_Drawing_BreakStrategy {
    /// Balanced strategy
    pub const BREAK_STRATEGY_BALANCED: OH_Drawing_BreakStrategy = OH_Drawing_BreakStrategy(2);
}
#[repr(transparent)]
/** @brief Enumerates break strategy.

@since 11
@version 1.0*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct OH_Drawing_BreakStrategy(pub ::core::ffi::c_uint);
impl OH_Drawing_WordBreakType {
    /// Normal type
    pub const WORD_BREAK_TYPE_NORMAL: OH_Drawing_WordBreakType = OH_Drawing_WordBreakType(0);
}
impl OH_Drawing_WordBreakType {
    /// Break All type
    pub const WORD_BREAK_TYPE_BREAK_ALL: OH_Drawing_WordBreakType = OH_Drawing_WordBreakType(1);
}
impl OH_Drawing_WordBreakType {
    /// Break Word type
    pub const WORD_BREAK_TYPE_BREAK_WORD: OH_Drawing_WordBreakType = OH_Drawing_WordBreakType(2);
}
#[repr(transparent)]
/** @brief Enumerates word break type.

@since 11
@version 1.0*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct OH_Drawing_WordBreakType(pub ::core::ffi::c_uint);
impl OH_Drawing_RectHeightStyle {
    /// Tight style
    pub const RECT_HEIGHT_STYLE_TIGHT: OH_Drawing_RectHeightStyle = OH_Drawing_RectHeightStyle(0);
}
impl OH_Drawing_RectHeightStyle {
    /// Max style
    pub const RECT_HEIGHT_STYLE_MAX: OH_Drawing_RectHeightStyle = OH_Drawing_RectHeightStyle(1);
}
impl OH_Drawing_RectHeightStyle {
    /// Includelinespacemiddle style
    pub const RECT_HEIGHT_STYLE_INCLUDELINESPACEMIDDLE: OH_Drawing_RectHeightStyle =
        OH_Drawing_RectHeightStyle(2);
}
impl OH_Drawing_RectHeightStyle {
    /// Includelinespacetop style
    pub const RECT_HEIGHT_STYLE_INCLUDELINESPACETOP: OH_Drawing_RectHeightStyle =
        OH_Drawing_RectHeightStyle(3);
}
impl OH_Drawing_RectHeightStyle {
    /// Includelinespacebottom style
    pub const RECT_HEIGHT_STYLE_INCLUDELINESPACEBOTTOM: OH_Drawing_RectHeightStyle =
        OH_Drawing_RectHeightStyle(4);
}
impl OH_Drawing_RectHeightStyle {
    /// Struct style
    pub const RECT_HEIGHT_STYLE_STRUCT: OH_Drawing_RectHeightStyle = OH_Drawing_RectHeightStyle(5);
}
#[repr(transparent)]
/** @brief Enumerates rect height style.

@since 11
@version 1.0*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct OH_Drawing_RectHeightStyle(pub ::core::ffi::c_uint);
impl OH_Drawing_RectWidthStyle {
    /// Tight style
    pub const RECT_WIDTH_STYLE_TIGHT: OH_Drawing_RectWidthStyle = OH_Drawing_RectWidthStyle(0);
}
impl OH_Drawing_RectWidthStyle {
    /// Max style
    pub const RECT_WIDTH_STYLE_MAX: OH_Drawing_RectWidthStyle = OH_Drawing_RectWidthStyle(1);
}
#[repr(transparent)]
/** @brief Enumerates rect Width style.

@since 11
@version 1.0*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct OH_Drawing_RectWidthStyle(pub ::core::ffi::c_uint);

extern "C" {
    /** @brief Sets the placeholder.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TypographyCreate Indicates the pointer to an <b>OH_Drawing_TypographyCreate</b> object.
    @param OH_Drawing_PlaceholderSpan Indicates the pointer to an <b>OH_Drawing_PlaceholderSpan</b> object.
    @since 11
    @version 1.0*/
    pub fn OH_Drawing_TypographyHandlerAddPlaceholder(
        arg1: *mut OH_Drawing_TypographyCreate,
        arg2: *mut OH_Drawing_PlaceholderSpan,
    );

    /** @brief Gets the exceed maxLines.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Typography Indicates the pointer to an <b>OH_Drawing_Typography</b> object.
    @return Returns the exceed maxLines.
    @since 11
    @version 1.0*/
    pub fn OH_Drawing_TypographyDidExceedMaxLines(arg1: *mut OH_Drawing_Typography) -> bool;

    /** @brief Gets the rects for range.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Typography Indicates the pointer to an <b>OH_Drawing_Typography</b> object.
    @param size_t Indicates the start of range to set.
    @param size_t Indicates the end of range to set.
    @param OH_Drawing_RectHeightStyle Indicates the height style to set.
    For details, see the enum <b>OH_Drawing_RectHeightStyle</b>.
    @param OH_Drawing_RectWidthStyle Indicates the width style to set.
    For details, see the enum <b>OH_Drawing_RectWidthStyle</b>.
    @return Returns the rects for range.
    @since 11
    @version 1.0*/
    pub fn OH_Drawing_TypographyGetRectsForRange(
        arg1: *mut OH_Drawing_Typography,
        arg2: usize,
        arg3: usize,
        arg4: OH_Drawing_RectHeightStyle,
        arg5: OH_Drawing_RectWidthStyle,
    ) -> *mut OH_Drawing_TextBox;

    /** @brief Gets the rects for placeholders.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Typography Indicates the pointer to an <b>OH_Drawing_Typography</b> object.
    @return Returns the rects for placeholders.
    @since 11
    @version 1.0*/
    pub fn OH_Drawing_TypographyGetRectsForPlaceholders(
        arg1: *mut OH_Drawing_Typography,
    ) -> *mut OH_Drawing_TextBox;

    /** @brief Gets left from textbox.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TextBox Indicates the pointer to an <b>OH_Drawing_TextBox</b> object.
    @param int Indicates the index of textbox.
    @return Returns left from textbox.
    @since 11
    @version 1.0*/
    pub fn OH_Drawing_GetLeftFromTextBox(
        arg1: *mut OH_Drawing_TextBox,
        arg2: ::core::ffi::c_int,
    ) -> f32;

    /** @brief Gets right from textbox.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TextBox Indicates the pointer to an <b>OH_Drawing_TextBox</b> object.
    @param int Indicates the index of textbox.
    @return Returns right from textbox.
    @since 11
    @version 1.0*/
    pub fn OH_Drawing_GetRightFromTextBox(
        arg1: *mut OH_Drawing_TextBox,
        arg2: ::core::ffi::c_int,
    ) -> f32;

    /** @brief Gets top from textbox.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TextBox Indicates the pointer to an <b>OH_Drawing_TextBox</b> object.
    @param int Indicates the index of textbox.
    @return Returns top from textbox.
    @since 11
    @version 1.0*/
    pub fn OH_Drawing_GetTopFromTextBox(
        arg1: *mut OH_Drawing_TextBox,
        arg2: ::core::ffi::c_int,
    ) -> f32;

    /** @brief Gets bottom from textbox.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TextBox Indicates the pointer to an <b>OH_Drawing_TextBox</b> object.
    @param int Indicates the index of textbox.
    @return Returns bottom from textbox.
    @since 11
    @version 1.0*/
    pub fn OH_Drawing_GetBottomFromTextBox(
        arg1: *mut OH_Drawing_TextBox,
        arg2: ::core::ffi::c_int,
    ) -> f32;

    /** @brief Gets direction from textbox.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TextBox Indicates the pointer to an <b>OH_Drawing_TextBox</b> object.
    @param int Indicates the index of textbox.
    @return Returns direction from textbox.
    @since 11
    @version 1.0*/
    pub fn OH_Drawing_GetTextDirectionFromTextBox(
        arg1: *mut OH_Drawing_TextBox,
        arg2: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;

    /** @brief Gets size of textbox.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TextBox Indicates the pointer to an <b>OH_Drawing_TextBox</b> object.
    @return Returns size of textbox.
    @since 11
    @version 1.0*/
    pub fn OH_Drawing_GetSizeOfTextBox(arg1: *mut OH_Drawing_TextBox) -> usize;

    /** @brief Gets the glyphposition at coordinate.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Typography Indicates the pointer to an <b>OH_Drawing_Typography</b> object.
    @param double Indicates the positionX of typography to set.
    @param double Indicates the positionY of typography to set.
    @return Returns the glyphposition at coordinate.
    @since 11
    @version 1.0*/
    pub fn OH_Drawing_TypographyGetGlyphPositionAtCoordinate(
        arg1: *mut OH_Drawing_Typography,
        arg2: f64,
        arg3: f64,
    ) -> *mut OH_Drawing_PositionAndAffinity;

    /** @brief Gets the glyphposition at coordinate with cluster.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Typography Indicates the pointer to an <b>OH_Drawing_Typography</b> object.
    @param double Indicates the positionX of typography to set.
    @param double Indicates the positionY of typography to set.
    @return Returns the glyphposition at coordinate with cluster.
    @since 11
    @version 1.0*/
    pub fn OH_Drawing_TypographyGetGlyphPositionAtCoordinateWithCluster(
        arg1: *mut OH_Drawing_Typography,
        arg2: f64,
        arg3: f64,
    ) -> *mut OH_Drawing_PositionAndAffinity;

    /** @brief Gets position from position and affinity.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_PositionAndAffinity Indicates the pointer to an <b>OH_Drawing_PositionAndAffinity</b> object.
    @return Returns position from position and affinity.
    @since 11
    @version 1.0*/
    pub fn OH_Drawing_GetPositionFromPositionAndAffinity(
        arg1: *mut OH_Drawing_PositionAndAffinity,
    ) -> usize;

    /** @brief Gets affinity from position and affinity.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_PositionAndAffinity Indicates the pointer to an <b>OH_Drawing_PositionAndAffinity</b> object.
    @return Returns affinity from position and affinity.
    @since 11
    @version 1.0*/
    pub fn OH_Drawing_GetAffinityFromPositionAndAffinity(
        arg1: *mut OH_Drawing_PositionAndAffinity,
    ) -> ::core::ffi::c_int;

    /** @brief Gets the word boundary.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Typography Indicates the pointer to an <b>OH_Drawing_Typography</b> object.
    @param size_t Indicates the size of text to set.
    @return Returns the word boundary.
    @since 11
    @version 1.0*/
    pub fn OH_Drawing_TypographyGetWordBoundary(
        arg1: *mut OH_Drawing_Typography,
        arg2: usize,
    ) -> *mut OH_Drawing_Range;

    /** @brief Gets start from range.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Range Indicates the pointer to an <b>OH_Drawing_Range</b> object.
    @return Returns start from range.
    @since 11
    @version 1.0*/
    pub fn OH_Drawing_GetStartFromRange(arg1: *mut OH_Drawing_Range) -> usize;

    /** @brief Gets end from range.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Range Indicates the pointer to an <b>OH_Drawing_Range</b> object.
    @return Returns end from range.
    @since 11
    @version 1.0*/
    pub fn OH_Drawing_GetEndFromRange(arg1: *mut OH_Drawing_Range) -> usize;

    /** @brief Gets the line count.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Typography Indicates the pointer to an <b>OH_Drawing_Typography</b> object.
    @return Returns the line count.
    @since 11
    @version 1.0*/
    pub fn OH_Drawing_TypographyGetLineCount(arg1: *mut OH_Drawing_Typography) -> usize;

    /** @brief Sets the decoration style.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TextStyle Indicates the pointer to an <b>OH_Drawing_TextStyle</b> object.
    @param int Indicates the text decoration style to set.
    For details, see the enum <b>OH_Drawing_TextDecorationStyle</b>.
    @since 11
    @version 1.0*/
    pub fn OH_Drawing_SetTextStyleDecorationStyle(
        arg1: *mut OH_Drawing_TextStyle,
        arg2: ::core::ffi::c_int,
    );

    /** @brief Sets the decoration thickness scale.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TextStyle Indicates the pointer to an <b>OH_Drawing_TextStyle</b> object.
    @param double Indicates the thickness scale of text decoration to set.
    @since 11
    @version 1.0*/
    pub fn OH_Drawing_SetTextStyleDecorationThicknessScale(
        arg1: *mut OH_Drawing_TextStyle,
        arg2: f64,
    );

    /** @brief Sets the letter spacing.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TextStyle Indicates the pointer to an <b>OH_Drawing_TextStyle</b> object.
    @param double Indicates the letter space to set.
    @since 11
    @version 1.0*/
    pub fn OH_Drawing_SetTextStyleLetterSpacing(arg1: *mut OH_Drawing_TextStyle, arg2: f64);

    /** @brief Sets the word spacing.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TextStyle Indicates the pointer to an <b>OH_Drawing_TextStyle</b> object.
    @param double Indicates the word space to set.
    @since 11
    @version 1.0*/
    pub fn OH_Drawing_SetTextStyleWordSpacing(arg1: *mut OH_Drawing_TextStyle, arg2: f64);

    /** @brief Sets the half leading.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TextStyle Indicates the pointer to an <b>OH_Drawing_TextStyle</b> object.
    @param bool Indicates the half leading to set.
    @since 11
    @version 1.0*/
    pub fn OH_Drawing_SetTextStyleHalfLeading(arg1: *mut OH_Drawing_TextStyle, arg2: bool);

    /** @brief Sets the ellipsis.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TextStyle Indicates the pointer to an <b>OH_Drawing_TextStyle</b> object.
    @param char* Indicates the pointer to ellipsis style.
    @since 11
    @version 1.0*/
    pub fn OH_Drawing_SetTextStyleEllipsis(
        arg1: *mut OH_Drawing_TextStyle,
        arg2: *const ::core::ffi::c_char,
    );

    /** @brief Sets the ellipsis modal.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TextStyle Indicates the pointer to an <b>OH_Drawing_TextStyle</b> object.
    @param int Indicates the ellipsis model to set. For details, see the enum <b>OH_Drawing_EllipsisModal</b>.
    @since 11
    @version 1.0*/
    pub fn OH_Drawing_SetTextStyleEllipsisModal(
        arg1: *mut OH_Drawing_TextStyle,
        arg2: ::core::ffi::c_int,
    );

    /** @brief Sets the break strategy.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TypographyStyle Indicates the pointer to an <b>OH_Drawing_TypographyStyle</b> object.
    @param int Indicates the break strategy to set. For details, see the enum <b>OH_Drawing_BreakStrategy</b>.
    @since 11
    @version 1.0*/
    pub fn OH_Drawing_SetTypographyTextBreakStrategy(
        arg1: *mut OH_Drawing_TypographyStyle,
        arg2: ::core::ffi::c_int,
    );

    /** @brief Sets the word break type.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TypographyStyle Indicates the pointer to an <b>OH_Drawing_TypographyStyle</b> object.
    @param int Indicates the word break type to set. For details, see the enum <b>OH_Drawing_WordBreakType</b>.
    @since 11
    @version 1.0*/
    pub fn OH_Drawing_SetTypographyTextWordBreakType(
        arg1: *mut OH_Drawing_TypographyStyle,
        arg2: ::core::ffi::c_int,
    );

    /** @brief Sets the ellipsis modal.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_TypographyStyle Indicates the pointer to an <b>OH_Drawing_TypographyStyle</b> object.
    @param int Indicates the ellipsis modal to set. For details, see the enum <b>OH_Drawing_EllipsisModal</b>.
    @since 11
    @version 1.0*/
    pub fn OH_Drawing_SetTypographyTextEllipsisModal(
        arg1: *mut OH_Drawing_TypographyStyle,
        arg2: ::core::ffi::c_int,
    );

    /** @brief get line height.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Typography Indicates the pointer to an <b>OH_Drawing_Typography</b> object.
    @param int Indicates the line number.
    @return Returns line height.
    @since 11
    @version 1.0*/
    pub fn OH_Drawing_TypographyGetLineHeight(
        arg1: *mut OH_Drawing_Typography,
        arg2: ::core::ffi::c_int,
    ) -> f64;

    /** @brief get line width.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Typography Indicates the pointer to an <b>OH_Drawing_Typography</b> object.
    @param int Indicates the line number.
    @return Returns line width.
    @since 11
    @version 1.0*/
    pub fn OH_Drawing_TypographyGetLineWidth(
        arg1: *mut OH_Drawing_Typography,
        arg2: ::core::ffi::c_int,
    ) -> f64;
}
