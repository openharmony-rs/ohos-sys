// automatically generated by rust-bindgen 0.71.1

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use crate::types::*;

extern "C" {
    /// Creates an <b>OH_Drawing_Point</b> object.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `x` - Indicates the x-axis coordinates of the point.
    ///
    /// * `y` - Indicates the y-axis coordinates of the point.
    ///
    /// # Returns
    ///
    /// * Returns the pointer to the <b>OH_Drawing_Point</b> object created.
    ///
    /// Available since API-level: 11
    ///
    /// Version: 1.0
    #[cfg(feature = "api-11")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-11")))]
    pub fn OH_Drawing_PointCreate(x: f32, y: f32) -> *mut OH_Drawing_Point;
    /// Gets the x-axis coordinate of the point.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `point` - Indicates the pointer to an <b>OH_Drawing_Point</b> object.
    ///
    /// * `x` - Indicates the x-axis coordinate of the point.
    ///
    /// # Returns
    ///
    /// * Returns the error code.
    /// Returns [`OH_DRAWING_SUCCESS`] if the operation is successful.
    /// Returns [`OH_DRAWING_ERROR_INVALID_PARAMETER`] if point or x is nullptr.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_PointGetX(
        point: *const OH_Drawing_Point,
        x: *mut f32,
    ) -> crate::error_code::DrawingResult;
    /// Gets the y-axis coordinate of the point.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `point` - Indicates the pointer to an <b>OH_Drawing_Point</b> object.
    ///
    /// * `y` - Indicates the y-axis coordinate of the point.
    ///
    /// # Returns
    ///
    /// * Returns the error code.
    /// Returns [`OH_DRAWING_SUCCESS`] if the operation is successful.
    /// Returns [`OH_DRAWING_ERROR_INVALID_PARAMETER`] if point or y is nullptr.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_PointGetY(
        point: *const OH_Drawing_Point,
        y: *mut f32,
    ) -> crate::error_code::DrawingResult;
    /// Sets the x-axis and y-axis coordinates of the point.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `point` - Indicates the pointer to an <b>OH_Drawing_Point</b> object.
    ///
    /// * `x` - Indicates the x-axis coordinate of the point.
    ///
    /// * `y` - Indicates the y-axis coordinate of the point.
    ///
    /// # Returns
    ///
    /// * Returns the error code.
    /// Returns [`OH_DRAWING_SUCCESS`] if the operation is successful.
    /// Returns [`OH_DRAWING_ERROR_INVALID_PARAMETER`] if point is nullptr.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_PointSet(
        point: *mut OH_Drawing_Point,
        x: f32,
        y: f32,
    ) -> crate::error_code::DrawingResult;
    /// Destroys an <b>OH_Drawing_Point</b> object and reclaims the memory occupied by the object.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `point` - Indicates the pointer to an <b>OH_Drawing_Point</b> object.
    ///
    /// Available since API-level: 11
    ///
    /// Version: 1.0
    #[cfg(feature = "api-11")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-11")))]
    pub fn OH_Drawing_PointDestroy(point: *mut OH_Drawing_Point);
}
