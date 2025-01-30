// automatically generated by rust-bindgen 0.71.1

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use crate::shader_effect::*;
use crate::types::*;

extern "C" {
    /// Creates an <b>OH_Drawing_ImageFilter</b> object that blurs its input by the separate x and y sigmas.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `sigmaX` - Indicates the Gaussian sigma value for blurring along the x axis.
    ///
    /// * `sigmaY` - Indicates the Gaussian sigma value for blurring along the y axis.
    ///
    /// * `tileMode` - Indicates the tile mode applied at edges.
    ///
    /// * `imageFilter` - Indicates the input filter that is blurred, uses source bitmap if this is null.
    ///
    /// # Returns
    ///
    /// * Returns the pointer to the <b>OH_Drawing_ImageFilter</b> object created.
    /// If nullptr is returned, the creation fails.
    /// The possible cause of the failure is that the available memory is empty.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_ImageFilterCreateBlur(
        sigmaX: f32,
        sigmaY: f32,
        tileMode: OH_Drawing_TileMode,
        imageFilter: *mut OH_Drawing_ImageFilter,
    ) -> *mut OH_Drawing_ImageFilter;
    /// Creates an <b>OH_Drawing_ImageFilter</b> object that applies the color filter to the input.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `colorFilter` - Indicates the color filter that transforms the input image.
    ///
    /// * `imageFilter` - Indicates the input filter, or uses the source bitmap if this is null.
    ///
    /// # Returns
    ///
    /// * Returns the pointer to the <b>OH_Drawing_ImageFilter</b> object created.
    /// If nullptr is returned, the creation fails.
    /// The possible cause of the failure is that the available memory is empty or
    /// a nullptr <b>OH_Drawing_ColorFilter</b> is passed.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_ImageFilterCreateFromColorFilter(
        colorFilter: *mut OH_Drawing_ColorFilter,
        imageFilter: *mut OH_Drawing_ImageFilter,
    ) -> *mut OH_Drawing_ImageFilter;
    /// Destroys an <b>OH_Drawing_ImageFilter</b> object and reclaims the memory occupied by the object.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `imageFilter` - Indicates the pointer to an <b>OH_Drawing_ImageFilter</b> object.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_ImageFilterDestroy(imageFilter: *mut OH_Drawing_ImageFilter);
}
