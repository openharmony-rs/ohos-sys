#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::error_code::OH_Drawing_ErrorCode;
use crate::types::*;

extern "C" {
    /** @brief Gets the x-axis coordinate of the point.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param point Indicates the pointer to an <b>OH_Drawing_Point</b> object.
    @param x Indicates the x-axis coordinate of the point.
    @return Returns the error code.
            Returns {@link OH_DRAWING_SUCCESS} if the operation is successful.
            Returns {@link OH_DRAWING_ERROR_INVALID_PARAMETER} if point or x is nullptr.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_PointGetX(
        point: *const OH_Drawing_Point,
        x: *mut f32,
    ) -> OH_Drawing_ErrorCode;
    /** @brief Gets the y-axis coordinate of the point.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param point Indicates the pointer to an <b>OH_Drawing_Point</b> object.
    @param y Indicates the y-axis coordinate of the point.
    @return Returns the error code.
            Returns {@link OH_DRAWING_SUCCESS} if the operation is successful.
            Returns {@link OH_DRAWING_ERROR_INVALID_PARAMETER} if point or y is nullptr.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_PointGetY(
        point: *const OH_Drawing_Point,
        y: *mut f32,
    ) -> OH_Drawing_ErrorCode;
    /** @brief Sets the x-axis and y-axis coordinates of the point.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param point Indicates the pointer to an <b>OH_Drawing_Point</b> object.
    @param x Indicates the x-axis coordinate of the point.
    @param y Indicates the y-axis coordinate of the point.
    @return Returns the error code.
            Returns {@link OH_DRAWING_SUCCESS} if the operation is successful.
            Returns {@link OH_DRAWING_ERROR_INVALID_PARAMETER} if point is nullptr.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_PointSet(
        point: *mut OH_Drawing_Point,
        x: f32,
        y: f32,
    ) -> OH_Drawing_ErrorCode;
}
