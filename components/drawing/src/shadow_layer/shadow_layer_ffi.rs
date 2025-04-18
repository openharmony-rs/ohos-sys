// automatically generated by rust-bindgen 0.71.1

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use crate::types::*;

extern "C" {
    /// Creates an <b>OH_Drawing_ShadowLayer</b> object.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `blurRadius` - Indicates the blur radius of the shadow.
    ///
    /// * `x` - Indicates the offset point on x-axis.
    ///
    /// * `y` - Indicates the offset point on y-axis.
    ///
    /// * `color` - Indicates the shadow color.
    ///
    /// # Returns
    ///
    /// * Returns the pointer to the <b>OH_Drawing_ShadowLayer</b> object created.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_ShadowLayerCreate(
        blurRadius: f32,
        x: f32,
        y: f32,
        color: u32,
    ) -> *mut OH_Drawing_ShadowLayer;
    /// Destroys an <b>OH_Drawing_ShadowLayer</b> object and reclaims the memory occupied by the object.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `shadowLayer` - Indicates the pointer to an <b>OH_Drawing_ShadowLayer</b> object.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_ShadowLayerDestroy(shadowLayer: *mut OH_Drawing_ShadowLayer);
}
