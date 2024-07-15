//! Contains the Additions of API-11 compared to API-10.
//!
//! This file is manually kept in sync.
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::drawing::{
    OH_Drawing_Bitmap, OH_Drawing_Brush, OH_Drawing_Canvas, OH_Drawing_FontCollection,
    OH_Drawing_Path, OH_Drawing_Pen, OH_Drawing_TextBaseline, OH_Drawing_TextStyle,
    OH_Drawing_Typography, OH_Drawing_TypographyCreate, OH_Drawing_TypographyStyle,
};

#[repr(C)]
pub struct OH_Drawing_Point {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct OH_Drawing_Rect {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct OH_Drawing_RoundRect {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct OH_Drawing_Matrix {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct OH_Drawing_ShaderEffect {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct OH_Drawing_Filter {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct OH_Drawing_MaskFilter {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct OH_Drawing_ColorFilter {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct OH_Drawing_Font {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct OH_Drawing_Typeface {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct OH_Drawing_TextBlob {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct OH_Drawing_TextBlobBuilder {
    _unused: [u8; 0],
}

impl OH_Drawing_BlendMode {
    pub const BLEND_MODE_CLEAR: OH_Drawing_BlendMode = OH_Drawing_BlendMode(0);
}
impl OH_Drawing_BlendMode {
    pub const BLEND_MODE_SRC: OH_Drawing_BlendMode = OH_Drawing_BlendMode(1);
}
impl OH_Drawing_BlendMode {
    pub const BLEND_MODE_DST: OH_Drawing_BlendMode = OH_Drawing_BlendMode(2);
}
impl OH_Drawing_BlendMode {
    pub const BLEND_MODE_SRC_OVER: OH_Drawing_BlendMode = OH_Drawing_BlendMode(3);
}
impl OH_Drawing_BlendMode {
    pub const BLEND_MODE_DST_OVER: OH_Drawing_BlendMode = OH_Drawing_BlendMode(4);
}
impl OH_Drawing_BlendMode {
    pub const BLEND_MODE_SRC_IN: OH_Drawing_BlendMode = OH_Drawing_BlendMode(5);
}
impl OH_Drawing_BlendMode {
    pub const BLEND_MODE_DST_IN: OH_Drawing_BlendMode = OH_Drawing_BlendMode(6);
}
impl OH_Drawing_BlendMode {
    pub const BLEND_MODE_SRC_OUT: OH_Drawing_BlendMode = OH_Drawing_BlendMode(7);
}
impl OH_Drawing_BlendMode {
    pub const BLEND_MODE_DST_OUT: OH_Drawing_BlendMode = OH_Drawing_BlendMode(8);
}
impl OH_Drawing_BlendMode {
    pub const BLEND_MODE_SRC_ATOP: OH_Drawing_BlendMode = OH_Drawing_BlendMode(9);
}
impl OH_Drawing_BlendMode {
    pub const BLEND_MODE_DST_ATOP: OH_Drawing_BlendMode = OH_Drawing_BlendMode(10);
}
impl OH_Drawing_BlendMode {
    pub const BLEND_MODE_XOR: OH_Drawing_BlendMode = OH_Drawing_BlendMode(11);
}
impl OH_Drawing_BlendMode {
    pub const BLEND_MODE_PLUS: OH_Drawing_BlendMode = OH_Drawing_BlendMode(12);
}
impl OH_Drawing_BlendMode {
    pub const BLEND_MODE_MODULATE: OH_Drawing_BlendMode = OH_Drawing_BlendMode(13);
}
impl OH_Drawing_BlendMode {
    pub const BLEND_MODE_SCREEN: OH_Drawing_BlendMode = OH_Drawing_BlendMode(14);
}
impl OH_Drawing_BlendMode {
    pub const BLEND_MODE_OVERLAY: OH_Drawing_BlendMode = OH_Drawing_BlendMode(15);
}
impl OH_Drawing_BlendMode {
    pub const BLEND_MODE_DARKEN: OH_Drawing_BlendMode = OH_Drawing_BlendMode(16);
}
impl OH_Drawing_BlendMode {
    pub const BLEND_MODE_LIGHTEN: OH_Drawing_BlendMode = OH_Drawing_BlendMode(17);
}
impl OH_Drawing_BlendMode {
    pub const BLEND_MODE_COLOR_DODGE: OH_Drawing_BlendMode = OH_Drawing_BlendMode(18);
}
impl OH_Drawing_BlendMode {
    pub const BLEND_MODE_COLOR_BURN: OH_Drawing_BlendMode = OH_Drawing_BlendMode(19);
}
impl OH_Drawing_BlendMode {
    pub const BLEND_MODE_HARD_LIGHT: OH_Drawing_BlendMode = OH_Drawing_BlendMode(20);
}
impl OH_Drawing_BlendMode {
    pub const BLEND_MODE_SOFT_LIGHT: OH_Drawing_BlendMode = OH_Drawing_BlendMode(21);
}
impl OH_Drawing_BlendMode {
    pub const BLEND_MODE_DIFFERENCE: OH_Drawing_BlendMode = OH_Drawing_BlendMode(22);
}
impl OH_Drawing_BlendMode {
    pub const BLEND_MODE_EXCLUSION: OH_Drawing_BlendMode = OH_Drawing_BlendMode(23);
}
impl OH_Drawing_BlendMode {
    pub const BLEND_MODE_MULTIPLY: OH_Drawing_BlendMode = OH_Drawing_BlendMode(24);
}
impl OH_Drawing_BlendMode {
    pub const BLEND_MODE_HUE: OH_Drawing_BlendMode = OH_Drawing_BlendMode(25);
}
impl OH_Drawing_BlendMode {
    pub const BLEND_MODE_SATURATION: OH_Drawing_BlendMode = OH_Drawing_BlendMode(26);
}
impl OH_Drawing_BlendMode {
    pub const BLEND_MODE_COLOR: OH_Drawing_BlendMode = OH_Drawing_BlendMode(27);
}
impl OH_Drawing_BlendMode {
    pub const BLEND_MODE_LUMINOSITY: OH_Drawing_BlendMode = OH_Drawing_BlendMode(28);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct OH_Drawing_BlendMode(pub ::core::ffi::c_uint);

extern "C" {
    pub fn OH_Drawing_BrushGetAlpha(arg1: *const OH_Drawing_Brush) -> u8;
}
extern "C" {
    pub fn OH_Drawing_BrushSetAlpha(arg1: *mut OH_Drawing_Brush, alpha: u8);
}
extern "C" {
    pub fn OH_Drawing_BrushSetShaderEffect(
        arg1: *mut OH_Drawing_Brush,
        arg2: *mut OH_Drawing_ShaderEffect,
    );
}
extern "C" {
    pub fn OH_Drawing_BrushSetFilter(arg1: *mut OH_Drawing_Brush, arg2: *mut OH_Drawing_Filter);
}

extern "C" {
    pub fn OH_Drawing_CanvasGetSaveCount(arg1: *mut OH_Drawing_Canvas) -> u32;
}
extern "C" {
    pub fn OH_Drawing_CanvasRestoreToCount(arg1: *mut OH_Drawing_Canvas, saveCount: u32);
}

extern "C" {
    pub fn OH_Drawing_CanvasDrawBitmap(
        arg1: *mut OH_Drawing_Canvas,
        arg2: *const OH_Drawing_Bitmap,
        left: f32,
        top: f32,
    );
}
extern "C" {
    pub fn OH_Drawing_CanvasDrawRect(arg1: *mut OH_Drawing_Canvas, arg2: *const OH_Drawing_Rect);
}
extern "C" {
    pub fn OH_Drawing_CanvasDrawCircle(
        arg1: *mut OH_Drawing_Canvas,
        arg2: *const OH_Drawing_Point,
        radius: f32,
    );
}
extern "C" {
    pub fn OH_Drawing_CanvasDrawOval(arg1: *mut OH_Drawing_Canvas, arg2: *const OH_Drawing_Rect);
}
extern "C" {
    pub fn OH_Drawing_CanvasDrawArc(
        arg1: *mut OH_Drawing_Canvas,
        arg2: *const OH_Drawing_Rect,
        startAngle: f32,
        sweepAngle: f32,
    );
}
extern "C" {
    pub fn OH_Drawing_CanvasDrawRoundRect(
        arg1: *mut OH_Drawing_Canvas,
        arg2: *const OH_Drawing_RoundRect,
    );
}
extern "C" {
    pub fn OH_Drawing_CanvasDrawTextBlob(
        arg1: *mut OH_Drawing_Canvas,
        arg2: *const OH_Drawing_TextBlob,
        x: f32,
        y: f32,
    );
}
impl OH_Drawing_CanvasClipOp {
    pub const DIFFERENCE: OH_Drawing_CanvasClipOp = OH_Drawing_CanvasClipOp(0);
}
impl OH_Drawing_CanvasClipOp {
    pub const INTERSECT: OH_Drawing_CanvasClipOp = OH_Drawing_CanvasClipOp(1);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct OH_Drawing_CanvasClipOp(pub ::core::ffi::c_uint);
extern "C" {
    pub fn OH_Drawing_CanvasClipRect(
        arg1: *mut OH_Drawing_Canvas,
        arg2: *const OH_Drawing_Rect,
        clipOp: OH_Drawing_CanvasClipOp,
        doAntiAlias: bool,
    );
}
extern "C" {
    pub fn OH_Drawing_CanvasClipPath(
        arg1: *mut OH_Drawing_Canvas,
        arg2: *const OH_Drawing_Path,
        clipOp: OH_Drawing_CanvasClipOp,
        doAntiAlias: bool,
    );
}
extern "C" {
    pub fn OH_Drawing_CanvasRotate(arg1: *mut OH_Drawing_Canvas, degrees: f32, px: f32, py: f32);
}
extern "C" {
    pub fn OH_Drawing_CanvasTranslate(arg1: *mut OH_Drawing_Canvas, dx: f32, dy: f32);
}
extern "C" {
    pub fn OH_Drawing_CanvasScale(arg1: *mut OH_Drawing_Canvas, sx: f32, sy: f32);
}
#[repr(C)]
pub struct OH_Drawing_TextBox {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct OH_Drawing_PositionAndAffinity {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct OH_Drawing_Range {
    _unused: [u8; 0],
}

extern "C" {
    pub fn OH_Drawing_PenGetAlpha(arg1: *const OH_Drawing_Pen) -> u8;
}
extern "C" {
    pub fn OH_Drawing_PenSetAlpha(arg1: *mut OH_Drawing_Pen, alpha: u8);
}

extern "C" {
    pub fn OH_Drawing_PenSetShaderEffect(
        arg1: *mut OH_Drawing_Pen,
        arg2: *mut OH_Drawing_ShaderEffect,
    );
}
extern "C" {
    pub fn OH_Drawing_PenSetFilter(arg1: *mut OH_Drawing_Pen, arg2: *mut OH_Drawing_Filter);
}

impl OH_Drawing_PlaceholderVerticalAlignment {
    pub const ALIGNMENT_OFFSET_AT_BASELINE: OH_Drawing_PlaceholderVerticalAlignment =
        OH_Drawing_PlaceholderVerticalAlignment(0);
}
impl OH_Drawing_PlaceholderVerticalAlignment {
    pub const ALIGNMENT_ABOVE_BASELINE: OH_Drawing_PlaceholderVerticalAlignment =
        OH_Drawing_PlaceholderVerticalAlignment(1);
}
impl OH_Drawing_PlaceholderVerticalAlignment {
    pub const ALIGNMENT_BELOW_BASELINE: OH_Drawing_PlaceholderVerticalAlignment =
        OH_Drawing_PlaceholderVerticalAlignment(2);
}
impl OH_Drawing_PlaceholderVerticalAlignment {
    pub const ALIGNMENT_TOP_OF_ROW_BOX: OH_Drawing_PlaceholderVerticalAlignment =
        OH_Drawing_PlaceholderVerticalAlignment(3);
}
impl OH_Drawing_PlaceholderVerticalAlignment {
    pub const ALIGNMENT_BOTTOM_OF_ROW_BOX: OH_Drawing_PlaceholderVerticalAlignment =
        OH_Drawing_PlaceholderVerticalAlignment(4);
}
impl OH_Drawing_PlaceholderVerticalAlignment {
    pub const ALIGNMENT_CENTER_OF_ROW_BOX: OH_Drawing_PlaceholderVerticalAlignment =
        OH_Drawing_PlaceholderVerticalAlignment(5);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct OH_Drawing_PlaceholderVerticalAlignment(pub ::core::ffi::c_uint);
#[repr(C)]
pub struct OH_Drawing_PlaceholderSpan {
    pub width: f64,
    pub height: f64,
    pub alignment: OH_Drawing_PlaceholderVerticalAlignment,
    pub baseline: OH_Drawing_TextBaseline,
    pub baselineOffset: f64,
}
impl OH_Drawing_TextDecorationStyle {
    pub const TEXT_DECORATION_STYLE_SOLID: OH_Drawing_TextDecorationStyle =
        OH_Drawing_TextDecorationStyle(0);
}
impl OH_Drawing_TextDecorationStyle {
    pub const TEXT_DECORATION_STYLE_DOUBLE: OH_Drawing_TextDecorationStyle =
        OH_Drawing_TextDecorationStyle(1);
}
impl OH_Drawing_TextDecorationStyle {
    pub const TEXT_DECORATION_STYLE_DOTTED: OH_Drawing_TextDecorationStyle =
        OH_Drawing_TextDecorationStyle(2);
}
impl OH_Drawing_TextDecorationStyle {
    pub const TEXT_DECORATION_STYLE_DASHED: OH_Drawing_TextDecorationStyle =
        OH_Drawing_TextDecorationStyle(3);
}
impl OH_Drawing_TextDecorationStyle {
    pub const TEXT_DECORATION_STYLE_WAVY: OH_Drawing_TextDecorationStyle =
        OH_Drawing_TextDecorationStyle(4);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct OH_Drawing_TextDecorationStyle(pub ::core::ffi::c_uint);
impl OH_Drawing_EllipsisModal {
    pub const ELLIPSIS_MODAL_HEAD: OH_Drawing_EllipsisModal = OH_Drawing_EllipsisModal(0);
}
impl OH_Drawing_EllipsisModal {
    pub const ELLIPSIS_MODAL_MIDDLE: OH_Drawing_EllipsisModal = OH_Drawing_EllipsisModal(1);
}
impl OH_Drawing_EllipsisModal {
    pub const ELLIPSIS_MODAL_TAIL: OH_Drawing_EllipsisModal = OH_Drawing_EllipsisModal(2);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct OH_Drawing_EllipsisModal(pub ::core::ffi::c_uint);
impl OH_Drawing_BreakStrategy {
    pub const BREAK_STRATEGY_GREEDY: OH_Drawing_BreakStrategy = OH_Drawing_BreakStrategy(0);
}
impl OH_Drawing_BreakStrategy {
    pub const BREAK_STRATEGY_HIGH_QUALITY: OH_Drawing_BreakStrategy = OH_Drawing_BreakStrategy(1);
}
impl OH_Drawing_BreakStrategy {
    pub const BREAK_STRATEGY_BALANCED: OH_Drawing_BreakStrategy = OH_Drawing_BreakStrategy(2);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct OH_Drawing_BreakStrategy(pub ::core::ffi::c_uint);
impl OH_Drawing_WordBreakType {
    pub const WORD_BREAK_TYPE_NORMAL: OH_Drawing_WordBreakType = OH_Drawing_WordBreakType(0);
}
impl OH_Drawing_WordBreakType {
    pub const WORD_BREAK_TYPE_BREAK_ALL: OH_Drawing_WordBreakType = OH_Drawing_WordBreakType(1);
}
impl OH_Drawing_WordBreakType {
    pub const WORD_BREAK_TYPE_BREAK_WORD: OH_Drawing_WordBreakType = OH_Drawing_WordBreakType(2);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct OH_Drawing_WordBreakType(pub ::core::ffi::c_uint);
impl OH_Drawing_RectHeightStyle {
    pub const RECT_HEIGHT_STYLE_TIGHT: OH_Drawing_RectHeightStyle = OH_Drawing_RectHeightStyle(0);
}
impl OH_Drawing_RectHeightStyle {
    pub const RECT_HEIGHT_STYLE_MAX: OH_Drawing_RectHeightStyle = OH_Drawing_RectHeightStyle(1);
}
impl OH_Drawing_RectHeightStyle {
    pub const RECT_HEIGHT_STYLE_INCLUDELINESPACEMIDDLE: OH_Drawing_RectHeightStyle =
        OH_Drawing_RectHeightStyle(2);
}
impl OH_Drawing_RectHeightStyle {
    pub const RECT_HEIGHT_STYLE_INCLUDELINESPACETOP: OH_Drawing_RectHeightStyle =
        OH_Drawing_RectHeightStyle(3);
}
impl OH_Drawing_RectHeightStyle {
    pub const RECT_HEIGHT_STYLE_INCLUDELINESPACEBOTTOM: OH_Drawing_RectHeightStyle =
        OH_Drawing_RectHeightStyle(4);
}
impl OH_Drawing_RectHeightStyle {
    pub const RECT_HEIGHT_STYLE_STRUCT: OH_Drawing_RectHeightStyle = OH_Drawing_RectHeightStyle(5);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct OH_Drawing_RectHeightStyle(pub ::core::ffi::c_uint);
impl OH_Drawing_RectWidthStyle {
    pub const RECT_WIDTH_STYLE_TIGHT: OH_Drawing_RectWidthStyle = OH_Drawing_RectWidthStyle(0);
}
impl OH_Drawing_RectWidthStyle {
    pub const RECT_WIDTH_STYLE_MAX: OH_Drawing_RectWidthStyle = OH_Drawing_RectWidthStyle(1);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct OH_Drawing_RectWidthStyle(pub ::core::ffi::c_uint);

extern "C" {
    pub fn OH_Drawing_TypographyHandlerAddPlaceholder(
        arg1: *mut OH_Drawing_TypographyCreate,
        arg2: *mut OH_Drawing_PlaceholderSpan,
    );
}
extern "C" {
    pub fn OH_Drawing_TypographyDidExceedMaxLines(arg1: *mut OH_Drawing_Typography) -> bool;
}
extern "C" {
    pub fn OH_Drawing_TypographyGetRectsForRange(
        arg1: *mut OH_Drawing_Typography,
        arg2: usize,
        arg3: usize,
        arg4: OH_Drawing_RectHeightStyle,
        arg5: OH_Drawing_RectWidthStyle,
    ) -> *mut OH_Drawing_TextBox;
}
extern "C" {
    pub fn OH_Drawing_TypographyGetRectsForPlaceholders(
        arg1: *mut OH_Drawing_Typography,
    ) -> *mut OH_Drawing_TextBox;
}
extern "C" {
    pub fn OH_Drawing_GetLeftFromTextBox(
        arg1: *mut OH_Drawing_TextBox,
        arg2: ::core::ffi::c_int,
    ) -> f32;
}
extern "C" {
    pub fn OH_Drawing_GetRightFromTextBox(
        arg1: *mut OH_Drawing_TextBox,
        arg2: ::core::ffi::c_int,
    ) -> f32;
}
extern "C" {
    pub fn OH_Drawing_GetTopFromTextBox(
        arg1: *mut OH_Drawing_TextBox,
        arg2: ::core::ffi::c_int,
    ) -> f32;
}
extern "C" {
    pub fn OH_Drawing_GetBottomFromTextBox(
        arg1: *mut OH_Drawing_TextBox,
        arg2: ::core::ffi::c_int,
    ) -> f32;
}
extern "C" {
    pub fn OH_Drawing_GetTextDirectionFromTextBox(
        arg1: *mut OH_Drawing_TextBox,
        arg2: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn OH_Drawing_GetSizeOfTextBox(arg1: *mut OH_Drawing_TextBox) -> usize;
}
extern "C" {
    pub fn OH_Drawing_TypographyGetGlyphPositionAtCoordinate(
        arg1: *mut OH_Drawing_Typography,
        arg2: f64,
        arg3: f64,
    ) -> *mut OH_Drawing_PositionAndAffinity;
}
extern "C" {
    pub fn OH_Drawing_TypographyGetGlyphPositionAtCoordinateWithCluster(
        arg1: *mut OH_Drawing_Typography,
        arg2: f64,
        arg3: f64,
    ) -> *mut OH_Drawing_PositionAndAffinity;
}
extern "C" {
    pub fn OH_Drawing_GetPositionFromPositionAndAffinity(
        arg1: *mut OH_Drawing_PositionAndAffinity,
    ) -> usize;
}
extern "C" {
    pub fn OH_Drawing_GetAffinityFromPositionAndAffinity(
        arg1: *mut OH_Drawing_PositionAndAffinity,
    ) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn OH_Drawing_TypographyGetWordBoundary(
        arg1: *mut OH_Drawing_Typography,
        arg2: usize,
    ) -> *mut OH_Drawing_Range;
}
extern "C" {
    pub fn OH_Drawing_GetStartFromRange(arg1: *mut OH_Drawing_Range) -> usize;
}
extern "C" {
    pub fn OH_Drawing_GetEndFromRange(arg1: *mut OH_Drawing_Range) -> usize;
}
extern "C" {
    pub fn OH_Drawing_TypographyGetLineCount(arg1: *mut OH_Drawing_Typography) -> usize;
}
extern "C" {
    pub fn OH_Drawing_SetTextStyleDecorationStyle(
        arg1: *mut OH_Drawing_TextStyle,
        arg2: ::core::ffi::c_int,
    );
}
extern "C" {
    pub fn OH_Drawing_SetTextStyleDecorationThicknessScale(
        arg1: *mut OH_Drawing_TextStyle,
        arg2: f64,
    );
}
extern "C" {
    pub fn OH_Drawing_SetTextStyleLetterSpacing(arg1: *mut OH_Drawing_TextStyle, arg2: f64);
}
extern "C" {
    pub fn OH_Drawing_SetTextStyleWordSpacing(arg1: *mut OH_Drawing_TextStyle, arg2: f64);
}
extern "C" {
    pub fn OH_Drawing_SetTextStyleHalfLeading(arg1: *mut OH_Drawing_TextStyle, arg2: bool);
}
extern "C" {
    pub fn OH_Drawing_SetTextStyleEllipsis(
        arg1: *mut OH_Drawing_TextStyle,
        arg2: *const ::core::ffi::c_char,
    );
}
extern "C" {
    pub fn OH_Drawing_SetTextStyleEllipsisModal(
        arg1: *mut OH_Drawing_TextStyle,
        arg2: ::core::ffi::c_int,
    );
}
extern "C" {
    pub fn OH_Drawing_SetTypographyTextBreakStrategy(
        arg1: *mut OH_Drawing_TypographyStyle,
        arg2: ::core::ffi::c_int,
    );
}
extern "C" {
    pub fn OH_Drawing_SetTypographyTextWordBreakType(
        arg1: *mut OH_Drawing_TypographyStyle,
        arg2: ::core::ffi::c_int,
    );
}
extern "C" {
    pub fn OH_Drawing_SetTypographyTextEllipsisModal(
        arg1: *mut OH_Drawing_TypographyStyle,
        arg2: ::core::ffi::c_int,
    );
}
extern "C" {
    pub fn OH_Drawing_TypographyGetLineHeight(
        arg1: *mut OH_Drawing_Typography,
        arg2: ::core::ffi::c_int,
    ) -> f64;
}
extern "C" {
    pub fn OH_Drawing_TypographyGetLineWidth(
        arg1: *mut OH_Drawing_Typography,
        arg2: ::core::ffi::c_int,
    ) -> f64;
}
extern "C" {
    pub fn OH_Drawing_ColorFilterCreateBlendMode(
        color: u32,
        arg1: OH_Drawing_BlendMode,
    ) -> *mut OH_Drawing_ColorFilter;
}
extern "C" {
    pub fn OH_Drawing_ColorFilterCreateCompose(
        colorFilter1: *mut OH_Drawing_ColorFilter,
        colorFilter2: *mut OH_Drawing_ColorFilter,
    ) -> *mut OH_Drawing_ColorFilter;
}
extern "C" {
    pub fn OH_Drawing_ColorFilterCreateMatrix(matrix: *const f32) -> *mut OH_Drawing_ColorFilter;
}
extern "C" {
    pub fn OH_Drawing_ColorFilterCreateLinearToSrgbGamma() -> *mut OH_Drawing_ColorFilter;
}
extern "C" {
    pub fn OH_Drawing_ColorFilterCreateSrgbGammaToLinear() -> *mut OH_Drawing_ColorFilter;
}
extern "C" {
    pub fn OH_Drawing_ColorFilterCreateLuma() -> *mut OH_Drawing_ColorFilter;
}
extern "C" {
    pub fn OH_Drawing_ColorFilterDestroy(arg1: *mut OH_Drawing_ColorFilter);
}
extern "C" {
    pub fn OH_Drawing_FilterCreate() -> *mut OH_Drawing_Filter;
}
extern "C" {
    pub fn OH_Drawing_FilterSetMaskFilter(
        arg1: *mut OH_Drawing_Filter,
        arg2: *mut OH_Drawing_MaskFilter,
    );
}
extern "C" {
    pub fn OH_Drawing_FilterSetColorFilter(
        arg1: *mut OH_Drawing_Filter,
        arg2: *mut OH_Drawing_ColorFilter,
    );
}
extern "C" {
    pub fn OH_Drawing_FilterDestroy(arg1: *mut OH_Drawing_Filter);
}
extern "C" {
    pub fn OH_Drawing_FontCreate() -> *mut OH_Drawing_Font;
}
extern "C" {
    pub fn OH_Drawing_FontSetTypeface(arg1: *mut OH_Drawing_Font, arg2: *mut OH_Drawing_Typeface);
}
extern "C" {
    pub fn OH_Drawing_FontSetTextSize(arg1: *mut OH_Drawing_Font, textSize: f32);
}
extern "C" {
    pub fn OH_Drawing_FontSetLinearText(arg1: *mut OH_Drawing_Font, isLinearText: bool);
}
extern "C" {
    pub fn OH_Drawing_FontSetTextSkewX(arg1: *mut OH_Drawing_Font, skewX: f32);
}
extern "C" {
    pub fn OH_Drawing_FontSetFakeBoldText(arg1: *mut OH_Drawing_Font, isFakeBoldText: bool);
}
extern "C" {
    pub fn OH_Drawing_FontDestroy(arg1: *mut OH_Drawing_Font);
}
impl OH_Drawing_BlurType {
    pub const NORMAL: OH_Drawing_BlurType = OH_Drawing_BlurType(0);
}
impl OH_Drawing_BlurType {
    pub const SOLID: OH_Drawing_BlurType = OH_Drawing_BlurType(1);
}
impl OH_Drawing_BlurType {
    pub const OUTER: OH_Drawing_BlurType = OH_Drawing_BlurType(2);
}
impl OH_Drawing_BlurType {
    pub const INNER: OH_Drawing_BlurType = OH_Drawing_BlurType(3);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct OH_Drawing_BlurType(pub ::core::ffi::c_uint);
extern "C" {
    pub fn OH_Drawing_MaskFilterCreateBlur(
        blurType: OH_Drawing_BlurType,
        sigma: f32,
        respectCTM: bool,
    ) -> *mut OH_Drawing_MaskFilter;
}
extern "C" {
    pub fn OH_Drawing_MaskFilterDestroy(arg1: *mut OH_Drawing_MaskFilter);
}
extern "C" {
    pub fn OH_Drawing_MatrixCreate() -> *mut OH_Drawing_Matrix;
}
extern "C" {
    pub fn OH_Drawing_MatrixSetMatrix(
        arg1: *mut OH_Drawing_Matrix,
        scaleX: f32,
        skewX: f32,
        transX: f32,
        skewY: f32,
        scaleY: f32,
        transY: f32,
        persp0: f32,
        persp1: f32,
        persp2: f32,
    );
}
extern "C" {
    pub fn OH_Drawing_MatrixDestroy(arg1: *mut OH_Drawing_Matrix);
}
extern "C" {
    pub fn OH_Drawing_PointCreate(x: f32, y: f32) -> *mut OH_Drawing_Point;
}
extern "C" {
    pub fn OH_Drawing_PointDestroy(arg1: *mut OH_Drawing_Point);
}
extern "C" {
    pub fn OH_Drawing_RectCreate(
        left: f32,
        top: f32,
        right: f32,
        bottom: f32,
    ) -> *mut OH_Drawing_Rect;
}
extern "C" {
    pub fn OH_Drawing_RectDestroy(arg1: *mut OH_Drawing_Rect);
}
extern "C" {
    pub fn OH_Drawing_RegisterFont(
        arg1: *mut OH_Drawing_FontCollection,
        fontFamily: *const ::core::ffi::c_char,
        familySrc: *const ::core::ffi::c_char,
    ) -> u32;
}
extern "C" {
    pub fn OH_Drawing_RegisterFontBuffer(
        arg1: *mut OH_Drawing_FontCollection,
        fontFamily: *const ::core::ffi::c_char,
        fontBuffer: *mut u8,
        length: usize,
    ) -> u32;
}
extern "C" {
    pub fn OH_Drawing_RoundRectCreate(
        arg1: *const OH_Drawing_Rect,
        xRad: f32,
        yRad: f32,
    ) -> *mut OH_Drawing_RoundRect;
}
extern "C" {
    pub fn OH_Drawing_RoundRectDestroy(arg1: *mut OH_Drawing_RoundRect);
}
impl OH_Drawing_TileMode {
    pub const CLAMP: OH_Drawing_TileMode = OH_Drawing_TileMode(0);
}
impl OH_Drawing_TileMode {
    pub const REPEAT: OH_Drawing_TileMode = OH_Drawing_TileMode(1);
}
impl OH_Drawing_TileMode {
    pub const MIRROR: OH_Drawing_TileMode = OH_Drawing_TileMode(2);
}
impl OH_Drawing_TileMode {
    pub const DECAL: OH_Drawing_TileMode = OH_Drawing_TileMode(3);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct OH_Drawing_TileMode(pub ::core::ffi::c_uint);
extern "C" {
    pub fn OH_Drawing_ShaderEffectCreateLinearGradient(
        startPt: *const OH_Drawing_Point,
        endPt: *const OH_Drawing_Point,
        colors: *const u32,
        pos: *const f32,
        size: u32,
        arg1: OH_Drawing_TileMode,
    ) -> *mut OH_Drawing_ShaderEffect;
}
extern "C" {
    pub fn OH_Drawing_ShaderEffectCreateRadialGradient(
        centerPt: *const OH_Drawing_Point,
        radius: f32,
        colors: *const u32,
        pos: *const f32,
        size: u32,
        arg1: OH_Drawing_TileMode,
    ) -> *mut OH_Drawing_ShaderEffect;
}
extern "C" {
    pub fn OH_Drawing_ShaderEffectCreateSweepGradient(
        centerPt: *const OH_Drawing_Point,
        colors: *const u32,
        pos: *const f32,
        size: u32,
        arg1: OH_Drawing_TileMode,
    ) -> *mut OH_Drawing_ShaderEffect;
}
extern "C" {
    pub fn OH_Drawing_ShaderEffectDestroy(arg1: *mut OH_Drawing_ShaderEffect);
}
extern "C" {
    pub fn OH_Drawing_TextBlobBuilderCreate() -> *mut OH_Drawing_TextBlobBuilder;
}
#[repr(C)]
pub struct OH_Drawing_RunBuffer {
    pub glyphs: *mut u16,
    pub pos: *mut f32,
    pub utf8text: *mut ::core::ffi::c_char,
    pub clusters: *mut u32,
}
extern "C" {
    pub fn OH_Drawing_TextBlobBuilderAllocRunPos(
        arg1: *mut OH_Drawing_TextBlobBuilder,
        arg2: *const OH_Drawing_Font,
        count: i32,
        arg3: *const OH_Drawing_Rect,
    ) -> *const OH_Drawing_RunBuffer;
}
extern "C" {
    pub fn OH_Drawing_TextBlobBuilderMake(
        arg1: *mut OH_Drawing_TextBlobBuilder,
    ) -> *mut OH_Drawing_TextBlob;
}
extern "C" {
    pub fn OH_Drawing_TextBlobDestroy(arg1: *mut OH_Drawing_TextBlob);
}
extern "C" {
    pub fn OH_Drawing_TextBlobBuilderDestroy(arg1: *mut OH_Drawing_TextBlobBuilder);
}
extern "C" {
    pub fn OH_Drawing_TypefaceCreateDefault() -> *mut OH_Drawing_Typeface;
}
extern "C" {
    pub fn OH_Drawing_TypefaceDestroy(arg1: *mut OH_Drawing_Typeface);
}
