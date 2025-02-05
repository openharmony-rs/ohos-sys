// automatically generated by rust-bindgen 0.71.1

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use crate::types::*;

#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
impl OH_Drawing_PathDirection {
    /// clockwise direction for adding closed contours
    pub const PATH_DIRECTION_CW: OH_Drawing_PathDirection = OH_Drawing_PathDirection(0);
    /// counter-clockwise direction for adding closed contours
    pub const PATH_DIRECTION_CCW: OH_Drawing_PathDirection = OH_Drawing_PathDirection(1);
}
#[repr(transparent)]
/// Direction for adding closed contours.
///
///
/// Available since API-level: 12
///
/// Version: 1.0
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct OH_Drawing_PathDirection(pub ::core::ffi::c_uint);
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
impl OH_Drawing_PathFillType {
    /// Specifies that "inside" is computed by a non-zero sum of signed edge crossings
    pub const PATH_FILL_TYPE_WINDING: OH_Drawing_PathFillType = OH_Drawing_PathFillType(0);
    /// Specifies that "inside" is computed by an odd number of edge crossings
    pub const PATH_FILL_TYPE_EVEN_ODD: OH_Drawing_PathFillType = OH_Drawing_PathFillType(1);
    /// Same as Winding, but draws outside of the path, rather than inside
    pub const PATH_FILL_TYPE_INVERSE_WINDING: OH_Drawing_PathFillType = OH_Drawing_PathFillType(2);
    /// Same as EvenOdd, but draws outside of the path, rather than inside
    pub const PATH_FILL_TYPE_INVERSE_EVEN_ODD: OH_Drawing_PathFillType = OH_Drawing_PathFillType(3);
}
#[repr(transparent)]
/// FillType of path.
///
///
/// Available since API-level: 12
///
/// Version: 1.0
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct OH_Drawing_PathFillType(pub ::core::ffi::c_uint);
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
impl OH_Drawing_PathAddMode {
    /// Appended to destination unaltered
    pub const PATH_ADD_MODE_APPEND: OH_Drawing_PathAddMode = OH_Drawing_PathAddMode(0);
    /// Add line if prior contour is not closed
    pub const PATH_ADD_MODE_EXTEND: OH_Drawing_PathAddMode = OH_Drawing_PathAddMode(1);
}
#[repr(transparent)]
/// Add mode of path.
///
///
/// Available since API-level: 12
///
/// Version: 1.0
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct OH_Drawing_PathAddMode(pub ::core::ffi::c_uint);
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
impl OH_Drawing_PathOpMode {
    /// Difference operation.
    pub const PATH_OP_MODE_DIFFERENCE: OH_Drawing_PathOpMode = OH_Drawing_PathOpMode(0);
    /// Intersect operation.
    pub const PATH_OP_MODE_INTERSECT: OH_Drawing_PathOpMode = OH_Drawing_PathOpMode(1);
    /// Union operation.
    pub const PATH_OP_MODE_UNION: OH_Drawing_PathOpMode = OH_Drawing_PathOpMode(2);
    /// Xor operation.
    pub const PATH_OP_MODE_XOR: OH_Drawing_PathOpMode = OH_Drawing_PathOpMode(3);
    /// Reverse difference operation.
    pub const PATH_OP_MODE_REVERSE_DIFFERENCE: OH_Drawing_PathOpMode = OH_Drawing_PathOpMode(4);
}
#[repr(transparent)]
/// Operations when two paths are combined.
///
///
/// Available since API-level: 12
///
/// Version: 1.0
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct OH_Drawing_PathOpMode(pub ::core::ffi::c_uint);
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
impl OH_Drawing_PathMeasureMatrixFlags {
    /// Gets position.
    pub const GET_POSITION_MATRIX: OH_Drawing_PathMeasureMatrixFlags =
        OH_Drawing_PathMeasureMatrixFlags(0);
    /// Gets tangent.
    pub const GET_TANGENT_MATRIX: OH_Drawing_PathMeasureMatrixFlags =
        OH_Drawing_PathMeasureMatrixFlags(1);
    /// Gets both position and tangent.
    pub const GET_POSITION_AND_TANGENT_MATRIX: OH_Drawing_PathMeasureMatrixFlags =
        OH_Drawing_PathMeasureMatrixFlags(2);
}
#[repr(transparent)]
/// Enumerates the matrix information corresponding to the path measurements.
///
///
/// Available since API-level: 12
///
/// Version: 1.0
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct OH_Drawing_PathMeasureMatrixFlags(pub ::core::ffi::c_uint);
extern "C" {
    /// Creates an <b>OH_Drawing_Path</b> object.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    ///
    /// # Returns
    ///
    /// * Returns the pointer to the <b>OH_Drawing_Path</b> object created.
    ///
    /// Available since API-level: 8
    ///
    /// Version: 1.0
    pub fn OH_Drawing_PathCreate() -> *mut OH_Drawing_Path;
    /// Creates an <b>OH_Drawing_Path</b> copy object.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `path` - Indicates the pointer to an <b>OH_Drawing_Rect</b> object.
    ///
    /// # Returns
    ///
    /// * Returns the pointer to the <b>OH_Drawing_Path</b> object created.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_PathCopy(path: *mut OH_Drawing_Path) -> *mut OH_Drawing_Path;
    /// Destroys an <b>OH_Drawing_Path</b> object and reclaims the memory occupied by the object.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `path` - Indicates the pointer to an <b>OH_Drawing_Path</b> object.
    ///
    /// Available since API-level: 8
    ///
    /// Version: 1.0
    pub fn OH_Drawing_PathDestroy(path: *mut OH_Drawing_Path);
    /// Sets the start point of a path.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `path` - Indicates the pointer to an <b>OH_Drawing_Path</b> object.
    ///
    /// * `x` - Indicates the x coordinate of the start point.
    ///
    /// * `y` - Indicates the y coordinate of the start point.
    ///
    /// Available since API-level: 8
    ///
    /// Version: 1.0
    pub fn OH_Drawing_PathMoveTo(path: *mut OH_Drawing_Path, x: f32, y: f32);
    /// Draws a line segment from the last point of a path to the target point.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `path` - Indicates the pointer to an <b>OH_Drawing_Path</b> object.
    ///
    /// * `x` - Indicates the x coordinate of the target point.
    ///
    /// * `y` - Indicates the y coordinate of the target point.
    ///
    /// Available since API-level: 8
    ///
    /// Version: 1.0
    pub fn OH_Drawing_PathLineTo(path: *mut OH_Drawing_Path, x: f32, y: f32);
    /// Draws an arc to a path.
    ///
    /// This is done by using angle arc mode. In this mode, a rectangle that encloses an ellipse is specified first,
    /// and then a start angle and a sweep angle are specified.
    /// The arc is a portion of the ellipse defined by the start angle and the sweep angle.
    /// By default, a line segment from the last point of the path to the start point of the arc is also added.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `path` - Indicates the pointer to an <b>OH_Drawing_Path</b> object.
    ///
    /// * `x1` - Indicates the x coordinate of the upper left corner of the rectangle.
    ///
    /// * `y1` - Indicates the y coordinate of the upper left corner of the rectangle.
    ///
    /// * `x2` - Indicates the x coordinate of the lower right corner of the rectangle.
    ///
    /// * `y2` - Indicates the y coordinate of the lower right corner of the rectangle.
    ///
    /// * `startDeg` - Indicates the start angle, in degrees.
    ///
    /// * `sweepDeg` - Indicates the angle to sweep, in degrees.
    ///
    /// Available since API-level: 8
    ///
    /// Version: 1.0
    pub fn OH_Drawing_PathArcTo(
        path: *mut OH_Drawing_Path,
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        startDeg: f32,
        sweepDeg: f32,
    );
    /// Draws a quadratic Bezier curve from the last point of a path to the target point.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `path` - Indicates the pointer to an <b>OH_Drawing_Path</b> object.
    ///
    /// * `ctrlX` - Indicates the x coordinate of the control point.
    ///
    /// * `ctrlY` - Indicates the y coordinate of the control point.
    ///
    /// * `endX` - Indicates the x coordinate of the target point.
    ///
    /// * `endY` - Indicates the y coordinate of the target point.
    ///
    /// Available since API-level: 8
    ///
    /// Version: 1.0
    pub fn OH_Drawing_PathQuadTo(
        path: *mut OH_Drawing_Path,
        ctrlX: f32,
        ctrlY: f32,
        endX: f32,
        endY: f32,
    );
    /// Draws a conic from the last point of a path to the target point.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `path` - Indicates the pointer to an <b>OH_Drawing_Path</b> object.
    ///
    /// * `ctrlX` - Indicates the x coordinate of the control point.
    ///
    /// * `ctrlY` - Indicates the y coordinate of the control point.
    ///
    /// * `endX` - Indicates the x coordinate of the target point.
    ///
    /// * `endY` - Indicates the y coordinate of the target point.
    ///
    /// * `weight` - Indicates the weight of added conic.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_PathConicTo(
        path: *mut OH_Drawing_Path,
        ctrlX: f32,
        ctrlY: f32,
        endX: f32,
        endY: f32,
        weight: f32,
    );
    /// Draws a cubic Bezier curve from the last point of a path to the target point.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `path` - Indicates the pointer to an <b>OH_Drawing_Path</b> object.
    ///
    /// * `ctrlX1` - Indicates the x coordinate of the first control point.
    ///
    /// * `ctrlY1` - Indicates the y coordinate of the first control point.
    ///
    /// * `ctrlX2` - Indicates the x coordinate of the second control point.
    ///
    /// * `ctrlY2` - Indicates the y coordinate of the second control point.
    ///
    /// * `endX` - Indicates the x coordinate of the target point.
    ///
    /// * `endY` - Indicates the y coordinate of the target point.
    ///
    /// Available since API-level: 8
    ///
    /// Version: 1.0
    pub fn OH_Drawing_PathCubicTo(
        path: *mut OH_Drawing_Path,
        ctrlX1: f32,
        ctrlY1: f32,
        ctrlX2: f32,
        ctrlY2: f32,
        endX: f32,
        endY: f32,
    );
    /// Sets the relative starting point of a path.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `path` - Indicates the pointer to an <b>OH_Drawing_Path</b> object.
    ///
    /// * `x` - Indicates the x coordinate of the relative starting point.
    ///
    /// * `y` - Indicates the y coordinate of the relative starting point.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_PathRMoveTo(path: *mut OH_Drawing_Path, x: f32, y: f32);
    /// Draws a line segment from the last point of a path to the relative target point.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `path` - Indicates the pointer to an <b>OH_Drawing_Path</b> object.
    ///
    /// * `x` - Indicates the x coordinate of the relative target point.
    ///
    /// * `y` - Indicates the y coordinate of the relative target point.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_PathRLineTo(path: *mut OH_Drawing_Path, x: f32, y: f32);
    /// Draws a quadratic bezier curve from the last point of a path to the relative target point.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `path` - Indicates the pointer to an <b>OH_Drawing_Path</b> object.
    ///
    /// * `ctrlX` - Indicates the x coordinate of the relative control point.
    ///
    /// * `ctrlY` - Indicates the y coordinate of the relative control point.
    ///
    /// * `endX` - Indicates the x coordinate of the relative target point.
    ///
    /// * `endY` - Indicates the y coordinate of the relative target point.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_PathRQuadTo(
        path: *mut OH_Drawing_Path,
        ctrlX: f32,
        ctrlY: f32,
        endX: f32,
        endY: f32,
    );
    /// Draws a conic from the last point of a path to the relative target point.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `path` - Indicates the pointer to an <b>OH_Drawing_Path</b> object.
    ///
    /// * `ctrlX` - Indicates the x coordinate of the relative control point.
    ///
    /// * `ctrlY` - Indicates the y coordinate of the relative control point.
    ///
    /// * `endX` - Indicates the x coordinate of the relative target point.
    ///
    /// * `endY` - Indicates the y coordinate of the relative target point.
    ///
    /// * `weight` - Indicates the weight of added conic.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_PathRConicTo(
        path: *mut OH_Drawing_Path,
        ctrlX: f32,
        ctrlY: f32,
        endX: f32,
        endY: f32,
        weight: f32,
    );
    /// Draws a cubic bezier curve from the last point of a path to the relative target point.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `path` - Indicates the pointer to an <b>OH_Drawing_Path</b> object.
    ///
    /// * `ctrlX1` - Indicates the x coordinate of the first relative control point.
    ///
    /// * `ctrlY1` - Indicates the y coordinate of the first relative control point.
    ///
    /// * `ctrlX2` - Indicates the x coordinate of the second relative control point.
    ///
    /// * `ctrlY2` - Indicates the y coordinate of the second relative control point.
    ///
    /// * `endX` - Indicates the x coordinate of the relative target point.
    ///
    /// * `endY` - Indicates the y coordinate of the relative target point.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_PathRCubicTo(
        path: *mut OH_Drawing_Path,
        ctrlX1: f32,
        ctrlY1: f32,
        ctrlX2: f32,
        ctrlY2: f32,
        endX: f32,
        endY: f32,
    );
    /// Adds a new contour to the path, defined by the rect, and wound in the specified direction.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `path` - Indicates the pointer to an <b>OH_Drawing_Path</b> object.
    ///
    /// * `left` - Indicates the left coordinate of the upper left corner of the rectangle.
    ///
    /// * `top` - Indicates the top coordinate of the upper top corner of the rectangle.
    ///
    /// * `right` - Indicates the right coordinate of the lower right corner of the rectangle.
    ///
    /// * `bottom` - Indicates the bottom coordinate of the lower bottom corner of the rectangle.
    ///
    /// * `pathDirection` - Indicates the path direction.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_PathAddRect(
        path: *mut OH_Drawing_Path,
        left: f32,
        top: f32,
        right: f32,
        bottom: f32,
        pathDirection: OH_Drawing_PathDirection,
    );
    /// Adds a new contour to the path, defined by the rect, and wound in the specified direction.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `path` - Indicates the pointer to an <b>OH_Drawing_Path</b> object.
    ///
    /// * `rect` - Indicates the pointer to an <b>OH_Drawing_Rect</b> object.
    ///
    /// * `pathDirection` - Indicates the path direction.
    ///
    /// * `start` - Indicates initial corner of rect to add.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_PathAddRectWithInitialCorner(
        path: *mut OH_Drawing_Path,
        rect: *const OH_Drawing_Rect,
        pathDirection: OH_Drawing_PathDirection,
        start: u32,
    );
    /// Adds a new contour to the path, defined by the round rect, and wound in the specified direction.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `path` - Indicates the pointer to an <b>OH_Drawing_Path</b> object.
    ///
    /// * `roundRect` - Indicates the pointer to an <b>OH_Drawing_RoundRect</b> object.
    ///
    /// * `pathDirection` - Indicates the path direction.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_PathAddRoundRect(
        path: *mut OH_Drawing_Path,
        roundRect: *const OH_Drawing_RoundRect,
        pathDirection: OH_Drawing_PathDirection,
    );
    /// Adds a oval to the path, defined by the rect, and wound in the specified direction.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `path` - Indicates the pointer to an <b>OH_Drawing_Path</b> object.
    ///
    /// * `rect` - Indicates the pointer to an <b>OH_Drawing_Rect</b> object.
    ///
    /// * `start` - Index of initial point of ellipse.
    ///
    /// * `pathDirection` - Indicates the path direction.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_PathAddOvalWithInitialPoint(
        path: *mut OH_Drawing_Path,
        rect: *const OH_Drawing_Rect,
        start: u32,
        pathDirection: OH_Drawing_PathDirection,
    );
    /// Adds a oval to the path, defined by the rect, and wound in the specified direction.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `path` - Indicates the pointer to an <b>OH_Drawing_Path</b> object.
    ///
    /// * `rect` - Indicates the pointer to an <b>OH_Drawing_Rect</b> object.
    ///
    /// * `pathDirection` - Indicates the path direction.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_PathAddOval(
        path: *mut OH_Drawing_Path,
        rect: *const OH_Drawing_Rect,
        pathDirection: OH_Drawing_PathDirection,
    );
    /// Appends arc to path, as the start of new contour.Arc added is part of ellipse bounded by oval,
    /// from startAngle through sweepAngle. Both startAngle and sweepAngle are measured in degrees, where zero degrees
    /// is aligned with the positive x-axis, and positive sweeps extends arc clockwise.If sweepAngle <= -360, or
    /// sweepAngle >= 360; and startAngle modulo 90 is nearly zero, append oval instead of arc. Otherwise, sweepAngle
    /// values are treated modulo 360, and arc may or may not draw depending on numeric rounding.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `path` - Indicates the pointer to an <b>OH_Drawing_Path</b> object.
    ///
    /// * `rect` - Indicates the pointer to an <b>OH_Drawing_Rect</b> object.
    ///
    /// * `startAngle` - Indicates the starting angle of arc in degrees.
    ///
    /// * `sweepAngle` - Indicates the sweep, in degrees. Positive is clockwise.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_PathAddArc(
        path: *mut OH_Drawing_Path,
        rect: *const OH_Drawing_Rect,
        startAngle: f32,
        sweepAngle: f32,
    );
    /// Appends src path to path, transformed by matrix. Transformed curves may have different verbs,
    /// point, and conic weights.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `path` - Indicates the pointer to an <b>OH_Drawing_Path</b> object.
    ///
    /// * `src` - Indicates the pointer to an <b>OH_Drawing_Path</b> object.
    ///
    /// * `matrix` - Indicates the length of the <b>OH_Drawing_Matrix</b> object.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_PathAddPath(
        path: *mut OH_Drawing_Path,
        src: *const OH_Drawing_Path,
        matrix: *const OH_Drawing_Matrix,
    );
    /// Appends src path to path, transformed by matrix and mode. Transformed curves may have different verbs,
    /// point, and conic weights.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `path` - Indicates the pointer to an <b>OH_Drawing_Path</b> object.
    ///
    /// * `src` - Indicates the pointer to an <b>OH_Drawing_Path</b> object.
    ///
    /// * `matrix` - Indicates the length of the <b>OH_Drawing_Matrix</b> object.
    ///
    /// * `pathAddMode` - Indicates the add path's add mode.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_PathAddPathWithMatrixAndMode(
        path: *mut OH_Drawing_Path,
        src: *const OH_Drawing_Path,
        matrix: *const OH_Drawing_Matrix,
        pathAddMode: OH_Drawing_PathAddMode,
    );
    /// Appends src path to path, transformed by mode. Transformed curves may have different verbs,
    /// point, and conic weights.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `path` - Indicates the pointer to an <b>OH_Drawing_Path</b> object.
    ///
    /// * `src` - Indicates the pointer to an <b>OH_Drawing_Path</b> object, which is Appends src path to path.
    ///
    /// * `pathAddMode` - Indicates the add path's add mode.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_PathAddPathWithMode(
        path: *mut OH_Drawing_Path,
        src: *const OH_Drawing_Path,
        pathAddMode: OH_Drawing_PathAddMode,
    );
    /// Appends src path to path, transformed by offset and mode. Transformed curves may have different verbs,
    /// point, and conic weights.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `path` - Indicates the pointer to an <b>OH_Drawing_Path</b> object.
    ///
    /// * `src` - Indicates the pointer to an <b>OH_Drawing_Path</b> object.
    ///
    /// * `dx` - Indicates offset added to src path x-axis coordinates.
    ///
    /// * `dy` - Indicates offset added to src path y-axis coordinates.
    ///
    /// * `pathAddMode` - Indicates the add path's add mode.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_PathAddPathWithOffsetAndMode(
        path: *mut OH_Drawing_Path,
        src: *const OH_Drawing_Path,
        dx: f32,
        dy: f32,
        pathAddMode: OH_Drawing_PathAddMode,
    );
    /// Adds contour created from point array, adding (count - 1) line segments.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `path` - Indicates the pointer to an <b>OH_Drawing_Path</b> object.
    ///
    /// * `points` - Indicates the point array.
    ///
    /// * `count` - Indicates the size of point array.
    ///
    /// * `isClosed` - Indicates Whether to add lines that connect the end and start.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_PathAddPolygon(
        path: *mut OH_Drawing_Path,
        points: *const OH_Drawing_Point2D,
        count: u32,
        isClosed: bool,
    );
    /// Adds a circle to the path, and wound in the specified direction.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `path` - Indicates the pointer to an <b>OH_Drawing_Path</b> object.
    ///
    /// * `x` - Indicates the x coordinate of the center of the circle.
    ///
    /// * `y` - Indicates the y coordinate of the center of the circle.
    ///
    /// * `radius` - Indicates the radius of the circle.
    ///
    /// * `pathDirection` - Indicates the path direction.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_PathAddCircle(
        path: *mut OH_Drawing_Path,
        x: f32,
        y: f32,
        radius: f32,
        pathDirection: OH_Drawing_PathDirection,
    );
    /// Parses the svg path from the string.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `path` - Indicates the pointer to an <b>OH_Drawing_Path</b> object.
    ///
    /// * `str` - Indicates the string of the SVG path.
    ///
    /// # Returns
    ///
    /// * Returns true if build path is successful, returns false otherwise.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_PathBuildFromSvgString(
        path: *mut OH_Drawing_Path,
        str_: *const ::core::ffi::c_char,
    ) -> bool;
    /// Return the status that point (x, y) is contained by path.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `path` - Indicates the pointer to an <b>OH_Drawing_Path</b> object.
    ///
    /// * `x` - Indicates the x-axis value of containment test.
    ///
    /// * `y` - Indicates the y-axis value of containment test.
    ///
    /// # Returns
    ///
    /// * Returns true if the point (x, y) is contained by path.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_PathContains(path: *mut OH_Drawing_Path, x: f32, y: f32) -> bool;
    /// Transforms verb array, point array, and weight by matrix. transform may change verbs
    /// and increase their number. path is replaced by transformed data.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `path` - Indicates the pointer to an <b>OH_Drawing_Path</b> object.
    ///
    /// * `matrix` - Indicates the pointer to an <b>OH_Drawing_Matrix</b> object.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_PathTransform(path: *mut OH_Drawing_Path, matrix: *const OH_Drawing_Matrix);
    /// Transforms verb array, point array, and weight by matrix.
    /// Transform may change verbs and increase their number.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `src` - Indicates the pointer to an <b>OH_Drawing_Path</b> object.
    ///
    /// * `matrix` - Indicates the pointer to an <b>OH_Drawing_Matrix</b> object.
    ///
    /// * `dst` - Indicates the pointer to an <b>OH_Drawing_Path</b> object.
    ///
    /// * `applyPerspectiveClip` - Indicates whether to apply perspective clip.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_PathTransformWithPerspectiveClip(
        src: *mut OH_Drawing_Path,
        matrix: *const OH_Drawing_Matrix,
        dst: *mut OH_Drawing_Path,
        applyPerspectiveClip: bool,
    );
    /// Sets FillType, the rule used to fill path.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `path` - Indicates the pointer to an <b>OH_Drawing_Path</b> object.
    ///
    /// * `pathFillType` - Indicates the add path's fill type.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_PathSetFillType(
        path: *mut OH_Drawing_Path,
        pathFillType: OH_Drawing_PathFillType,
    );
    /// Gets the length of the current path object.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `path` - Indicates the pointer to an <b>OH_Drawing_Path</b> object.
    ///
    /// * `forceClosed` - Indicates whether free to modify/delete the path after this call.
    ///
    /// # Returns
    ///
    /// * Returns the length of the current path object.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_PathGetLength(path: *mut OH_Drawing_Path, forceClosed: bool) -> f32;
    /// Gets the smallest bounding box that contains the path.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `path` - Indicates the pointer to an <b>OH_Drawing_Path</b> object.
    ///
    /// * `rect` - Indicates the pointer to an <b>OH_Drawing_Rect</b> object.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_PathGetBounds(path: *mut OH_Drawing_Path, rect: *mut OH_Drawing_Rect);
    /// Closes a path. A line segment from the start point to the last point of the path is added.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `path` - Indicates the pointer to an <b>OH_Drawing_Path</b> object.
    ///
    /// Available since API-level: 8
    ///
    /// Version: 1.0
    pub fn OH_Drawing_PathClose(path: *mut OH_Drawing_Path);
    /// Offset path replaces dst.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `path` - Indicates the pointer to an <b>OH_Drawing_Path</b> object.
    ///
    /// * `dst` - Indicates the pointer to an <b>OH_Drawing_Path</b> object.
    ///
    /// * `dx` - Indicates offset added to dst path x-axis coordinates.
    ///
    /// * `dy` - Indicates offset added to dst path y-axis coordinates.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_PathOffset(
        path: *mut OH_Drawing_Path,
        dst: *mut OH_Drawing_Path,
        dx: f32,
        dy: f32,
    );
    /// Resets path data.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `path` - Indicates the pointer to an <b>OH_Drawing_Path</b> object.
    ///
    /// Available since API-level: 8
    ///
    /// Version: 1.0
    pub fn OH_Drawing_PathReset(path: *mut OH_Drawing_Path);
    /// Determines whether the path current contour is closed.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `path` - Indicates the pointer to an <b>OH_Drawing_Path</b> object.
    ///
    /// * `forceClosed` - Whether to close the Path.
    ///
    /// # Returns
    ///
    /// * Returns <b>true</b> if the path current contour is closed; returns <b>false</b> otherwise.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_PathIsClosed(path: *mut OH_Drawing_Path, forceClosed: bool) -> bool;
    /// Gets the position and tangent of the distance from the starting position of the Path.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `path` - Indicates the pointer to an <b>OH_Drawing_Path</b> object.
    ///
    /// * `forceClosed` - Whether to close the Path.
    ///
    /// * `distance` - The distance from the start of the Path.
    ///
    /// * `position` - Sets to the position of distance from the starting position of the Path.
    ///
    /// * `tangent` - Sets to the tangent of distance from the starting position of the Path.
    ///
    /// # Returns
    ///
    /// * Returns <b>true</b> if succeeded; returns <b>false</b> otherwise.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_PathGetPositionTangent(
        path: *mut OH_Drawing_Path,
        forceClosed: bool,
        distance: f32,
        position: *mut OH_Drawing_Point2D,
        tangent: *mut OH_Drawing_Point2D,
    ) -> bool;
    /// Combines two paths.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `path` - Indicates the pointer to an <b>OH_Drawing_Path</b> object.
    ///
    /// * `other` - Indicates the pointer to an <b>OH_Drawing_Path</b> object.
    ///
    /// * `op` - Indicates the operation to apply to combine.
    ///
    /// # Returns
    ///
    /// * Returns <b>true</b> if constructed path is not empty; returns <b>false</b> otherwise.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_PathOp(
        path: *mut OH_Drawing_Path,
        other: *const OH_Drawing_Path,
        op: OH_Drawing_PathOpMode,
    ) -> bool;
    /// Computes the corresponding matrix at the specified distance.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `path` - Indicates the pointer to an <b>OH_Drawing_Path</b> object.
    ///
    /// * `forceClosed` - Whether to close the Path.
    ///
    /// * `distance` - The distance from the start of the Path.
    ///
    /// * `matrix` - Indicates the pointer to an <b>OH_Drawing_Matrix</b> object.
    ///
    /// * `flag` - Indicates what should be returned in the matrix.
    ///
    /// # Returns
    ///
    /// * Returns <b>false</b> if path is nullptr or zero-length;
    /// returns <b>true</b> if path is not nullptr and not zero-length.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_PathGetMatrix(
        path: *mut OH_Drawing_Path,
        forceClosed: bool,
        distance: f32,
        matrix: *mut OH_Drawing_Matrix,
        flag: OH_Drawing_PathMeasureMatrixFlags,
    ) -> bool;
}
