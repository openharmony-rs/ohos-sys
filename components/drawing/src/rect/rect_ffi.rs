// automatically generated by rust-bindgen 0.71.1

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use crate::types::*;

#[allow(unused_imports)]
#[cfg(feature = "api-12")]
use crate::error_code::OH_Drawing_ErrorCode;

extern "C" {
    /// Creates an <b>OH_Drawing_Rect</b> object.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `left` - Indicates the left position of the rect.
    ///
    /// * `top` - Indicates the top position of the rect.
    ///
    /// * `right` - Indicates the right position of the rect.
    ///
    /// * `bottom` - Indicates the bottom position of the rect.
    ///
    /// # Returns
    ///
    /// * Returns the pointer to the <b>OH_Drawing_Rect</b> object created.
    ///
    /// Available since API-level: 11
    ///
    /// Version: 1.0
    #[cfg(feature = "api-11")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-11")))]
    pub fn OH_Drawing_RectCreate(
        left: f32,
        top: f32,
        right: f32,
        bottom: f32,
    ) -> *mut OH_Drawing_Rect;
    /// If rect intersects other, sets rect to intersection.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `rect` - Indicates the pointer to an <b>OH_Drawing_Rect</b> object.
    ///
    /// * `other` - Indicates the pointer to an <b>OH_Drawing_Rect</b> object.
    ///
    /// # Returns
    ///
    /// * Returns true if have area in common.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_RectIntersect(
        rect: *mut OH_Drawing_Rect,
        other: *const OH_Drawing_Rect,
    ) -> bool;
    /// Sets rect to the union of rect and other.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `rect` - Indicates the pointer to an <b>OH_Drawing_Rect</b> object.
    ///
    /// * `other` - Indicates the pointer to an <b>OH_Drawing_Rect</b> object.
    ///
    /// # Returns
    ///
    /// * Returns true if rect and other are not nullptr, and other is not empty;
    /// false if rect or other is nullptr, or other is empty.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_RectJoin(rect: *mut OH_Drawing_Rect, other: *const OH_Drawing_Rect) -> bool;
    /// Set the left position of the rect.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `rect` - Indicates the pointer to an <b>OH_Drawing_Rect</b> object.
    ///
    /// * `left` - Indicates the left position of the rect.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_RectSetLeft(rect: *mut OH_Drawing_Rect, left: f32);
    /// Set the top position of the rect.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `rect` - Indicates the pointer to an <b>OH_Drawing_Rect</b> object.
    ///
    /// * `top` - Indicates the top position of the rect.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_RectSetTop(rect: *mut OH_Drawing_Rect, top: f32);
    /// Set the right position of the rect.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `rect` - Indicates the pointer to an <b>OH_Drawing_Rect</b> object.
    ///
    /// * `right` - Indicates the right position of the rect.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_RectSetRight(rect: *mut OH_Drawing_Rect, right: f32);
    /// Set the bottom position of the rect.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `rect` - Indicates the pointer to an <b>OH_Drawing_Rect</b> object.
    ///
    /// * `bottom` - Indicates the bottom position of the rect.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_RectSetBottom(rect: *mut OH_Drawing_Rect, bottom: f32);
    /// Get the left position of the rect.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `rect` - Indicates the pointer to an <b>OH_Drawing_Rect</b> object.
    ///
    /// # Returns
    ///
    /// * Return the left position of the rect.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_RectGetLeft(rect: *mut OH_Drawing_Rect) -> f32;
    /// Get the top position of the rect.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `rect` - Indicates the pointer to an <b>OH_Drawing_Rect</b> object.
    ///
    /// # Returns
    ///
    /// * Return the top position of the rect.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_RectGetTop(rect: *mut OH_Drawing_Rect) -> f32;
    /// Get the right position of the rect.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `rect` - Indicates the pointer to an <b>OH_Drawing_Rect</b> object.
    ///
    /// # Returns
    ///
    /// * Return the right position of the rect.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_RectGetRight(rect: *mut OH_Drawing_Rect) -> f32;
    /// Get the bottom position of the rect.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `rect` - Indicates the pointer to an <b>OH_Drawing_Rect</b> object.
    ///
    /// # Returns
    ///
    /// * Return the bottom position of the rect.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_RectGetBottom(rect: *mut OH_Drawing_Rect) -> f32;
    /// Get the height position of the rect.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `rect` - Indicates the pointer to an <b>OH_Drawing_Rect</b> object.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_RectGetHeight(rect: *mut OH_Drawing_Rect) -> f32;
    /// Get the width position of the rect.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `rect` - Indicates the pointer to an <b>OH_Drawing_Rect</b> object.
    ///
    /// # Returns
    ///
    /// * Returns the width.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_RectGetWidth(rect: *mut OH_Drawing_Rect) -> f32;
    /// Copy the original rectangular object to the destination rectangular object.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `src` - Indicates the pointer to an <b>OH_Drawing_Rect</b> object.
    ///
    /// * `dst` - Indicates the pointer to an <b>OH_Drawing_Rect</b> object.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_RectCopy(src: *mut OH_Drawing_Rect, dst: *mut OH_Drawing_Rect);
    /// Destroys an <b>OH_Drawing_Rect</b> object and reclaims the memory occupied by the object.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `rect` - Indicates the pointer to an <b>OH_Drawing_Rect</b> object.
    ///
    /// Available since API-level: 11
    ///
    /// Version: 1.0
    #[cfg(feature = "api-11")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-11")))]
    pub fn OH_Drawing_RectDestroy(rect: *mut OH_Drawing_Rect);
}
