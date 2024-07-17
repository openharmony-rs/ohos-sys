#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::drawing::types::{OH_Drawing_Corner_Radii, OH_Drawing_RoundRect};

impl OH_Drawing_CornerPos {
    /// Index of top-left corner radii.
    pub const CORNER_POS_TOP_LEFT: OH_Drawing_CornerPos = OH_Drawing_CornerPos(0);
}
impl OH_Drawing_CornerPos {
    /// Index of top-right corner radii.
    pub const CORNER_POS_TOP_RIGHT: OH_Drawing_CornerPos = OH_Drawing_CornerPos(1);
}
impl OH_Drawing_CornerPos {
    /// Index of bottom-right corner radii.
    pub const CORNER_POS_BOTTOM_RIGHT: OH_Drawing_CornerPos = OH_Drawing_CornerPos(2);
}
impl OH_Drawing_CornerPos {
    /// Index of bottom-left corner radii.
    pub const CORNER_POS_BOTTOM_LEFT: OH_Drawing_CornerPos = OH_Drawing_CornerPos(3);
}
#[repr(transparent)]
/** @brief Enumerates of corner radii position.

@since 12
@version 1.0*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct OH_Drawing_CornerPos(pub ::core::ffi::c_uint);

extern "C" {
    /** @brief Sets the radiusX and radiusY for a specific corner position.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_RoundRect Indicates the pointer to an <b>OH_Drawing_Rect</b> object.
    @param pos Indicates the corner radii position.
    @param OH_Drawing_Corner_Radii Indicates the corner radii on x-axis and y-axis.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_RoundRectSetCorner(
        arg1: *mut OH_Drawing_RoundRect,
        pos: OH_Drawing_CornerPos,
        arg2: OH_Drawing_Corner_Radii,
    );
    /** @brief Gets an <b>OH_Drawing_Corner_Radii</b> struct, the point is round corner radiusX and radiusY.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_RoundRect Indicates the pointer to an <b>OH_Drawing_RoundRect</b> object.
    @param pos Indicates the corner radii position.
    @return Returns the corner radii of <b>OH_Drawing_Corner_Radii</b> struct.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_RoundRectGetCorner(
        arg1: *mut OH_Drawing_RoundRect,
        pos: OH_Drawing_CornerPos,
    ) -> OH_Drawing_Corner_Radii;
}
