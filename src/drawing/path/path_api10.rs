/* automatically generated by rust-bindgen 0.69.4 */

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use crate::drawing::types::*;

extern "C" {
    /** @brief Creates an <b>OH_Drawing_Path</b> object.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @return Returns the pointer to the <b>OH_Drawing_Path</b> object created.
    @since 8
    @version 1.0*/
    pub fn OH_Drawing_PathCreate() -> *mut OH_Drawing_Path;
    /** @brief Destroys an <b>OH_Drawing_Path</b> object and reclaims the memory occupied by the object.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Path Indicates the pointer to an <b>OH_Drawing_Path</b> object.
    @since 8
    @version 1.0*/
    pub fn OH_Drawing_PathDestroy(arg1: *mut OH_Drawing_Path);
    /** @brief Sets the start point of a path.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Path Indicates the pointer to an <b>OH_Drawing_Path</b> object.
    @param x Indicates the x coordinate of the start point.
    @param y Indicates the y coordinate of the start point.
    @since 8
    @version 1.0*/
    pub fn OH_Drawing_PathMoveTo(arg1: *mut OH_Drawing_Path, x: f32, y: f32);
    /** @brief Draws a line segment from the last point of a path to the target point.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Path Indicates the pointer to an <b>OH_Drawing_Path</b> object.
    @param x Indicates the x coordinate of the target point.
    @param y Indicates the y coordinate of the target point.
    @since 8
    @version 1.0*/
    pub fn OH_Drawing_PathLineTo(arg1: *mut OH_Drawing_Path, x: f32, y: f32);
    /** @brief Draws an arc to a path.

    This is done by using angle arc mode. In this mode, a rectangle that encloses an ellipse is specified first,
    and then a start angle and a sweep angle are specified.
    The arc is a portion of the ellipse defined by the start angle and the sweep angle.
    By default, a line segment from the last point of the path to the start point of the arc is also added.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Path Indicates the pointer to an <b>OH_Drawing_Path</b> object.
    @param x1 Indicates the x coordinate of the upper left corner of the rectangle.
    @param y1 Indicates the y coordinate of the upper left corner of the rectangle.
    @param x2 Indicates the x coordinate of the lower right corner of the rectangle.
    @param y2 Indicates the y coordinate of the lower right corner of the rectangle.
    @param startDeg Indicates the start angle, in degrees.
    @param sweepDeg Indicates the angle to sweep, in degrees.
    @since 8
    @version 1.0*/
    pub fn OH_Drawing_PathArcTo(
        arg1: *mut OH_Drawing_Path,
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        startDeg: f32,
        sweepDeg: f32,
    );
    /** @brief Draws a quadratic Bezier curve from the last point of a path to the target point.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Path Indicates the pointer to an <b>OH_Drawing_Path</b> object.
    @param ctrlX Indicates the x coordinate of the control point.
    @param ctrlY Indicates the y coordinate of the control point.
    @param endX Indicates the x coordinate of the target point.
    @param endY Indicates the y coordinate of the target point.
    @since 8
    @version 1.0*/
    pub fn OH_Drawing_PathQuadTo(
        arg1: *mut OH_Drawing_Path,
        ctrlX: f32,
        ctrlY: f32,
        endX: f32,
        endY: f32,
    );
    /** @brief Draws a cubic Bezier curve from the last point of a path to the target point.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Path Indicates the pointer to an <b>OH_Drawing_Path</b> object.
    @param ctrlX1 Indicates the x coordinate of the first control point.
    @param ctrlY1 Indicates the y coordinate of the first control point.
    @param ctrlX2 Indicates the x coordinate of the second control point.
    @param ctrlY2 Indicates the y coordinate of the second control point.
    @param endX Indicates the x coordinate of the target point.
    @param endY Indicates the y coordinate of the target point.
    @since 8
    @version 1.0*/
    pub fn OH_Drawing_PathCubicTo(
        arg1: *mut OH_Drawing_Path,
        ctrlX1: f32,
        ctrlY1: f32,
        ctrlX2: f32,
        ctrlY2: f32,
        endX: f32,
        endY: f32,
    );
    /** @brief Closes a path. A line segment from the start point to the last point of the path is added.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Path Indicates the pointer to an <b>OH_Drawing_Path</b> object.
    @since 8
    @version 1.0*/
    pub fn OH_Drawing_PathClose(arg1: *mut OH_Drawing_Path);
    /** @brief Resets path data.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Path Indicates the pointer to an <b>OH_Drawing_Path</b> object.
    @since 8
    @version 1.0*/
    pub fn OH_Drawing_PathReset(arg1: *mut OH_Drawing_Path);
}
