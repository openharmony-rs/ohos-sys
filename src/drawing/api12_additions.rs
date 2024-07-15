//! Contains the Additions of API-11 compared to API-10.
//
// This file is manually created from the diff between `drawing_api11` and `drawing_api12`.
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use super::{
    OH_Drawing_AlphaFormat, OH_Drawing_Bitmap, OH_Drawing_BlendMode, OH_Drawing_Brush,
    OH_Drawing_Canvas, OH_Drawing_CanvasClipOp, OH_Drawing_ColorFilter, OH_Drawing_ColorFormat,
    OH_Drawing_Filter, OH_Drawing_Font, OH_Drawing_FontCollection, OH_Drawing_FontStyle,
    OH_Drawing_FontWeight, OH_Drawing_Matrix, OH_Drawing_Path, OH_Drawing_Pen, OH_Drawing_Point,
    OH_Drawing_Range, OH_Drawing_Rect, OH_Drawing_RoundRect, OH_Drawing_ShaderEffect,
    OH_Drawing_TextAlign, OH_Drawing_TextBaseline, OH_Drawing_TextBlob, OH_Drawing_TextBox,
    OH_Drawing_TextDecorationStyle, OH_Drawing_TextDirection, OH_Drawing_TextStyle,
    OH_Drawing_TileMode, OH_Drawing_Typeface, OH_Drawing_Typography, OH_Drawing_TypographyCreate,
    OH_Drawing_TypographyStyle,
};

#[repr(C)]
pub struct OH_Drawing_Region {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct OH_Drawing_PixelMap {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct OH_Drawing_ColorSpace {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct OH_Drawing_Point2D {
    pub x: f32,
    pub y: f32,
}
pub type OH_Drawing_Corner_Radii = OH_Drawing_Point2D;
#[repr(C)]
pub struct OH_Drawing_Point3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
#[repr(C)]
pub struct OH_Drawing_PathEffect {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct OH_Drawing_ShadowLayer {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct OH_Drawing_MemoryStream {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct OH_Drawing_Image {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct OH_Drawing_ImageFilter {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct OH_Drawing_SamplingOptions {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct OH_Drawing_GpuContext {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct OH_Drawing_Surface {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OH_Drawing_Image_Info {
    pub width: i32,
    pub height: i32,
    pub colorType: OH_Drawing_ColorFormat,
    pub alphaType: OH_Drawing_AlphaFormat,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OH_Drawing_RectStyle_Info {
    pub color: u32,
    pub leftTopRadius: f64,
    pub rightTopRadius: f64,
    pub rightBottomRadius: f64,
    pub leftBottomRadius: f64,
}
impl OH_Drawing_TextEncoding {
    pub const TEXT_ENCODING_UTF8: OH_Drawing_TextEncoding = OH_Drawing_TextEncoding(0);
}
impl OH_Drawing_TextEncoding {
    pub const TEXT_ENCODING_UTF16: OH_Drawing_TextEncoding = OH_Drawing_TextEncoding(1);
}
impl OH_Drawing_TextEncoding {
    pub const TEXT_ENCODING_UTF32: OH_Drawing_TextEncoding = OH_Drawing_TextEncoding(2);
}
impl OH_Drawing_TextEncoding {
    pub const TEXT_ENCODING_GLYPH_ID: OH_Drawing_TextEncoding = OH_Drawing_TextEncoding(3);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct OH_Drawing_TextEncoding(pub ::core::ffi::c_uint);
#[repr(C)]
pub struct OH_Drawing_FontMgr {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct OH_Drawing_FontStyleSet {
    _unused: [u8; 0],
}
extern "C" {
    pub fn OH_Drawing_BitmapCreateFromPixels(
        arg1: *mut OH_Drawing_Image_Info,
        pixels: *mut ::core::ffi::c_void,
        rowBytes: u32,
    ) -> *mut OH_Drawing_Bitmap;
}
extern "C" {
    pub fn OH_Drawing_BitmapGetColorFormat(arg1: *mut OH_Drawing_Bitmap) -> OH_Drawing_ColorFormat;
}
extern "C" {
    pub fn OH_Drawing_BitmapGetAlphaFormat(arg1: *mut OH_Drawing_Bitmap) -> OH_Drawing_AlphaFormat;
}
extern "C" {
    pub fn OH_Drawing_BitmapGetImageInfo(
        arg1: *mut OH_Drawing_Bitmap,
        arg2: *mut OH_Drawing_Image_Info,
    );
}
extern "C" {
    pub fn OH_Drawing_BitmapReadPixels(
        arg1: *mut OH_Drawing_Bitmap,
        dstInfo: *const OH_Drawing_Image_Info,
        dstPixels: *mut ::core::ffi::c_void,
        dstRowBytes: usize,
        srcX: i32,
        srcY: i32,
    ) -> bool;
}
extern "C" {
    pub fn OH_Drawing_BrushCopy(arg1: *mut OH_Drawing_Brush) -> *mut OH_Drawing_Brush;
}
extern "C" {
    pub fn OH_Drawing_BrushSetShadowLayer(
        arg1: *mut OH_Drawing_Brush,
        arg2: *mut OH_Drawing_ShadowLayer,
    );
}
extern "C" {
    pub fn OH_Drawing_BrushGetFilter(arg1: *mut OH_Drawing_Brush, arg2: *mut OH_Drawing_Filter);
}
extern "C" {
    pub fn OH_Drawing_BrushSetBlendMode(arg1: *mut OH_Drawing_Brush, arg2: OH_Drawing_BlendMode);
}
extern "C" {
    pub fn OH_Drawing_BrushReset(arg1: *mut OH_Drawing_Brush);
}
impl OH_Drawing_SrcRectConstraint {
    pub const STRICT_SRC_RECT_CONSTRAINT: OH_Drawing_SrcRectConstraint =
        OH_Drawing_SrcRectConstraint(0);
}
impl OH_Drawing_SrcRectConstraint {
    pub const FAST_SRC_RECT_CONSTRAINT: OH_Drawing_SrcRectConstraint =
        OH_Drawing_SrcRectConstraint(1);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct OH_Drawing_SrcRectConstraint(pub ::core::ffi::c_uint);

extern "C" {
    pub fn OH_Drawing_CanvasSaveLayer(
        arg1: *mut OH_Drawing_Canvas,
        arg2: *const OH_Drawing_Rect,
        arg3: *const OH_Drawing_Brush,
    );
}

extern "C" {
    pub fn OH_Drawing_CanvasDrawPixelMapRect(
        arg1: *mut OH_Drawing_Canvas,
        arg2: *mut OH_Drawing_PixelMap,
        src: *const OH_Drawing_Rect,
        dst: *const OH_Drawing_Rect,
        arg3: *const OH_Drawing_SamplingOptions,
    );
}
extern "C" {
    pub fn OH_Drawing_CanvasDrawBackground(
        arg1: *mut OH_Drawing_Canvas,
        arg2: *const OH_Drawing_Brush,
    );
}
extern "C" {
    pub fn OH_Drawing_CanvasDrawRegion(
        arg1: *mut OH_Drawing_Canvas,
        arg2: *const OH_Drawing_Region,
    );
}
impl OH_Drawing_PointMode {
    pub const POINT_MODE_POINTS: OH_Drawing_PointMode = OH_Drawing_PointMode(0);
}
impl OH_Drawing_PointMode {
    pub const POINT_MODE_LINES: OH_Drawing_PointMode = OH_Drawing_PointMode(1);
}
impl OH_Drawing_PointMode {
    pub const POINT_MODE_POLYGON: OH_Drawing_PointMode = OH_Drawing_PointMode(2);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct OH_Drawing_PointMode(pub ::core::ffi::c_uint);
extern "C" {
    pub fn OH_Drawing_CanvasDrawPoints(
        arg1: *mut OH_Drawing_Canvas,
        mode: OH_Drawing_PointMode,
        count: u32,
        arg2: *const OH_Drawing_Point2D,
    );
}

extern "C" {
    pub fn OH_Drawing_CanvasDrawBitmapRect(
        arg1: *mut OH_Drawing_Canvas,
        arg2: *const OH_Drawing_Bitmap,
        src: *const OH_Drawing_Rect,
        dst: *const OH_Drawing_Rect,
        arg3: *const OH_Drawing_SamplingOptions,
    );
}

extern "C" {
    pub fn OH_Drawing_CanvasClipRoundRect(
        arg1: *mut OH_Drawing_Canvas,
        arg2: *const OH_Drawing_RoundRect,
        clipOp: OH_Drawing_CanvasClipOp,
        doAntiAlias: bool,
    );
}

extern "C" {
    pub fn OH_Drawing_CanvasSkew(arg1: *mut OH_Drawing_Canvas, sx: f32, sy: f32);
}

extern "C" {
    pub fn OH_Drawing_CanvasGetWidth(arg1: *mut OH_Drawing_Canvas) -> i32;
}
extern "C" {
    pub fn OH_Drawing_CanvasGetHeight(arg1: *mut OH_Drawing_Canvas) -> i32;
}
extern "C" {
    pub fn OH_Drawing_CanvasGetLocalClipBounds(
        arg1: *mut OH_Drawing_Canvas,
        arg2: *mut OH_Drawing_Rect,
    );
}
extern "C" {
    pub fn OH_Drawing_CanvasGetTotalMatrix(
        arg1: *mut OH_Drawing_Canvas,
        arg2: *mut OH_Drawing_Matrix,
    );
}
extern "C" {
    pub fn OH_Drawing_CanvasConcatMatrix(
        arg1: *mut OH_Drawing_Canvas,
        arg2: *mut OH_Drawing_Matrix,
    );
}
impl OH_Drawing_CanvasShadowFlags {
    pub const SHADOW_FLAGS_NONE: OH_Drawing_CanvasShadowFlags = OH_Drawing_CanvasShadowFlags(0);
}
impl OH_Drawing_CanvasShadowFlags {
    pub const SHADOW_FLAGS_TRANSPARENT_OCCLUDER: OH_Drawing_CanvasShadowFlags =
        OH_Drawing_CanvasShadowFlags(1);
}
impl OH_Drawing_CanvasShadowFlags {
    pub const SHADOW_FLAGS_GEOMETRIC_ONLY: OH_Drawing_CanvasShadowFlags =
        OH_Drawing_CanvasShadowFlags(2);
}
impl OH_Drawing_CanvasShadowFlags {
    pub const SHADOW_FLAGS_ALL: OH_Drawing_CanvasShadowFlags = OH_Drawing_CanvasShadowFlags(3);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct OH_Drawing_CanvasShadowFlags(pub ::core::ffi::c_uint);
extern "C" {
    pub fn OH_Drawing_CanvasDrawShadow(
        arg1: *mut OH_Drawing_Canvas,
        arg2: *mut OH_Drawing_Path,
        planeParams: OH_Drawing_Point3D,
        devLightPos: OH_Drawing_Point3D,
        lightRadius: f32,
        ambientColor: u32,
        spotColor: u32,
        flag: OH_Drawing_CanvasShadowFlags,
    );
}
extern "C" {
    pub fn OH_Drawing_CanvasSetMatrix(arg1: *mut OH_Drawing_Canvas, arg2: *mut OH_Drawing_Matrix);
}
extern "C" {
    pub fn OH_Drawing_CanvasResetMatrix(arg1: *mut OH_Drawing_Canvas);
}
extern "C" {
    pub fn OH_Drawing_CanvasDrawImageRectWithSrc(
        arg1: *mut OH_Drawing_Canvas,
        arg2: *const OH_Drawing_Image,
        src: *const OH_Drawing_Rect,
        dst: *const OH_Drawing_Rect,
        arg3: *const OH_Drawing_SamplingOptions,
        arg4: OH_Drawing_SrcRectConstraint,
    );
}
extern "C" {
    pub fn OH_Drawing_CanvasDrawImageRect(
        arg1: *mut OH_Drawing_Canvas,
        arg2: *mut OH_Drawing_Image,
        dst: *mut OH_Drawing_Rect,
        arg3: *mut OH_Drawing_SamplingOptions,
    );
}
impl OH_Drawing_VertexMode {
    pub const VERTEX_MODE_TRIANGLES: OH_Drawing_VertexMode = OH_Drawing_VertexMode(0);
}
impl OH_Drawing_VertexMode {
    pub const VERTEX_MODE_TRIANGLES_STRIP: OH_Drawing_VertexMode = OH_Drawing_VertexMode(1);
}
impl OH_Drawing_VertexMode {
    pub const VERTEX_MODE_TRIANGLE_FAN: OH_Drawing_VertexMode = OH_Drawing_VertexMode(2);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct OH_Drawing_VertexMode(pub ::core::ffi::c_uint);
extern "C" {
    pub fn OH_Drawing_CanvasDrawVertices(
        arg1: *mut OH_Drawing_Canvas,
        vertexMmode: OH_Drawing_VertexMode,
        vertexCount: i32,
        positions: *const OH_Drawing_Point2D,
        texs: *const OH_Drawing_Point2D,
        colors: *const u32,
        indexCount: i32,
        indices: *const u16,
        mode: OH_Drawing_BlendMode,
    );
}
extern "C" {
    pub fn OH_Drawing_CanvasReadPixels(
        arg1: *mut OH_Drawing_Canvas,
        arg2: *mut OH_Drawing_Image_Info,
        dstPixels: *mut ::core::ffi::c_void,
        dstRowBytes: u32,
        srcX: i32,
        srcY: i32,
    ) -> bool;
}
extern "C" {
    pub fn OH_Drawing_CanvasReadPixelsToBitmap(
        arg1: *mut OH_Drawing_Canvas,
        arg2: *mut OH_Drawing_Bitmap,
        srcX: i32,
        srcY: i32,
    ) -> bool;
}

#[repr(C)]
pub struct OH_Drawing_FontParser {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct OH_Drawing_TextShadow {
    _unused: [u8; 0],
}

extern "C" {
    pub fn OH_Drawing_DisableFontCollectionFallback(arg1: *mut OH_Drawing_FontCollection);
}
extern "C" {
    pub fn OH_Drawing_DisableFontCollectionSystemFont(arg1: *mut OH_Drawing_FontCollection);
}
extern "C" {
    pub fn OH_Drawing_CreateSharedFontCollection() -> *mut OH_Drawing_FontCollection;
}
impl OH_Drawing_PathDirection {
    pub const PATH_DIRECTION_CW: OH_Drawing_PathDirection = OH_Drawing_PathDirection(0);
}
impl OH_Drawing_PathDirection {
    pub const PATH_DIRECTION_CCW: OH_Drawing_PathDirection = OH_Drawing_PathDirection(1);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct OH_Drawing_PathDirection(pub ::core::ffi::c_uint);
impl OH_Drawing_PathFillType {
    pub const PATH_FILL_TYPE_WINDING: OH_Drawing_PathFillType = OH_Drawing_PathFillType(0);
}
impl OH_Drawing_PathFillType {
    pub const PATH_FILL_TYPE_EVEN_ODD: OH_Drawing_PathFillType = OH_Drawing_PathFillType(1);
}
impl OH_Drawing_PathFillType {
    pub const PATH_FILL_TYPE_INVERSE_WINDING: OH_Drawing_PathFillType = OH_Drawing_PathFillType(2);
}
impl OH_Drawing_PathFillType {
    pub const PATH_FILL_TYPE_INVERSE_EVEN_ODD: OH_Drawing_PathFillType = OH_Drawing_PathFillType(3);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct OH_Drawing_PathFillType(pub ::core::ffi::c_uint);
impl OH_Drawing_PathAddMode {
    pub const PATH_ADD_MODE_APPEND: OH_Drawing_PathAddMode = OH_Drawing_PathAddMode(0);
}
impl OH_Drawing_PathAddMode {
    pub const PATH_ADD_MODE_EXTEND: OH_Drawing_PathAddMode = OH_Drawing_PathAddMode(1);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct OH_Drawing_PathAddMode(pub ::core::ffi::c_uint);
impl OH_Drawing_PathOpMode {
    pub const PATH_OP_MODE_DIFFERENCE: OH_Drawing_PathOpMode = OH_Drawing_PathOpMode(0);
}
impl OH_Drawing_PathOpMode {
    pub const PATH_OP_MODE_INTERSECT: OH_Drawing_PathOpMode = OH_Drawing_PathOpMode(1);
}
impl OH_Drawing_PathOpMode {
    pub const PATH_OP_MODE_UNION: OH_Drawing_PathOpMode = OH_Drawing_PathOpMode(2);
}
impl OH_Drawing_PathOpMode {
    pub const PATH_OP_MODE_XOR: OH_Drawing_PathOpMode = OH_Drawing_PathOpMode(3);
}
impl OH_Drawing_PathOpMode {
    pub const PATH_OP_MODE_REVERSE_DIFFERENCE: OH_Drawing_PathOpMode = OH_Drawing_PathOpMode(4);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct OH_Drawing_PathOpMode(pub ::core::ffi::c_uint);
impl OH_Drawing_PathMeasureMatrixFlags {
    pub const GET_POSITION_MATRIX: OH_Drawing_PathMeasureMatrixFlags =
        OH_Drawing_PathMeasureMatrixFlags(0);
}
impl OH_Drawing_PathMeasureMatrixFlags {
    pub const GET_TANGENT_MATRIX: OH_Drawing_PathMeasureMatrixFlags =
        OH_Drawing_PathMeasureMatrixFlags(1);
}
impl OH_Drawing_PathMeasureMatrixFlags {
    pub const GET_POSITION_AND_TANGENT_MATRIX: OH_Drawing_PathMeasureMatrixFlags =
        OH_Drawing_PathMeasureMatrixFlags(2);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct OH_Drawing_PathMeasureMatrixFlags(pub ::core::ffi::c_uint);

extern "C" {
    pub fn OH_Drawing_PathCopy(arg1: *mut OH_Drawing_Path) -> *mut OH_Drawing_Path;
}

extern "C" {
    pub fn OH_Drawing_PathConicTo(
        arg1: *mut OH_Drawing_Path,
        ctrlX: f32,
        ctrlY: f32,
        endX: f32,
        endY: f32,
        weight: f32,
    );
}

extern "C" {
    pub fn OH_Drawing_PathRMoveTo(arg1: *mut OH_Drawing_Path, x: f32, y: f32);
}
extern "C" {
    pub fn OH_Drawing_PathRLineTo(arg1: *mut OH_Drawing_Path, x: f32, y: f32);
}
extern "C" {
    pub fn OH_Drawing_PathRQuadTo(
        arg1: *mut OH_Drawing_Path,
        ctrlX: f32,
        ctrlY: f32,
        endX: f32,
        endY: f32,
    );
}
extern "C" {
    pub fn OH_Drawing_PathRConicTo(
        arg1: *mut OH_Drawing_Path,
        ctrlX: f32,
        ctrlY: f32,
        endX: f32,
        endY: f32,
        weight: f32,
    );
}
extern "C" {
    pub fn OH_Drawing_PathRCubicTo(
        arg1: *mut OH_Drawing_Path,
        ctrlX1: f32,
        ctrlY1: f32,
        ctrlX2: f32,
        ctrlY2: f32,
        endX: f32,
        endY: f32,
    );
}
extern "C" {
    pub fn OH_Drawing_PathAddRect(
        arg1: *mut OH_Drawing_Path,
        left: f32,
        top: f32,
        right: f32,
        bottom: f32,
        arg2: OH_Drawing_PathDirection,
    );
}
extern "C" {
    pub fn OH_Drawing_PathAddRectWithInitialCorner(
        arg1: *mut OH_Drawing_Path,
        arg2: *const OH_Drawing_Rect,
        arg3: OH_Drawing_PathDirection,
        start: u32,
    );
}
extern "C" {
    pub fn OH_Drawing_PathAddRoundRect(
        arg1: *mut OH_Drawing_Path,
        roundRect: *const OH_Drawing_RoundRect,
        arg2: OH_Drawing_PathDirection,
    );
}
extern "C" {
    pub fn OH_Drawing_PathAddOvalWithInitialPoint(
        arg1: *mut OH_Drawing_Path,
        arg2: *const OH_Drawing_Rect,
        start: u32,
        arg3: OH_Drawing_PathDirection,
    );
}
extern "C" {
    pub fn OH_Drawing_PathAddOval(
        arg1: *mut OH_Drawing_Path,
        arg2: *const OH_Drawing_Rect,
        arg3: OH_Drawing_PathDirection,
    );
}
extern "C" {
    pub fn OH_Drawing_PathAddArc(
        arg1: *mut OH_Drawing_Path,
        arg2: *const OH_Drawing_Rect,
        startAngle: f32,
        sweepAngle: f32,
    );
}
extern "C" {
    pub fn OH_Drawing_PathAddPath(
        arg1: *mut OH_Drawing_Path,
        src: *const OH_Drawing_Path,
        arg2: *const OH_Drawing_Matrix,
    );
}
extern "C" {
    pub fn OH_Drawing_PathAddPathWithMatrixAndMode(
        path: *mut OH_Drawing_Path,
        src: *const OH_Drawing_Path,
        arg1: *const OH_Drawing_Matrix,
        arg2: OH_Drawing_PathAddMode,
    );
}
extern "C" {
    pub fn OH_Drawing_PathAddPathWithMode(
        path: *mut OH_Drawing_Path,
        src: *const OH_Drawing_Path,
        arg1: OH_Drawing_PathAddMode,
    );
}
extern "C" {
    pub fn OH_Drawing_PathAddPathWithOffsetAndMode(
        path: *mut OH_Drawing_Path,
        src: *const OH_Drawing_Path,
        dx: f32,
        dy: f32,
        arg1: OH_Drawing_PathAddMode,
    );
}
extern "C" {
    pub fn OH_Drawing_PathAddPolygon(
        path: *mut OH_Drawing_Path,
        points: *const OH_Drawing_Point2D,
        count: u32,
        isClosed: bool,
    );
}
extern "C" {
    pub fn OH_Drawing_PathAddCircle(
        path: *mut OH_Drawing_Path,
        x: f32,
        y: f32,
        radius: f32,
        arg1: OH_Drawing_PathDirection,
    );
}
extern "C" {
    pub fn OH_Drawing_PathBuildFromSvgString(
        path: *mut OH_Drawing_Path,
        str_: *const ::core::ffi::c_char,
    ) -> bool;
}
extern "C" {
    pub fn OH_Drawing_PathContains(arg1: *mut OH_Drawing_Path, x: f32, y: f32) -> bool;
}
extern "C" {
    pub fn OH_Drawing_PathTransform(arg1: *mut OH_Drawing_Path, arg2: *const OH_Drawing_Matrix);
}
extern "C" {
    pub fn OH_Drawing_PathTransformWithPerspectiveClip(
        src: *mut OH_Drawing_Path,
        arg1: *const OH_Drawing_Matrix,
        dst: *mut OH_Drawing_Path,
        applyPerspectiveClip: bool,
    );
}
extern "C" {
    pub fn OH_Drawing_PathSetFillType(arg1: *mut OH_Drawing_Path, arg2: OH_Drawing_PathFillType);
}
extern "C" {
    pub fn OH_Drawing_PathGetLength(arg1: *mut OH_Drawing_Path, forceClosed: bool) -> f32;
}
extern "C" {
    pub fn OH_Drawing_PathGetBounds(arg1: *mut OH_Drawing_Path, arg2: *mut OH_Drawing_Rect);
}

extern "C" {
    pub fn OH_Drawing_PathOffset(
        path: *mut OH_Drawing_Path,
        dst: *mut OH_Drawing_Path,
        dx: f32,
        dy: f32,
    );
}

extern "C" {
    pub fn OH_Drawing_PathIsClosed(path: *mut OH_Drawing_Path, forceClosed: bool) -> bool;
}
extern "C" {
    pub fn OH_Drawing_PathGetPositionTangent(
        path: *mut OH_Drawing_Path,
        forceClosed: bool,
        distance: f32,
        position: *mut OH_Drawing_Point2D,
        tangent: *mut OH_Drawing_Point2D,
    ) -> bool;
}
extern "C" {
    pub fn OH_Drawing_PathOp(
        path: *mut OH_Drawing_Path,
        srcPath: *const OH_Drawing_Path,
        op: OH_Drawing_PathOpMode,
    ) -> bool;
}
extern "C" {
    pub fn OH_Drawing_PathGetMatrix(
        path: *mut OH_Drawing_Path,
        forceClosed: bool,
        distance: f32,
        matrix: *mut OH_Drawing_Matrix,
        flag: OH_Drawing_PathMeasureMatrixFlags,
    ) -> bool;
}

extern "C" {
    pub fn OH_Drawing_PenCopy(arg1: *mut OH_Drawing_Pen) -> *mut OH_Drawing_Pen;
}

extern "C" {
    pub fn OH_Drawing_PenSetShadowLayer(
        arg1: *mut OH_Drawing_Pen,
        arg2: *mut OH_Drawing_ShadowLayer,
    );
}
extern "C" {
    pub fn OH_Drawing_PenSetPathEffect(arg1: *mut OH_Drawing_Pen, arg2: *mut OH_Drawing_PathEffect);
}

extern "C" {
    pub fn OH_Drawing_PenGetFilter(arg1: *mut OH_Drawing_Pen, arg2: *mut OH_Drawing_Filter);
}
extern "C" {
    pub fn OH_Drawing_PenSetBlendMode(arg1: *mut OH_Drawing_Pen, arg2: OH_Drawing_BlendMode);
}
extern "C" {
    pub fn OH_Drawing_PenGetFillPath(
        arg1: *mut OH_Drawing_Pen,
        src: *const OH_Drawing_Path,
        dst: *mut OH_Drawing_Path,
        arg2: *const OH_Drawing_Rect,
        arg3: *const OH_Drawing_Matrix,
    ) -> bool;
}
extern "C" {
    pub fn OH_Drawing_PenReset(arg1: *mut OH_Drawing_Pen);
}
impl OH_Drawing_FontHinting {
    pub const FONT_HINTING_NONE: OH_Drawing_FontHinting = OH_Drawing_FontHinting(0);
}
impl OH_Drawing_FontHinting {
    pub const FONT_HINTING_SLIGHT: OH_Drawing_FontHinting = OH_Drawing_FontHinting(1);
}
impl OH_Drawing_FontHinting {
    pub const FONT_HINTING_NORMAL: OH_Drawing_FontHinting = OH_Drawing_FontHinting(2);
}
impl OH_Drawing_FontHinting {
    pub const FONT_HINTING_FULL: OH_Drawing_FontHinting = OH_Drawing_FontHinting(3);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct OH_Drawing_FontHinting(pub ::core::ffi::c_uint);
impl OH_Drawing_FontEdging {
    pub const FONT_EDGING_ALIAS: OH_Drawing_FontEdging = OH_Drawing_FontEdging(0);
}
impl OH_Drawing_FontEdging {
    pub const FONT_EDGING_ANTI_ALIAS: OH_Drawing_FontEdging = OH_Drawing_FontEdging(1);
}
impl OH_Drawing_FontEdging {
    pub const FONT_EDGING_SUBPIXEL_ANTI_ALIAS: OH_Drawing_FontEdging = OH_Drawing_FontEdging(2);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct OH_Drawing_FontEdging(pub ::core::ffi::c_uint);
extern "C" {
    pub fn OH_Drawing_FontSetBaselineSnap(arg1: *mut OH_Drawing_Font, baselineSnap: bool);
}
extern "C" {
    pub fn OH_Drawing_FontIsBaselineSnap(arg1: *const OH_Drawing_Font) -> bool;
}
extern "C" {
    pub fn OH_Drawing_FontSetSubpixel(arg1: *mut OH_Drawing_Font, isSubpixel: bool);
}
extern "C" {
    pub fn OH_Drawing_FontIsSubpixel(arg1: *const OH_Drawing_Font) -> bool;
}
extern "C" {
    pub fn OH_Drawing_FontSetForceAutoHinting(arg1: *mut OH_Drawing_Font, isForceAutoHinting: bool);
}
extern "C" {
    pub fn OH_Drawing_FontIsForceAutoHinting(arg1: *const OH_Drawing_Font) -> bool;
}
extern "C" {
    pub fn OH_Drawing_FontGetTypeface(arg1: *mut OH_Drawing_Font) -> *mut OH_Drawing_Typeface;
}
extern "C" {
    pub fn OH_Drawing_FontGetTextSize(arg1: *const OH_Drawing_Font) -> f32;
}
extern "C" {
    pub fn OH_Drawing_FontCountText(
        arg1: *mut OH_Drawing_Font,
        text: *const ::core::ffi::c_void,
        byteLength: usize,
        encoding: OH_Drawing_TextEncoding,
    ) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn OH_Drawing_FontTextToGlyphs(
        arg1: *const OH_Drawing_Font,
        text: *const ::core::ffi::c_void,
        byteLength: u32,
        encoding: OH_Drawing_TextEncoding,
        glyphs: *mut u16,
        maxGlyphCount: ::core::ffi::c_int,
    ) -> u32;
}
extern "C" {
    pub fn OH_Drawing_FontGetWidths(
        arg1: *const OH_Drawing_Font,
        glyphs: *const u16,
        count: ::core::ffi::c_int,
        widths: *mut f32,
    );
}
extern "C" {
    pub fn OH_Drawing_FontIsLinearText(arg1: *const OH_Drawing_Font) -> bool;
}
extern "C" {
    pub fn OH_Drawing_FontGetTextSkewX(arg1: *const OH_Drawing_Font) -> f32;
}

extern "C" {
    pub fn OH_Drawing_FontIsFakeBoldText(arg1: *const OH_Drawing_Font) -> bool;
}
extern "C" {
    pub fn OH_Drawing_FontSetScaleX(arg1: *mut OH_Drawing_Font, scaleX: f32);
}
extern "C" {
    pub fn OH_Drawing_FontGetScaleX(arg1: *const OH_Drawing_Font) -> f32;
}
extern "C" {
    pub fn OH_Drawing_FontSetHinting(arg1: *mut OH_Drawing_Font, arg2: OH_Drawing_FontHinting);
}
extern "C" {
    pub fn OH_Drawing_FontGetHinting(arg1: *const OH_Drawing_Font) -> OH_Drawing_FontHinting;
}
extern "C" {
    pub fn OH_Drawing_FontSetEmbeddedBitmaps(arg1: *mut OH_Drawing_Font, isEmbeddedBitmaps: bool);
}
extern "C" {
    pub fn OH_Drawing_FontIsEmbeddedBitmaps(arg1: *const OH_Drawing_Font) -> bool;
}
extern "C" {
    pub fn OH_Drawing_FontSetEdging(arg1: *mut OH_Drawing_Font, arg2: OH_Drawing_FontEdging);
}
extern "C" {
    pub fn OH_Drawing_FontGetEdging(arg1: *const OH_Drawing_Font) -> OH_Drawing_FontEdging;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OH_Drawing_Font_Metrics {
    pub flags: u32,
    pub top: f32,
    pub ascent: f32,
    pub descent: f32,
    pub bottom: f32,
    pub leading: f32,
    pub avgCharWidth: f32,
    pub maxCharWidth: f32,
    pub xMin: f32,
    pub xMax: f32,
    pub xHeight: f32,
    pub capHeight: f32,
    pub underlineThickness: f32,
    pub underlinePosition: f32,
    pub strikeoutThickness: f32,
    pub strikeoutPosition: f32,
}
extern "C" {
    pub fn OH_Drawing_FontGetMetrics(
        arg1: *mut OH_Drawing_Font,
        arg2: *mut OH_Drawing_Font_Metrics,
    ) -> f32;
}

impl OH_Drawing_FontStyle {
    pub const FONT_STYLE_OBLIQUE: OH_Drawing_FontStyle = OH_Drawing_FontStyle(2);
}

impl OH_Drawing_FontConfigInfoErrorCode {
    pub const SUCCESS_FONT_CONFIG_INFO: OH_Drawing_FontConfigInfoErrorCode =
        OH_Drawing_FontConfigInfoErrorCode(0);
}
impl OH_Drawing_FontConfigInfoErrorCode {
    pub const ERROR_FONT_CONFIG_INFO_UNKNOWN: OH_Drawing_FontConfigInfoErrorCode =
        OH_Drawing_FontConfigInfoErrorCode(1);
}
impl OH_Drawing_FontConfigInfoErrorCode {
    pub const ERROR_FONT_CONFIG_INFO_PARSE_FILE: OH_Drawing_FontConfigInfoErrorCode =
        OH_Drawing_FontConfigInfoErrorCode(2);
}
impl OH_Drawing_FontConfigInfoErrorCode {
    pub const ERROR_FONT_CONFIG_INFO_ALLOC_MEMORY: OH_Drawing_FontConfigInfoErrorCode =
        OH_Drawing_FontConfigInfoErrorCode(3);
}
impl OH_Drawing_FontConfigInfoErrorCode {
    pub const ERROR_FONT_CONFIG_INFO_COPY_STRING_DATA: OH_Drawing_FontConfigInfoErrorCode =
        OH_Drawing_FontConfigInfoErrorCode(4);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct OH_Drawing_FontConfigInfoErrorCode(pub ::core::ffi::c_uint);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OH_Drawing_FontFallbackInfo {
    pub language: *mut ::core::ffi::c_char,
    pub familyName: *mut ::core::ffi::c_char,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OH_Drawing_FontFallbackGroup {
    pub groupName: *mut ::core::ffi::c_char,
    pub fallbackInfoSize: usize,
    pub fallbackInfoSet: *mut OH_Drawing_FontFallbackInfo,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OH_Drawing_FontAdjustInfo {
    pub weight: ::core::ffi::c_int,
    pub to: ::core::ffi::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OH_Drawing_FontAliasInfo {
    pub familyName: *mut ::core::ffi::c_char,
    pub weight: ::core::ffi::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OH_Drawing_FontGenericInfo {
    pub familyName: *mut ::core::ffi::c_char,
    pub aliasInfoSize: usize,
    pub adjustInfoSize: usize,
    pub aliasInfoSet: *mut OH_Drawing_FontAliasInfo,
    pub adjustInfoSet: *mut OH_Drawing_FontAdjustInfo,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OH_Drawing_FontConfigInfo {
    pub fontDirSize: usize,
    pub fontGenericInfoSize: usize,
    pub fallbackGroupSize: usize,
    pub fontDirSet: *mut *mut ::core::ffi::c_char,
    pub fontGenericInfoSet: *mut OH_Drawing_FontGenericInfo,
    pub fallbackGroupSet: *mut OH_Drawing_FontFallbackGroup,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OH_Drawing_FontDescriptor {
    pub path: *mut ::core::ffi::c_char,
    pub postScriptName: *mut ::core::ffi::c_char,
    pub fullName: *mut ::core::ffi::c_char,
    pub fontFamily: *mut ::core::ffi::c_char,
    pub fontSubfamily: *mut ::core::ffi::c_char,
    pub weight: ::core::ffi::c_int,
    pub width: ::core::ffi::c_int,
    pub italic: ::core::ffi::c_int,
    pub monoSpace: bool,
    pub symbolic: bool,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OH_Drawing_LineMetrics {
    pub ascender: f64,
    pub descender: f64,
    pub capHeight: f64,
    pub xHeight: f64,
    pub width: f64,
    pub height: f64,
    pub x: f64,
    pub y: f64,
    pub startIndex: usize,
    pub endIndex: usize,
    pub firstCharMetrics: OH_Drawing_Font_Metrics,
}
impl OH_Drawing_TextHeightBehavior {
    pub const TEXT_HEIGHT_ALL: OH_Drawing_TextHeightBehavior = OH_Drawing_TextHeightBehavior(0);
}
impl OH_Drawing_TextHeightBehavior {
    pub const TEXT_HEIGHT_DISABLE_FIRST_ASCENT: OH_Drawing_TextHeightBehavior =
        OH_Drawing_TextHeightBehavior(1);
}
impl OH_Drawing_TextHeightBehavior {
    pub const TEXT_HEIGHT_DISABLE_LAST_ASCENT: OH_Drawing_TextHeightBehavior =
        OH_Drawing_TextHeightBehavior(2);
}
impl OH_Drawing_TextHeightBehavior {
    pub const TEXT_HEIGHT_DISABLE_ALL: OH_Drawing_TextHeightBehavior =
        OH_Drawing_TextHeightBehavior(3);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct OH_Drawing_TextHeightBehavior(pub ::core::ffi::c_uint);
impl OH_Drawing_TextStyleType {
    pub const TEXT_STYLE_NONE: OH_Drawing_TextStyleType = OH_Drawing_TextStyleType(0);
}
impl OH_Drawing_TextStyleType {
    pub const TEXT_STYLE_ALL_ATTRIBUTES: OH_Drawing_TextStyleType = OH_Drawing_TextStyleType(1);
}
impl OH_Drawing_TextStyleType {
    pub const TEXT_STYLE_FONT: OH_Drawing_TextStyleType = OH_Drawing_TextStyleType(2);
}
impl OH_Drawing_TextStyleType {
    pub const TEXT_STYLE_FOREGROUND: OH_Drawing_TextStyleType = OH_Drawing_TextStyleType(3);
}
impl OH_Drawing_TextStyleType {
    pub const TEXT_STYLE_BACKGROUND: OH_Drawing_TextStyleType = OH_Drawing_TextStyleType(4);
}
impl OH_Drawing_TextStyleType {
    pub const TEXT_STYLE_SHADOW: OH_Drawing_TextStyleType = OH_Drawing_TextStyleType(5);
}
impl OH_Drawing_TextStyleType {
    pub const TEXT_STYLE_DECORATIONS: OH_Drawing_TextStyleType = OH_Drawing_TextStyleType(6);
}
impl OH_Drawing_TextStyleType {
    pub const TEXT_STYLE_LETTER_SPACING: OH_Drawing_TextStyleType = OH_Drawing_TextStyleType(7);
}
impl OH_Drawing_TextStyleType {
    pub const TEXT_STYLE_WORD_SPACING: OH_Drawing_TextStyleType = OH_Drawing_TextStyleType(8);
}
#[repr(transparent)]
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
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct OH_Drawing_FontWidth(pub ::core::ffi::c_uint);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OH_Drawing_FontStyleStruct {
    pub weight: OH_Drawing_FontWeight,
    pub width: OH_Drawing_FontWidth,
    pub slant: OH_Drawing_FontStyle,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OH_Drawing_FontFeature {
    pub tag: *mut ::core::ffi::c_char,
    pub value: ::core::ffi::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OH_Drawing_StrutStyle {
    pub weight: OH_Drawing_FontWeight,
    pub style: OH_Drawing_FontStyle,
    pub size: f64,
    pub heightScale: f64,
    pub heightOverride: bool,
    pub halfLeading: bool,
    pub leading: f64,
    pub forceStrutHeight: bool,
    pub familiesSize: usize,
    pub families: *mut *mut ::core::ffi::c_char,
}

extern "C" {
    pub fn OH_Drawing_SetTextStyleForegroundBrush(
        arg1: *mut OH_Drawing_TextStyle,
        arg2: *mut OH_Drawing_Brush,
    );
}
extern "C" {
    pub fn OH_Drawing_TextStyleGetForegroundBrush(
        arg1: *mut OH_Drawing_TextStyle,
        arg2: *mut OH_Drawing_Brush,
    );
}
extern "C" {
    pub fn OH_Drawing_SetTextStyleForegroundPen(
        arg1: *mut OH_Drawing_TextStyle,
        arg2: *mut OH_Drawing_Pen,
    );
}
extern "C" {
    pub fn OH_Drawing_TextStyleGetForegroundPen(
        arg1: *mut OH_Drawing_TextStyle,
        arg2: *mut OH_Drawing_Pen,
    );
}
extern "C" {
    pub fn OH_Drawing_SetTextStyleBackgroundBrush(
        arg1: *mut OH_Drawing_TextStyle,
        arg2: *mut OH_Drawing_Brush,
    );
}
extern "C" {
    pub fn OH_Drawing_TextStyleGetBackgroundBrush(
        arg1: *mut OH_Drawing_TextStyle,
        arg2: *mut OH_Drawing_Brush,
    );
}
extern "C" {
    pub fn OH_Drawing_SetTextStyleBackgroundPen(
        arg1: *mut OH_Drawing_TextStyle,
        arg2: *mut OH_Drawing_Pen,
    );
}
extern "C" {
    pub fn OH_Drawing_TextStyleGetBackgroundPen(
        arg1: *mut OH_Drawing_TextStyle,
        arg2: *mut OH_Drawing_Pen,
    );
}

extern "C" {
    pub fn OH_Drawing_TypographyGetLineTextRange(
        arg1: *mut OH_Drawing_Typography,
        arg2: ::core::ffi::c_int,
        arg3: bool,
    ) -> *mut OH_Drawing_Range;
}
extern "C" {
    pub fn OH_Drawing_CreateFontDescriptor() -> *mut OH_Drawing_FontDescriptor;
}
extern "C" {
    pub fn OH_Drawing_DestroyFontDescriptor(arg1: *mut OH_Drawing_FontDescriptor);
}
extern "C" {
    pub fn OH_Drawing_CreateFontParser() -> *mut OH_Drawing_FontParser;
}
extern "C" {
    pub fn OH_Drawing_DestroyFontParser(arg1: *mut OH_Drawing_FontParser);
}
extern "C" {
    pub fn OH_Drawing_FontParserGetSystemFontList(
        arg1: *mut OH_Drawing_FontParser,
        arg2: *mut usize,
    ) -> *mut *mut ::core::ffi::c_char;
}
extern "C" {
    pub fn OH_Drawing_DestroySystemFontList(arg1: *mut *mut ::core::ffi::c_char, arg2: usize);
}
extern "C" {
    pub fn OH_Drawing_FontParserGetFontByName(
        arg1: *mut OH_Drawing_FontParser,
        arg2: *const ::core::ffi::c_char,
    ) -> *mut OH_Drawing_FontDescriptor;
}
extern "C" {
    pub fn OH_Drawing_TypographyGetLineMetrics(
        arg1: *mut OH_Drawing_Typography,
    ) -> *mut OH_Drawing_LineMetrics;
}
extern "C" {
    pub fn OH_Drawing_LineMetricsGetSize(arg1: *mut OH_Drawing_LineMetrics) -> usize;
}
extern "C" {
    pub fn OH_Drawing_DestroyLineMetrics(arg1: *mut OH_Drawing_LineMetrics);
}
extern "C" {
    pub fn OH_Drawing_TypographyGetLineMetricsAt(
        arg1: *mut OH_Drawing_Typography,
        arg2: ::core::ffi::c_int,
        arg3: *mut OH_Drawing_LineMetrics,
    ) -> bool;
}
extern "C" {
    pub fn OH_Drawing_SetTypographyTextEllipsis(
        arg1: *mut OH_Drawing_TypographyStyle,
        arg2: *const ::core::ffi::c_char,
    );
}
extern "C" {
    pub fn OH_Drawing_SetTypographyTextLocale(
        arg1: *mut OH_Drawing_TypographyStyle,
        arg2: *const ::core::ffi::c_char,
    );
}
extern "C" {
    pub fn OH_Drawing_SetTypographyTextSplitRatio(arg1: *mut OH_Drawing_TypographyStyle, arg2: f32);
}
extern "C" {
    pub fn OH_Drawing_TypographyGetTextStyle(
        arg1: *mut OH_Drawing_TypographyStyle,
    ) -> *mut OH_Drawing_TextStyle;
}
extern "C" {
    pub fn OH_Drawing_TypographyGetEffectiveAlignment(
        arg1: *mut OH_Drawing_TypographyStyle,
    ) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn OH_Drawing_TypographyIsLineUnlimited(arg1: *mut OH_Drawing_TypographyStyle) -> bool;
}
extern "C" {
    pub fn OH_Drawing_TypographyIsEllipsized(arg1: *mut OH_Drawing_TypographyStyle) -> bool;
}
extern "C" {
    pub fn OH_Drawing_SetTypographyTextStyle(
        arg1: *mut OH_Drawing_TypographyStyle,
        arg2: *mut OH_Drawing_TextStyle,
    );
}
extern "C" {
    pub fn OH_Drawing_TextStyleGetFontMetrics(
        arg1: *mut OH_Drawing_Typography,
        arg2: *mut OH_Drawing_TextStyle,
        arg3: *mut OH_Drawing_Font_Metrics,
    ) -> bool;
}
extern "C" {
    pub fn OH_Drawing_TypographyGetLineInfo(
        arg1: *mut OH_Drawing_Typography,
        arg2: ::core::ffi::c_int,
        arg3: bool,
        arg4: bool,
        arg5: *mut OH_Drawing_LineMetrics,
    ) -> bool;
}
extern "C" {
    pub fn OH_Drawing_SetTypographyTextFontWeight(
        arg1: *mut OH_Drawing_TypographyStyle,
        arg2: ::core::ffi::c_int,
    );
}
extern "C" {
    pub fn OH_Drawing_SetTypographyTextFontStyle(
        arg1: *mut OH_Drawing_TypographyStyle,
        arg2: ::core::ffi::c_int,
    );
}
extern "C" {
    pub fn OH_Drawing_SetTypographyTextFontFamily(
        arg1: *mut OH_Drawing_TypographyStyle,
        arg2: *const ::core::ffi::c_char,
    );
}
extern "C" {
    pub fn OH_Drawing_SetTypographyTextFontSize(arg1: *mut OH_Drawing_TypographyStyle, arg2: f64);
}
extern "C" {
    pub fn OH_Drawing_SetTypographyTextFontHeight(arg1: *mut OH_Drawing_TypographyStyle, arg2: f64);
}
extern "C" {
    pub fn OH_Drawing_SetTypographyTextHalfLeading(
        arg1: *mut OH_Drawing_TypographyStyle,
        arg2: bool,
    );
}
extern "C" {
    pub fn OH_Drawing_SetTypographyTextUseLineStyle(
        arg1: *mut OH_Drawing_TypographyStyle,
        arg2: bool,
    );
}
extern "C" {
    pub fn OH_Drawing_SetTypographyTextLineStyleFontWeight(
        arg1: *mut OH_Drawing_TypographyStyle,
        arg2: ::core::ffi::c_int,
    );
}
extern "C" {
    pub fn OH_Drawing_SetTypographyTextLineStyleFontStyle(
        arg1: *mut OH_Drawing_TypographyStyle,
        arg2: ::core::ffi::c_int,
    );
}
extern "C" {
    pub fn OH_Drawing_SetTypographyTextLineStyleFontFamilies(
        arg1: *mut OH_Drawing_TypographyStyle,
        arg2: ::core::ffi::c_int,
        fontFamilies: *mut *const ::core::ffi::c_char,
    );
}
extern "C" {
    pub fn OH_Drawing_SetTypographyTextLineStyleFontSize(
        arg1: *mut OH_Drawing_TypographyStyle,
        arg2: f64,
    );
}
extern "C" {
    pub fn OH_Drawing_SetTypographyTextLineStyleFontHeight(
        arg1: *mut OH_Drawing_TypographyStyle,
        arg2: f64,
    );
}
extern "C" {
    pub fn OH_Drawing_SetTypographyTextLineStyleHalfLeading(
        arg1: *mut OH_Drawing_TypographyStyle,
        arg2: bool,
    );
}
extern "C" {
    pub fn OH_Drawing_SetTypographyTextLineStyleSpacingScale(
        arg1: *mut OH_Drawing_TypographyStyle,
        arg2: f64,
    );
}
extern "C" {
    pub fn OH_Drawing_SetTypographyTextLineStyleOnly(
        arg1: *mut OH_Drawing_TypographyStyle,
        arg2: bool,
    );
}
extern "C" {
    pub fn OH_Drawing_CreateTextShadow() -> *mut OH_Drawing_TextShadow;
}
extern "C" {
    pub fn OH_Drawing_DestroyTextShadow(arg1: *mut OH_Drawing_TextShadow);
}
extern "C" {
    pub fn OH_Drawing_TextStyleGetShadows(
        arg1: *mut OH_Drawing_TextStyle,
    ) -> *mut OH_Drawing_TextShadow;
}
extern "C" {
    pub fn OH_Drawing_TextStyleGetShadowCount(
        arg1: *mut OH_Drawing_TextStyle,
    ) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn OH_Drawing_TextStyleAddShadow(
        arg1: *mut OH_Drawing_TextStyle,
        arg2: *const OH_Drawing_TextShadow,
    );
}
extern "C" {
    pub fn OH_Drawing_TextStyleClearShadows(arg1: *mut OH_Drawing_TextStyle);
}
extern "C" {
    pub fn OH_Drawing_TextStyleGetShadowWithIndex(
        arg1: *mut OH_Drawing_TextStyle,
        arg2: ::core::ffi::c_int,
    ) -> *mut OH_Drawing_TextShadow;
}
extern "C" {
    pub fn OH_Drawing_TypographySetIndents(
        arg1: *mut OH_Drawing_Typography,
        arg2: ::core::ffi::c_int,
        indents: *const f32,
    );
}
extern "C" {
    pub fn OH_Drawing_TypographyGetIndentsWithIndex(
        arg1: *mut OH_Drawing_Typography,
        arg2: ::core::ffi::c_int,
    ) -> f32;
}
extern "C" {
    pub fn OH_Drawing_DestroyTextShadows(arg1: *mut OH_Drawing_TextShadow);
}
extern "C" {
    pub fn OH_Drawing_TypographyTextSetHeightBehavior(
        arg1: *mut OH_Drawing_TypographyStyle,
        heightMode: OH_Drawing_TextHeightBehavior,
    );
}
extern "C" {
    pub fn OH_Drawing_TypographyTextGetHeightBehavior(
        arg1: *mut OH_Drawing_TypographyStyle,
    ) -> OH_Drawing_TextHeightBehavior;
}
extern "C" {
    pub fn OH_Drawing_TextStyleSetBackgroundRect(
        arg1: *mut OH_Drawing_TextStyle,
        arg2: *const OH_Drawing_RectStyle_Info,
        styleId: ::core::ffi::c_int,
    );
}
extern "C" {
    pub fn OH_Drawing_TypographyHandlerAddSymbol(
        arg1: *mut OH_Drawing_TypographyCreate,
        symbol: u32,
    );
}
extern "C" {
    pub fn OH_Drawing_TextStyleAddFontFeature(
        arg1: *mut OH_Drawing_TextStyle,
        tag: *const ::core::ffi::c_char,
        value: ::core::ffi::c_int,
    );
}
extern "C" {
    pub fn OH_Drawing_TextStyleGetFontFeatures(
        arg1: *mut OH_Drawing_TextStyle,
    ) -> *mut OH_Drawing_FontFeature;
}
extern "C" {
    pub fn OH_Drawing_TextStyleDestroyFontFeatures(
        arg1: *mut OH_Drawing_FontFeature,
        fontFeatureSize: usize,
    );
}
extern "C" {
    pub fn OH_Drawing_TextStyleGetFontFeatureSize(arg1: *mut OH_Drawing_TextStyle) -> usize;
}
extern "C" {
    pub fn OH_Drawing_TextStyleClearFontFeature(arg1: *mut OH_Drawing_TextStyle);
}
extern "C" {
    pub fn OH_Drawing_TextStyleSetBaselineShift(arg1: *mut OH_Drawing_TextStyle, lineShift: f64);
}
extern "C" {
    pub fn OH_Drawing_TextStyleGetBaselineShift(arg1: *mut OH_Drawing_TextStyle) -> f64;
}
extern "C" {
    pub fn OH_Drawing_TextStyleGetColor(arg1: *mut OH_Drawing_TextStyle) -> u32;
}
extern "C" {
    pub fn OH_Drawing_TextStyleGetDecorationStyle(
        arg1: *mut OH_Drawing_TextStyle,
    ) -> OH_Drawing_TextDecorationStyle;
}
extern "C" {
    pub fn OH_Drawing_TextStyleGetFontWeight(
        arg1: *mut OH_Drawing_TextStyle,
    ) -> OH_Drawing_FontWeight;
}
extern "C" {
    pub fn OH_Drawing_TextStyleGetFontStyle(
        arg1: *mut OH_Drawing_TextStyle,
    ) -> OH_Drawing_FontStyle;
}
extern "C" {
    pub fn OH_Drawing_TextStyleGetBaseline(
        arg1: *mut OH_Drawing_TextStyle,
    ) -> OH_Drawing_TextBaseline;
}
extern "C" {
    pub fn OH_Drawing_TextStyleGetFontFamilies(
        arg1: *mut OH_Drawing_TextStyle,
        num: *mut usize,
    ) -> *mut *mut ::core::ffi::c_char;
}
extern "C" {
    pub fn OH_Drawing_TextStyleDestroyFontFamilies(
        fontFamilies: *mut *mut ::core::ffi::c_char,
        num: usize,
    );
}
extern "C" {
    pub fn OH_Drawing_TextStyleGetFontSize(arg1: *mut OH_Drawing_TextStyle) -> f64;
}
extern "C" {
    pub fn OH_Drawing_TextStyleGetLetterSpacing(arg1: *mut OH_Drawing_TextStyle) -> f64;
}
extern "C" {
    pub fn OH_Drawing_TextStyleGetWordSpacing(arg1: *mut OH_Drawing_TextStyle) -> f64;
}
extern "C" {
    pub fn OH_Drawing_TextStyleGetFontHeight(arg1: *mut OH_Drawing_TextStyle) -> f64;
}
extern "C" {
    pub fn OH_Drawing_TextStyleGetHalfLeading(arg1: *mut OH_Drawing_TextStyle) -> bool;
}
extern "C" {
    pub fn OH_Drawing_TextStyleGetLocale(
        arg1: *mut OH_Drawing_TextStyle,
    ) -> *const ::core::ffi::c_char;
}
extern "C" {
    pub fn OH_Drawing_SetTextStyleFontStyleStruct(
        drawingTextStyle: *mut OH_Drawing_TextStyle,
        fontStyle: OH_Drawing_FontStyleStruct,
    );
}
extern "C" {
    pub fn OH_Drawing_TextStyleGetFontStyleStruct(
        drawingTextStyle: *mut OH_Drawing_TextStyle,
    ) -> OH_Drawing_FontStyleStruct;
}
extern "C" {
    pub fn OH_Drawing_SetTypographyStyleFontStyleStruct(
        drawingStyle: *mut OH_Drawing_TypographyStyle,
        fontStyle: OH_Drawing_FontStyleStruct,
    );
}
extern "C" {
    pub fn OH_Drawing_TypographyStyleGetFontStyleStruct(
        drawingStyle: *mut OH_Drawing_TypographyStyle,
    ) -> OH_Drawing_FontStyleStruct;
}
extern "C" {
    pub fn OH_Drawing_TextStyleIsEqual(
        style: *const OH_Drawing_TextStyle,
        comparedStyle: *const OH_Drawing_TextStyle,
    ) -> bool;
}
extern "C" {
    pub fn OH_Drawing_TextStyleIsEqualByFont(
        style: *const OH_Drawing_TextStyle,
        comparedStyle: *const OH_Drawing_TextStyle,
    ) -> bool;
}
extern "C" {
    pub fn OH_Drawing_TextStyleIsAttributeMatched(
        style: *const OH_Drawing_TextStyle,
        comparedStyle: *const OH_Drawing_TextStyle,
        textStyleType: OH_Drawing_TextStyleType,
    ) -> bool;
}
extern "C" {
    pub fn OH_Drawing_TextStyleSetPlaceholder(style: *mut OH_Drawing_TextStyle);
}
extern "C" {
    pub fn OH_Drawing_TextStyleIsPlaceholder(style: *mut OH_Drawing_TextStyle) -> bool;
}
extern "C" {
    pub fn OH_Drawing_TypographyStyleGetEffectiveAlignment(
        style: *mut OH_Drawing_TypographyStyle,
    ) -> OH_Drawing_TextAlign;
}
extern "C" {
    pub fn OH_Drawing_TypographyStyleIsHintEnabled(style: *mut OH_Drawing_TypographyStyle) -> bool;
}
extern "C" {
    pub fn OH_Drawing_GetSystemFontConfigInfo(
        arg1: *mut OH_Drawing_FontConfigInfoErrorCode,
    ) -> *mut OH_Drawing_FontConfigInfo;
}
extern "C" {
    pub fn OH_Drawing_DestroySystemFontConfigInfo(arg1: *mut OH_Drawing_FontConfigInfo);
}
extern "C" {
    pub fn OH_Drawing_SetTypographyStyleTextStrutStyle(
        arg1: *mut OH_Drawing_TypographyStyle,
        arg2: *mut OH_Drawing_StrutStyle,
    );
}
extern "C" {
    pub fn OH_Drawing_TypographyStyleDestroyStrutStyle(arg1: *mut OH_Drawing_StrutStyle);
}
extern "C" {
    pub fn OH_Drawing_TypographyStyleGetStrutStyle(
        arg1: *mut OH_Drawing_TypographyStyle,
    ) -> *mut OH_Drawing_StrutStyle;
}
extern "C" {
    pub fn OH_Drawing_TypographyStyleStrutStyleEquals(
        from: *mut OH_Drawing_StrutStyle,
        to: *mut OH_Drawing_StrutStyle,
    ) -> bool;
}
extern "C" {
    pub fn OH_Drawing_TypographyStyleSetHintsEnabled(
        style: *mut OH_Drawing_TypographyStyle,
        hintsEnabled: bool,
    );
}
extern "C" {
    pub fn OH_Drawing_TypographyGetLineFontMetrics(
        arg1: *mut OH_Drawing_Typography,
        lineNumber: usize,
        fontMetricsSize: *mut usize,
    ) -> *mut OH_Drawing_Font_Metrics;
}
extern "C" {
    pub fn OH_Drawing_TypographyDestroyLineFontMetrics(arg1: *mut OH_Drawing_Font_Metrics);
}
extern "C" {
    pub fn OH_Drawing_TypographyMarkDirty(arg1: *mut OH_Drawing_Typography);
}
extern "C" {
    pub fn OH_Drawing_TypographyGetUnresolvedGlyphsCount(arg1: *mut OH_Drawing_Typography) -> i32;
}
extern "C" {
    pub fn OH_Drawing_TypographyUpdateFontSize(
        arg1: *mut OH_Drawing_Typography,
        from: usize,
        to: usize,
        fontSize: f32,
    );
}
extern "C" {
    pub fn OH_Drawing_TypographyTextGetLineStyle(arg1: *mut OH_Drawing_TypographyStyle) -> bool;
}
extern "C" {
    pub fn OH_Drawing_TypographyTextlineStyleGetFontWeight(
        arg1: *mut OH_Drawing_TypographyStyle,
    ) -> OH_Drawing_FontWeight;
}
extern "C" {
    pub fn OH_Drawing_TypographyTextlineStyleGetFontStyle(
        arg1: *mut OH_Drawing_TypographyStyle,
    ) -> OH_Drawing_FontStyle;
}
extern "C" {
    pub fn OH_Drawing_TypographyTextlineStyleGetFontFamilies(
        arg1: *mut OH_Drawing_TypographyStyle,
        num: *mut usize,
    ) -> *mut *mut ::core::ffi::c_char;
}
extern "C" {
    pub fn OH_Drawing_TypographyTextlineStyleDestroyFontFamilies(
        fontFamilies: *mut *mut ::core::ffi::c_char,
        fontFamiliesNum: usize,
    );
}
extern "C" {
    pub fn OH_Drawing_TypographyTextlineStyleGetFontSize(
        arg1: *mut OH_Drawing_TypographyStyle,
    ) -> f64;
}
extern "C" {
    pub fn OH_Drawing_TypographyTextlineStyleGetHeightScale(
        arg1: *mut OH_Drawing_TypographyStyle,
    ) -> f64;
}
extern "C" {
    pub fn OH_Drawing_TypographyTextlineStyleGetHeightOnly(
        arg1: *mut OH_Drawing_TypographyStyle,
    ) -> bool;
}
extern "C" {
    pub fn OH_Drawing_TypographyTextlineStyleGetHalfLeading(
        arg1: *mut OH_Drawing_TypographyStyle,
    ) -> bool;
}
extern "C" {
    pub fn OH_Drawing_TypographyTextlineStyleGetSpacingScale(
        arg1: *mut OH_Drawing_TypographyStyle,
    ) -> f64;
}
extern "C" {
    pub fn OH_Drawing_TypographyTextlineGetStyleOnly(arg1: *mut OH_Drawing_TypographyStyle)
        -> bool;
}
extern "C" {
    pub fn OH_Drawing_TypographyGetTextAlign(
        arg1: *mut OH_Drawing_TypographyStyle,
    ) -> OH_Drawing_TextAlign;
}
extern "C" {
    pub fn OH_Drawing_TypographyGetTextDirection(
        arg1: *mut OH_Drawing_TypographyStyle,
    ) -> OH_Drawing_TextDirection;
}
extern "C" {
    pub fn OH_Drawing_TypographyGetTextMaxLines(arg1: *mut OH_Drawing_TypographyStyle) -> usize;
}
extern "C" {
    pub fn OH_Drawing_TypographyGetTextEllipsis(
        arg1: *mut OH_Drawing_TypographyStyle,
    ) -> *mut ::core::ffi::c_char;
}
extern "C" {
    pub fn OH_Drawing_TypographyDestroyEllipsis(ellipsis: *mut ::core::ffi::c_char);
}
extern "C" {
    pub fn OH_Drawing_TypographyStyleEquals(
        from: *mut OH_Drawing_TypographyStyle,
        to: *mut OH_Drawing_TypographyStyle,
    ) -> bool;
}
extern "C" {
    pub fn OH_Drawing_TypographyDestroyTextBox(arg1: *mut OH_Drawing_TextBox);
}
extern "C" {
    pub fn OH_Drawing_SetTextShadow(
        shadow: *mut OH_Drawing_TextShadow,
        color: u32,
        offset: *mut OH_Drawing_Point,
        blurRadius: f64,
    );
}

extern "C" {
    pub fn OH_Drawing_FilterSetImageFilter(
        arg1: *mut OH_Drawing_Filter,
        arg2: *mut OH_Drawing_ImageFilter,
    );
}

extern "C" {
    pub fn OH_Drawing_FilterGetColorFilter(
        arg1: *mut OH_Drawing_Filter,
        arg2: *mut OH_Drawing_ColorFilter,
    );
}

extern "C" {
    pub fn OH_Drawing_MatrixCreateRotation(deg: f32, x: f32, y: f32) -> *mut OH_Drawing_Matrix;
}
extern "C" {
    pub fn OH_Drawing_MatrixCreateScale(
        sx: f32,
        sy: f32,
        px: f32,
        py: f32,
    ) -> *mut OH_Drawing_Matrix;
}
extern "C" {
    pub fn OH_Drawing_MatrixCreateTranslation(dx: f32, dy: f32) -> *mut OH_Drawing_Matrix;
}

impl OH_Drawing_ScaleToFit {
    pub const SCALE_TO_FIT_FILL: OH_Drawing_ScaleToFit = OH_Drawing_ScaleToFit(0);
}
impl OH_Drawing_ScaleToFit {
    pub const SCALE_TO_FIT_START: OH_Drawing_ScaleToFit = OH_Drawing_ScaleToFit(1);
}
impl OH_Drawing_ScaleToFit {
    pub const SCALE_TO_FIT_CENTER: OH_Drawing_ScaleToFit = OH_Drawing_ScaleToFit(2);
}
impl OH_Drawing_ScaleToFit {
    pub const SCALE_TO_FIT_END: OH_Drawing_ScaleToFit = OH_Drawing_ScaleToFit(3);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct OH_Drawing_ScaleToFit(pub ::core::ffi::c_uint);
extern "C" {
    pub fn OH_Drawing_MatrixSetRectToRect(
        arg1: *mut OH_Drawing_Matrix,
        src: *const OH_Drawing_Rect,
        dst: *const OH_Drawing_Rect,
        stf: OH_Drawing_ScaleToFit,
    ) -> bool;
}
extern "C" {
    pub fn OH_Drawing_MatrixPreRotate(arg1: *mut OH_Drawing_Matrix, degree: f32, px: f32, py: f32);
}
extern "C" {
    pub fn OH_Drawing_MatrixPreScale(
        arg1: *mut OH_Drawing_Matrix,
        sx: f32,
        sy: f32,
        px: f32,
        py: f32,
    );
}
extern "C" {
    pub fn OH_Drawing_MatrixPreTranslate(arg1: *mut OH_Drawing_Matrix, dx: f32, dy: f32);
}
extern "C" {
    pub fn OH_Drawing_MatrixPostRotate(arg1: *mut OH_Drawing_Matrix, degree: f32, px: f32, py: f32);
}
extern "C" {
    pub fn OH_Drawing_MatrixPostScale(
        arg1: *mut OH_Drawing_Matrix,
        sx: f32,
        sy: f32,
        px: f32,
        py: f32,
    );
}
extern "C" {
    pub fn OH_Drawing_MatrixPostTranslate(arg1: *mut OH_Drawing_Matrix, dx: f32, dy: f32);
}
extern "C" {
    pub fn OH_Drawing_MatrixReset(arg1: *mut OH_Drawing_Matrix);
}
extern "C" {
    pub fn OH_Drawing_MatrixConcat(
        total: *mut OH_Drawing_Matrix,
        a: *const OH_Drawing_Matrix,
        b: *const OH_Drawing_Matrix,
    );
}
extern "C" {
    pub fn OH_Drawing_MatrixGetValue(
        arg1: *mut OH_Drawing_Matrix,
        index: ::core::ffi::c_int,
    ) -> f32;
}
extern "C" {
    pub fn OH_Drawing_MatrixRotate(arg1: *mut OH_Drawing_Matrix, degree: f32, px: f32, py: f32);
}
extern "C" {
    pub fn OH_Drawing_MatrixTranslate(arg1: *mut OH_Drawing_Matrix, dx: f32, dy: f32);
}
extern "C" {
    pub fn OH_Drawing_MatrixScale(arg1: *mut OH_Drawing_Matrix, sx: f32, sy: f32, px: f32, py: f32);
}
extern "C" {
    pub fn OH_Drawing_MatrixInvert(
        arg1: *mut OH_Drawing_Matrix,
        inverse: *mut OH_Drawing_Matrix,
    ) -> bool;
}
extern "C" {
    pub fn OH_Drawing_MatrixSetPolyToPoly(
        arg1: *mut OH_Drawing_Matrix,
        src: *const OH_Drawing_Point2D,
        dst: *const OH_Drawing_Point2D,
        count: u32,
    ) -> bool;
}
extern "C" {
    pub fn OH_Drawing_MatrixMapPoints(
        arg1: *const OH_Drawing_Matrix,
        src: *const OH_Drawing_Point2D,
        dst: *mut OH_Drawing_Point2D,
        count: ::core::ffi::c_int,
    );
}
extern "C" {
    pub fn OH_Drawing_MatrixMapRect(
        arg1: *const OH_Drawing_Matrix,
        src: *const OH_Drawing_Rect,
        dst: *mut OH_Drawing_Rect,
    ) -> bool;
}
extern "C" {
    pub fn OH_Drawing_MatrixIsEqual(
        arg1: *mut OH_Drawing_Matrix,
        other: *mut OH_Drawing_Matrix,
    ) -> bool;
}
extern "C" {
    pub fn OH_Drawing_MatrixIsIdentity(arg1: *mut OH_Drawing_Matrix) -> bool;
}

extern "C" {
    pub fn OH_Drawing_RectIntersect(
        rect: *mut OH_Drawing_Rect,
        other: *const OH_Drawing_Rect,
    ) -> bool;
}
extern "C" {
    pub fn OH_Drawing_RectJoin(rect: *mut OH_Drawing_Rect, other: *const OH_Drawing_Rect) -> bool;
}
extern "C" {
    pub fn OH_Drawing_RectSetLeft(rect: *mut OH_Drawing_Rect, left: f32);
}
extern "C" {
    pub fn OH_Drawing_RectSetTop(rect: *mut OH_Drawing_Rect, top: f32);
}
extern "C" {
    pub fn OH_Drawing_RectSetRight(rect: *mut OH_Drawing_Rect, right: f32);
}
extern "C" {
    pub fn OH_Drawing_RectSetBottom(rect: *mut OH_Drawing_Rect, bottom: f32);
}
extern "C" {
    pub fn OH_Drawing_RectGetLeft(arg1: *mut OH_Drawing_Rect) -> f32;
}
extern "C" {
    pub fn OH_Drawing_RectGetTop(arg1: *mut OH_Drawing_Rect) -> f32;
}
extern "C" {
    pub fn OH_Drawing_RectGetRight(arg1: *mut OH_Drawing_Rect) -> f32;
}
extern "C" {
    pub fn OH_Drawing_RectGetBottom(arg1: *mut OH_Drawing_Rect) -> f32;
}
extern "C" {
    pub fn OH_Drawing_RectGetHeight(arg1: *mut OH_Drawing_Rect) -> f32;
}
extern "C" {
    pub fn OH_Drawing_RectGetWidth(arg1: *mut OH_Drawing_Rect) -> f32;
}
extern "C" {
    pub fn OH_Drawing_RectCopy(src: *mut OH_Drawing_Rect, dst: *mut OH_Drawing_Rect);
}

impl OH_Drawing_CornerPos {
    pub const CORNER_POS_TOP_LEFT: OH_Drawing_CornerPos = OH_Drawing_CornerPos(0);
}
impl OH_Drawing_CornerPos {
    pub const CORNER_POS_TOP_RIGHT: OH_Drawing_CornerPos = OH_Drawing_CornerPos(1);
}
impl OH_Drawing_CornerPos {
    pub const CORNER_POS_BOTTOM_RIGHT: OH_Drawing_CornerPos = OH_Drawing_CornerPos(2);
}
impl OH_Drawing_CornerPos {
    pub const CORNER_POS_BOTTOM_LEFT: OH_Drawing_CornerPos = OH_Drawing_CornerPos(3);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct OH_Drawing_CornerPos(pub ::core::ffi::c_uint);

extern "C" {
    pub fn OH_Drawing_RoundRectSetCorner(
        arg1: *mut OH_Drawing_RoundRect,
        pos: OH_Drawing_CornerPos,
        arg2: OH_Drawing_Corner_Radii,
    );
}
extern "C" {
    pub fn OH_Drawing_RoundRectGetCorner(
        arg1: *mut OH_Drawing_RoundRect,
        pos: OH_Drawing_CornerPos,
    ) -> OH_Drawing_Corner_Radii;
}

extern "C" {
    pub fn OH_Drawing_ShaderEffectCreateColorShader(color: u32) -> *mut OH_Drawing_ShaderEffect;
}

extern "C" {
    pub fn OH_Drawing_ShaderEffectCreateLinearGradientWithLocalMatrix(
        startPt: *const OH_Drawing_Point2D,
        endPt: *const OH_Drawing_Point2D,
        colors: *const u32,
        pos: *const f32,
        size: u32,
        arg1: OH_Drawing_TileMode,
        arg2: *const OH_Drawing_Matrix,
    ) -> *mut OH_Drawing_ShaderEffect;
}

extern "C" {
    pub fn OH_Drawing_ShaderEffectCreateRadialGradientWithLocalMatrix(
        centerPt: *const OH_Drawing_Point2D,
        radius: f32,
        colors: *const u32,
        pos: *const f32,
        size: u32,
        arg1: OH_Drawing_TileMode,
        arg2: *const OH_Drawing_Matrix,
    ) -> *mut OH_Drawing_ShaderEffect;
}

extern "C" {
    pub fn OH_Drawing_ShaderEffectCreateImageShader(
        arg1: *mut OH_Drawing_Image,
        tileX: OH_Drawing_TileMode,
        tileY: OH_Drawing_TileMode,
        arg2: *const OH_Drawing_SamplingOptions,
        arg3: *const OH_Drawing_Matrix,
    ) -> *mut OH_Drawing_ShaderEffect;
}
extern "C" {
    pub fn OH_Drawing_ShaderEffectCreateTwoPointConicalGradient(
        startPt: *const OH_Drawing_Point2D,
        startRadius: f32,
        endPt: *const OH_Drawing_Point2D,
        endRadius: f32,
        colors: *const u32,
        pos: *const f32,
        size: u32,
        arg1: OH_Drawing_TileMode,
        arg2: *const OH_Drawing_Matrix,
    ) -> *mut OH_Drawing_ShaderEffect;
}

extern "C" {
    pub fn OH_Drawing_TextBlobCreateFromText(
        text: *const ::core::ffi::c_void,
        byteLength: usize,
        arg1: *const OH_Drawing_Font,
        arg2: OH_Drawing_TextEncoding,
    ) -> *mut OH_Drawing_TextBlob;
}
extern "C" {
    pub fn OH_Drawing_TextBlobCreateFromPosText(
        text: *const ::core::ffi::c_void,
        byteLength: usize,
        arg1: *mut OH_Drawing_Point2D,
        arg2: *const OH_Drawing_Font,
        arg3: OH_Drawing_TextEncoding,
    ) -> *mut OH_Drawing_TextBlob;
}
extern "C" {
    pub fn OH_Drawing_TextBlobCreateFromString(
        str_: *const ::core::ffi::c_char,
        arg1: *const OH_Drawing_Font,
        arg2: OH_Drawing_TextEncoding,
    ) -> *mut OH_Drawing_TextBlob;
}
extern "C" {
    pub fn OH_Drawing_TextBlobGetBounds(arg1: *mut OH_Drawing_TextBlob, arg2: *mut OH_Drawing_Rect);
}
extern "C" {
    pub fn OH_Drawing_TextBlobUniqueID(arg1: *const OH_Drawing_TextBlob) -> u32;
}
extern "C" {
    pub fn OH_Drawing_TypefaceCreateFromFile(
        path: *const ::core::ffi::c_char,
        index: ::core::ffi::c_int,
    ) -> *mut OH_Drawing_Typeface;
}
extern "C" {
    pub fn OH_Drawing_TypefaceCreateFromStream(
        arg1: *mut OH_Drawing_MemoryStream,
        index: i32,
    ) -> *mut OH_Drawing_Typeface;
}
