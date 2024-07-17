#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::drawing::types::{
    OH_Drawing_Bitmap, OH_Drawing_Canvas, OH_Drawing_Path, OH_Drawing_Point, OH_Drawing_Rect,
    OH_Drawing_RoundRect, OH_Drawing_TextBlob,
};

extern "C" {
    /** @brief Gets the number of the canvas status (canvas matrix) saved in the stack.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Canvas Indicates the pointer to an <b>OH_Drawing_Canvas</b> object.
    @return Returns a 32-bit variable that describes the number of canvas status.
    @since 11
    @version 1.0*/
    pub fn OH_Drawing_CanvasGetSaveCount(arg1: *mut OH_Drawing_Canvas) -> u32;

    /** @brief Restores the specific number of the canvas status (canvas matrix) saved in the stack.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Canvas Indicates the pointer to an <b>OH_Drawing_Canvas</b> object.
    @param saveCount Indicates the specific number of canvas status.
    @since 11
    @version 1.0*/
    pub fn OH_Drawing_CanvasRestoreToCount(arg1: *mut OH_Drawing_Canvas, saveCount: u32);
}

extern "C" {
    /** @brief Draws a bitmap.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Canvas Indicates the pointer to an <b>OH_Drawing_Canvas</b> object.
    @param OH_Drawing_Bitmap Indicates the pointer to an <b>OH_Drawing_Bitmap</b> object.
    @param left Indicates the left position of the <b>OH_Drawing_Bitmap</b>.
    @param top Indicates the top position of the <b>OH_Drawing_Bitmap</b>.
    @since 11
    @version 1.0*/
    pub fn OH_Drawing_CanvasDrawBitmap(
        arg1: *mut OH_Drawing_Canvas,
        arg2: *const OH_Drawing_Bitmap,
        left: f32,
        top: f32,
    );

    /** @brief Draws a rect.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Canvas Indicates the pointer to an <b>OH_Drawing_Canvas</b> object.
    @param OH_Drawing_Rect Indicates the pointer to an <b>OH_Drawing_Rect</b> object.
    @since 11
    @version 1.0*/
    pub fn OH_Drawing_CanvasDrawRect(arg1: *mut OH_Drawing_Canvas, arg2: *const OH_Drawing_Rect);

    /** @brief Draws a circle.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Canvas Indicates the pointer to an <b>OH_Drawing_Canvas</b> object.
    @param OH_Drawing_Point Indicates the pointer to an <b>OH_Drawing_Point</b> object.
    @param radius Indicates the radius of the circle.
    @since 11
    @version 1.0*/
    pub fn OH_Drawing_CanvasDrawCircle(
        arg1: *mut OH_Drawing_Canvas,
        arg2: *const OH_Drawing_Point,
        radius: f32,
    );

    /** @brief Draws an oval.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Canvas Indicates the pointer to an <b>OH_Drawing_Canvas</b> object.
    @param OH_Drawing_Rect Indicates the pointer to an <b>OH_Drawing_Rect</b> object.
    @since 11
    @version 1.0*/
    pub fn OH_Drawing_CanvasDrawOval(arg1: *mut OH_Drawing_Canvas, arg2: *const OH_Drawing_Rect);

    /** @brief Draws an arc.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Canvas Indicates the pointer to an <b>OH_Drawing_Canvas</b> object.
    @param OH_Drawing_Rect Indicates the pointer to an <b>OH_Drawing_Rect</b> object.
    @param startAngle Indicates the startAngle of the arc.
    @param sweepAngle Indicates the sweepAngle of the arc.
    @since 11
    @version 1.0*/
    pub fn OH_Drawing_CanvasDrawArc(
        arg1: *mut OH_Drawing_Canvas,
        arg2: *const OH_Drawing_Rect,
        startAngle: f32,
        sweepAngle: f32,
    );

    /** @brief Draws a roundrect.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Canvas Indicates the pointer to an <b>OH_Drawing_Canvas</b> object.
    @param OH_Drawing_RoundRect Indicates the pointer to an <b>OH_Drawing_RoundRect</b> object.
    @since 11
    @version 1.0*/
    pub fn OH_Drawing_CanvasDrawRoundRect(
        arg1: *mut OH_Drawing_Canvas,
        arg2: *const OH_Drawing_RoundRect,
    );

    /** @brief Draws a textblob.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Canvas Indicates the pointer to an <b>OH_Drawing_Canvas</b> object.
    @param OH_Drawing_TextBlob Indicates the pointer to an <b>OH_Drawing_TextBlob</b> object.
    @param x Indicates the horizontal offset applied to blob.
    @param y Indicates the vertical offset applied to blob.
    @since 11
    @version 1.0*/
    pub fn OH_Drawing_CanvasDrawTextBlob(
        arg1: *mut OH_Drawing_Canvas,
        arg2: *const OH_Drawing_TextBlob,
        x: f32,
        y: f32,
    );
}
impl OH_Drawing_CanvasClipOp {
    /// Clip with difference.
    pub const DIFFERENCE: OH_Drawing_CanvasClipOp = OH_Drawing_CanvasClipOp(0);
}
impl OH_Drawing_CanvasClipOp {
    /// Clip with intersection.
    pub const INTERSECT: OH_Drawing_CanvasClipOp = OH_Drawing_CanvasClipOp(1);
}
#[repr(transparent)]
/** @brief Enumerates clip op.

@since 11
@version 1.0*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct OH_Drawing_CanvasClipOp(pub ::core::ffi::c_uint);
extern "C" {
    /** @brief Clip a rect.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Canvas Indicates the pointer to an <b>OH_Drawing_Canvas</b> object.
    @param OH_Drawing_Rect Indicates the pointer to an <b>OH_Drawing_Rect</b> object.
    @param clipOp Indicates the operation to apply to clip.
    @param doAntiAlias Indicates whether clip operation requires anti-aliased.
    @since 11
    @version 1.0*/
    pub fn OH_Drawing_CanvasClipRect(
        arg1: *mut OH_Drawing_Canvas,
        arg2: *const OH_Drawing_Rect,
        clipOp: OH_Drawing_CanvasClipOp,
        doAntiAlias: bool,
    );

    /** @brief Clip a path.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Canvas Indicates the pointer to an <b>OH_Drawing_Canvas</b> object.
    @param OH_Drawing_Path Indicates the pointer to an <b>OH_Drawing_Path</b> object.
    @param clipOp Indicates the operation to apply to clip.
    @param doAntiAlias Indicates whether clip operation requires anti-aliased.
    @since 11
    @version 1.0*/
    pub fn OH_Drawing_CanvasClipPath(
        arg1: *mut OH_Drawing_Canvas,
        arg2: *const OH_Drawing_Path,
        clipOp: OH_Drawing_CanvasClipOp,
        doAntiAlias: bool,
    );

    /** @brief Rotates by degrees. Positive degrees rotates clockwise.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Canvas Indicates the pointer to an <b>OH_Drawing_Canvas</b> object.
    @param degrees Indicates the amount to rotate, in degrees.
    @param px Indicates the x-axis value of the point to rotate about.
    @param py Indicates the y-axis value of the point to rotate about.
    @since 11
    @version 1.0*/
    pub fn OH_Drawing_CanvasRotate(arg1: *mut OH_Drawing_Canvas, degrees: f32, px: f32, py: f32);

    /** @brief Translates by dx along the x-axis and dy along the y-axis.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Canvas Indicates the pointer to an <b>OH_Drawing_Canvas</b> object.
    @param dx Indicates the distance to translate on x-axis.
    @param dy Indicates the distance to translate on y-axis.
    @since 11
    @version 1.0*/
    pub fn OH_Drawing_CanvasTranslate(arg1: *mut OH_Drawing_Canvas, dx: f32, dy: f32);

    /** @brief Scales by sx on the x-axis and sy on the y-axis.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Canvas Indicates the pointer to an <b>OH_Drawing_Canvas</b> object.
    @param sx Indicates the amount to scale on x-axis.
    @param sy Indicates the amount to scale on y-axis.
    @since 11
    @version 1.0*/
    pub fn OH_Drawing_CanvasScale(arg1: *mut OH_Drawing_Canvas, sx: f32, sy: f32);
}
