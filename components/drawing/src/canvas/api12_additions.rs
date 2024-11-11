#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::canvas::OH_Drawing_CanvasClipOp;
use crate::error_code::OH_Drawing_ErrorCode;
use crate::types::{
    OH_Drawing_Bitmap, OH_Drawing_BlendMode, OH_Drawing_Brush, OH_Drawing_Canvas, OH_Drawing_Font,
    OH_Drawing_Image, OH_Drawing_Image_Info, OH_Drawing_Matrix, OH_Drawing_Path,
    OH_Drawing_PixelMap, OH_Drawing_Point2D, OH_Drawing_Point3D, OH_Drawing_Rect,
    OH_Drawing_Region, OH_Drawing_RoundRect, OH_Drawing_SamplingOptions,
};

impl OH_Drawing_SrcRectConstraint {
    /// Using sampling only inside bounds in a slower manner.
    pub const STRICT_SRC_RECT_CONSTRAINT: OH_Drawing_SrcRectConstraint =
        OH_Drawing_SrcRectConstraint(0);
}
impl OH_Drawing_SrcRectConstraint {
    /// Using sampling outside bounds in a faster manner.
    pub const FAST_SRC_RECT_CONSTRAINT: OH_Drawing_SrcRectConstraint =
        OH_Drawing_SrcRectConstraint(1);
}
#[repr(transparent)]
/** @brief Enumeration defines the constraint type.

@since 12
@version 1.0*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct OH_Drawing_SrcRectConstraint(pub ::core::ffi::c_uint);
impl OH_Drawing_PointMode {
    /// Draw each point separately.
    pub const POINT_MODE_POINTS: OH_Drawing_PointMode = OH_Drawing_PointMode(0);
}
impl OH_Drawing_PointMode {
    /// Draw each pair of points as a line segment.
    pub const POINT_MODE_LINES: OH_Drawing_PointMode = OH_Drawing_PointMode(1);
}
impl OH_Drawing_PointMode {
    /// Draw the array of points as a open polygon.
    pub const POINT_MODE_POLYGON: OH_Drawing_PointMode = OH_Drawing_PointMode(2);
}
#[repr(transparent)]
/** @brief Enumerates of scale to fit flags, selects if an array of points are drawn as discrete points, as lines,
or as an open polygon.

@since 12
@version 1.0*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct OH_Drawing_PointMode(pub ::core::ffi::c_uint);

impl OH_Drawing_CanvasShadowFlags {
    /// Use no shadow flags.
    pub const SHADOW_FLAGS_NONE: OH_Drawing_CanvasShadowFlags = OH_Drawing_CanvasShadowFlags(0);
}
impl OH_Drawing_CanvasShadowFlags {
    /// The occluding object is transparent.
    pub const SHADOW_FLAGS_TRANSPARENT_OCCLUDER: OH_Drawing_CanvasShadowFlags =
        OH_Drawing_CanvasShadowFlags(1);
}
impl OH_Drawing_CanvasShadowFlags {
    /// No need to analyze shadows.
    pub const SHADOW_FLAGS_GEOMETRIC_ONLY: OH_Drawing_CanvasShadowFlags =
        OH_Drawing_CanvasShadowFlags(2);
}
impl OH_Drawing_CanvasShadowFlags {
    /// Use all shadow flags.
    pub const SHADOW_FLAGS_ALL: OH_Drawing_CanvasShadowFlags = OH_Drawing_CanvasShadowFlags(3);
}
#[repr(transparent)]
/** @brief Enumerates of shadow flags.

@since 12
@version 1.0*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct OH_Drawing_CanvasShadowFlags(pub ::core::ffi::c_uint);
impl OH_Drawing_VertexMode {
    /// The vertices are a triangle list.
    pub const VERTEX_MODE_TRIANGLES: OH_Drawing_VertexMode = OH_Drawing_VertexMode(0);
}
impl OH_Drawing_VertexMode {
    /// The vertices are a triangle strip.
    pub const VERTEX_MODE_TRIANGLES_STRIP: OH_Drawing_VertexMode = OH_Drawing_VertexMode(1);
}
impl OH_Drawing_VertexMode {
    /// The vertices are a triangle fan.
    pub const VERTEX_MODE_TRIANGLE_FAN: OH_Drawing_VertexMode = OH_Drawing_VertexMode(2);
}
#[repr(transparent)]
/** @brief Enumerates of vertices flags.

@since 12
@version 1.0*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct OH_Drawing_VertexMode(pub ::core::ffi::c_uint);

extern "C" {
    /** @brief Saves matrix and clip, and allocates a bitmap for subsequent drawing.
    Calling restore discards changes to matrix and clip, and draws the bitmap.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Canvas Indicates the pointer to an <b>OH_Drawing_Canvas</b> object.
    @param OH_Drawing_Rect Indicates the pointer to an <b>OH_Drawing_Rect</b> object.
    @param OH_Drawing_Brush Indicates the pointer to an <b>OH_Drawing_Brush</b> object.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_CanvasSaveLayer(
        arg1: *mut OH_Drawing_Canvas,
        arg2: *const OH_Drawing_Rect,
        arg3: *const OH_Drawing_Brush,
    );
    /** @brief Draw the specified area of the Media::PixelMap to the specified area of the canvas.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Canvas Indicates the pointer to an <b>OH_Drawing_Canvas</b> object.
    @param OH_Drawing_PixelMap Indicates the pointer to an <b>OH_Drawing_PixelMap</b> object.
    @param src the area of source pixelmap.
    @param dst the area of destination canvas.
    @param OH_Drawing_SamplingOptions the sampling mode.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_CanvasDrawPixelMapRect(
        arg1: *mut OH_Drawing_Canvas,
        arg2: *mut OH_Drawing_PixelMap,
        src: *const OH_Drawing_Rect,
        dst: *const OH_Drawing_Rect,
        arg3: *const OH_Drawing_SamplingOptions,
    );
    /** @brief Fills clipped canvas area with brush.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Canvas Indicates the pointer to an <b>OH_Drawing_Canvas</b> object.
    @param OH_Drawing_Brush Indicates the pointer to an <b>OH_Drawing_Brush</b> object.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_CanvasDrawBackground(
        arg1: *mut OH_Drawing_Canvas,
        arg2: *const OH_Drawing_Brush,
    );
    /** @brief Draws region using clip, matrix and paint.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Canvas Indicates the pointer to an <b>OH_Drawing_Canvas</b> object.
    @param OH_Drawing_Region Indicates the pointer to an <b>OH_Drawing_Region</b> object.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_CanvasDrawRegion(
        arg1: *mut OH_Drawing_Canvas,
        arg2: *const OH_Drawing_Region,
    );
    /** @brief Draws a point.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param canvas Indicates the pointer to an <b>OH_Drawing_Canvas</b> object.
    @param point Indicates the pointer to an <b>OH_Drawing_Point</b> object.
    @return Returns the error code.
            Returns {@link OH_DRAWING_SUCCESS} if the operation is successful.
            Returns {@link OH_DRAWING_ERROR_INVALID_PARAMETER} if canvas or point is nullptr.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_CanvasDrawPoint(
        canvas: *mut OH_Drawing_Canvas,
        point: *const OH_Drawing_Point2D,
    ) -> OH_Drawing_ErrorCode;
    /** @brief Draws point array as separate point, line segment or open polygon according to given point mode.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Canvas Indicates the pointer to an <b>OH_Drawing_Canvas</b> object.
    @param mode Draw points enum.
    @param count The point count.
    @param OH_Drawing_Point2D Point struct array.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_CanvasDrawPoints(
        arg1: *mut OH_Drawing_Canvas,
        mode: OH_Drawing_PointMode,
        count: u32,
        arg2: *const OH_Drawing_Point2D,
    );
    /** @brief Draw the specified area of the bitmap to the specified area of the canvas.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Canvas Indicates the pointer to an <b>OH_Drawing_Canvas</b> object.
    @param OH_Drawing_Bitmap Indicates the pointer to an <b>OH_Drawing_Bitmap</b> object.
    @param src the area of source bitmap, can be nullptr.
    @param dst the area of destination canvas.
    @param OH_Drawing_SamplingOptions the sampling mode.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_CanvasDrawBitmapRect(
        arg1: *mut OH_Drawing_Canvas,
        arg2: *const OH_Drawing_Bitmap,
        src: *const OH_Drawing_Rect,
        dst: *const OH_Drawing_Rect,
        arg3: *const OH_Drawing_SamplingOptions,
    );
    /** @brief Fills the entire canvas with the specified color and blend mode.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param canvas Indicates the pointer to an <b>OH_Drawing_Canvas</b> object.
    @param color Indicates the color, which is a 32-bit variable.
    @param blendMode Indicates the blend mode.
    @return Returns the error code.
            Returns {@link OH_DRAWING_SUCCESS} if the operation is successful.
            Returns {@link OH_DRAWING_ERROR_INVALID_PARAMETER} if canvas is nullptr.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_CanvasDrawColor(
        canvas: *mut OH_Drawing_Canvas,
        color: u32,
        blendMode: OH_Drawing_BlendMode,
    ) -> OH_Drawing_ErrorCode;
    /** @brief Clip a round rect.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Canvas Indicates the pointer to an <b>OH_Drawing_Canvas</b> object.
    @param OH_Drawing_RoundRect Indicates the pointer to an <b>OH_Drawing_RoundRect</b> object.
    @param clipOp Indicates the operation to apply to clip.
    @param doAntiAlias Indicates whether clip operation requires anti-aliased.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_CanvasClipRoundRect(
        arg1: *mut OH_Drawing_Canvas,
        arg2: *const OH_Drawing_RoundRect,
        clipOp: OH_Drawing_CanvasClipOp,
        doAntiAlias: bool,
    );
    /** @brief Draws a single character.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Canvas Indicates the pointer to an <b>OH_Drawing_Canvas</b> object.
    @param str Indicates the single character encoded in UTF-8.
    @param OH_Drawing_Font Indicates the pointer to an <b>OH_Drawing_Font</b> object.
    @param x Indicates the horizontal offset applied to the single character.
    @param y Indicates the vertical offset applied to the single character.
    @return Returns the error code.
            Returns {@link OH_DRAWING_SUCCESS} if the operation is successful.
            Returns {@link OH_DRAWING_ERROR_INVALID_PARAMETER} if any of canvas, str
                    and font is nullptr or strlen(str) is 0.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_CanvasDrawSingleCharacter(
        canvas: *mut OH_Drawing_Canvas,
        str_: *const ::core::ffi::c_char,
        font: *const OH_Drawing_Font,
        x: f32,
        y: f32,
    ) -> OH_Drawing_ErrorCode;
    /** @brief Clips a region.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param canvas Indicates the pointer to an <b>OH_Drawing_Canvas</b> object.
    @param region Indicates the pointer to an <b>OH_Drawing_Region</b> object.
    @param clipOp To apply to clip.
    @return Returns the error code.
            Returns {@link OH_DRAWING_SUCCESS} if the operation is successful.
            Returns {@link OH_DRAWING_ERROR_INVALID_PARAMETER} if canvas or region is nullptr.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_CanvasClipRegion(
        canvas: *mut OH_Drawing_Canvas,
        region: *const OH_Drawing_Region,
        clipOp: OH_Drawing_CanvasClipOp,
    ) -> OH_Drawing_ErrorCode;
    /** @brief Skew by sx on the x-axis and sy on the y-axis.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Canvas Indicates the pointer to an <b>OH_Drawing_Canvas</b> object.
    @param sx Indicates the amount to skew on x-axis.
    @param sy Indicates the amount to skew on y-axis.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_CanvasSkew(arg1: *mut OH_Drawing_Canvas, sx: f32, sy: f32);
    /** @brief Get the width of a canvas.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Canvas Indicates the pointer to an <b>OH_Drawing_Canvas</b> object.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_CanvasGetWidth(arg1: *mut OH_Drawing_Canvas) -> i32;
    /** @brief Get the height of a canvas.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Canvas Indicates the pointer to an <b>OH_Drawing_Canvas</b> object.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_CanvasGetHeight(arg1: *mut OH_Drawing_Canvas) -> i32;
    /** @brief Get the bounds of clip of a canvas.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Canvas Indicates the pointer to an <b>OH_Drawing_Canvas</b> object.
    @param OH_Drawing_Rect Indicates the pointer to an <b>OH_Drawing_Rect</b> object.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_CanvasGetLocalClipBounds(
        arg1: *mut OH_Drawing_Canvas,
        arg2: *mut OH_Drawing_Rect,
    );
    /** @brief Get a 3x3 matrix of the transform from local coordinates to 'device'.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Canvas Indicates the pointer to an <b>OH_Drawing_Canvas</b> object.
    @param OH_Drawing_Matrix Indicates the pointer to an <b>OH_Drawing_Matrix</b> object.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_CanvasGetTotalMatrix(
        arg1: *mut OH_Drawing_Canvas,
        arg2: *mut OH_Drawing_Matrix,
    );
    /** @brief Use the passed matrix to transforming the geometry, then use existing matrix.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Canvas Indicates the pointer to an <b>OH_Drawing_Canvas</b> object.
    @param OH_Drawing_Matrix Indicates the pointer to an <b>OH_Drawing_Matrix</b> object,
    represents the matrix which is passed.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_CanvasConcatMatrix(
        arg1: *mut OH_Drawing_Canvas,
        arg2: *mut OH_Drawing_Matrix,
    );
    /** @brief Use circular light to draw an offset spot shadow and outlining ambient shadow for the given path.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Canvas Indicates the pointer to an <b>OH_Drawing_Canvas</b> object.
    @param OH_Drawing_Path Indicates the pointer to an <b>OH_Drawing_Path</b> object, use to generate shadows.
    @param planeParams Represents the value of the function which returns Z offset of the occluder from the
    canvas based on x and y.
    @param devLightPos Represents the position of the light relative to the canvas.
    @param lightRadius The radius of the circular light.
    @param ambientColor Ambient shadow's color.
    @param spotColor Spot shadow's color.
    @param flag Indicates the flag to control opaque occluder, shadow, and light position.
    @since 12
    @version 1.0*/
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
    /** @brief Sets matrix of canvas.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Canvas Indicates the pointer to an <b>OH_Drawing_Canvas</b> object.
    @param OH_Drawing_Matrix Indicates the pointer to an <b>OH_Drawing_Matrix</b> object.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_CanvasSetMatrix(arg1: *mut OH_Drawing_Canvas, arg2: *mut OH_Drawing_Matrix);
    /** @brief Reset matrix to the idenmtity matrix, any prior matrix state is overwritten.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Canvas Indicates the pointer to an <b>OH_Drawing_Canvas</b> object.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_CanvasResetMatrix(arg1: *mut OH_Drawing_Canvas);
    /** @brief Draws the specified source rectangle of the image onto the canvas,
    scaled and translated to the destination rectangle.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Canvas Indicates the pointer to an <b>OH_Drawing_Canvas</b> object.
    @param OH_Drawing_Image Indicates the pointer to an <b>OH_Drawing_Image</b> object.
    @param src The area of source image.
    @param dst The area of destination canvas.
    @param OH_Drawing_SamplingOptions Indicates the pointer to an <b>OH_Drawing_SamplingOptions</b> object.
    @param OH_Drawing_SrcRectConstraint Constraint type.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_CanvasDrawImageRectWithSrc(
        arg1: *mut OH_Drawing_Canvas,
        arg2: *const OH_Drawing_Image,
        src: *const OH_Drawing_Rect,
        dst: *const OH_Drawing_Rect,
        arg3: *const OH_Drawing_SamplingOptions,
        arg4: OH_Drawing_SrcRectConstraint,
    );
    /** @brief Draws the specified source rectangle of the image onto the canvas,
    scaled and translated to the destination rectangle.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Canvas Indicates the pointer to an <b>OH_Drawing_Canvas</b> object.
    @param OH_Drawing_Image Indicates the pointer to an <b>OH_Drawing_Image</b> object.
    @param OH_Drawing_Rect Indicates the pointer to an <b>OH_Drawing_Rect</b> object.
    @param OH_Drawing_SamplingOptions Indicates the pointer to an <b>OH_Drawing_SamplingOptions</b> object.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_CanvasDrawImageRect(
        arg1: *mut OH_Drawing_Canvas,
        arg2: *mut OH_Drawing_Image,
        dst: *mut OH_Drawing_Rect,
        arg3: *mut OH_Drawing_SamplingOptions,
    );
    /** @brief Draw a triangular mesh with vertex descriptions.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Canvas Indicates the pointer to an <b>OH_Drawing_Canvas</b> object.
    @param vertexMmode Draw a set of vertices.
    @param vertexCount Vertex count.
    @param positions Positions data pointer.
    @param texs Texture coordinate data pointer.
    @param colors Color data pointer.
    @param indexCount Index count.
    @param indices Index data pointer.
    @since 12
    @version 1.0*/
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
    /** @brief Read pixels data from canvas.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Canvas Indicates the pointer to an <b>OH_Drawing_Canvas</b> object.
    @param OH_Drawing_Image_Info width, height, colorType, and alphaType of dstPixels.
    @param dstPixels destination pixel storage.
    @param dstRowBytes size of one row of pixels.
    @param srcX offset into canvas writable pixels on x-axis.
    @param srcY offset into canvas writable pixels on y-axis.
    @return true if pixels are copied to dstPixels.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_CanvasReadPixels(
        arg1: *mut OH_Drawing_Canvas,
        arg2: *mut OH_Drawing_Image_Info,
        dstPixels: *mut ::core::ffi::c_void,
        dstRowBytes: u32,
        srcX: i32,
        srcY: i32,
    ) -> bool;
    /** @brief Read pixels data to a bitmap from canvas.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Canvas Indicates the pointer to an <b>OH_Drawing_Canvas</b> object.
    @param OH_Drawing_Bitmap Indicates the pointer to an <b>OH_Drawing_Bitmap</b> object.
    @param srcX offset into canvas writable pixels on x-axis.
    @param srcY offset into canvas writable pixels on y-axis.
    @return true if pixels are copied to dstBitmap.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_CanvasReadPixelsToBitmap(
        arg1: *mut OH_Drawing_Canvas,
        arg2: *mut OH_Drawing_Bitmap,
        srcX: i32,
        srcY: i32,
    ) -> bool;
    /** @brief Checks whether the drawable area is empty.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param canvas Indicates the pointer to an <b>OH_Drawing_Canvas</b> object.
    @param isClipEmpty Indicates if drawable area is empty.
    @return Returns the error code.
            Returns {@link OH_DRAWING_SUCCESS} if the operation is successful.
            Returns {@link OH_DRAWING_ERROR_INVALID_PARAMETER} if canvas or isClipEmpty is nullptr.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_CanvasIsClipEmpty(
        canvas: *mut OH_Drawing_Canvas,
        isClipEmpty: *mut bool,
    ) -> OH_Drawing_ErrorCode;
    /** @brief Gets image info of canvas.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param canvas Indicates the pointer to an <b>OH_Drawing_Canvas</b> object.
    @param imageInfo Indicates the pointer to an <b>OH_Drawing_Image_Info</b> object.
    @return Returns the error code.
            Returns {@link OH_DRAWING_SUCCESS} if the operation is successful.
            Returns {@link OH_DRAWING_ERROR_INVALID_PARAMETER} if canvas or imageInfo is nullptr.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_CanvasGetImageInfo(
        canvas: *mut OH_Drawing_Canvas,
        imageInfo: *mut OH_Drawing_Image_Info,
    ) -> OH_Drawing_ErrorCode;
}
