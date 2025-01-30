// automatically generated by rust-bindgen 0.71.1

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use crate::types::*;

#[cfg(feature = "api-11")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-11")))]
impl OH_Drawing_TileMode {
    /// Replicate the edge color if the shader effect draws outside of its original bounds.
    pub const CLAMP: OH_Drawing_TileMode = OH_Drawing_TileMode(0);
}
#[cfg(feature = "api-11")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-11")))]
impl OH_Drawing_TileMode {
    /// Repeat the shader effect image horizontally and vertically.
    pub const REPEAT: OH_Drawing_TileMode = OH_Drawing_TileMode(1);
}
#[cfg(feature = "api-11")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-11")))]
impl OH_Drawing_TileMode {
    /// Repeat the shader effect image horizontally and vertically, alternating mirror images
    /// so that adjacent images always seam.
    pub const MIRROR: OH_Drawing_TileMode = OH_Drawing_TileMode(2);
}
#[cfg(feature = "api-11")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-11")))]
impl OH_Drawing_TileMode {
    /// Only draw within the original domain, return transparent-black everywhere else.
    pub const DECAL: OH_Drawing_TileMode = OH_Drawing_TileMode(3);
}
#[repr(transparent)]
/// Enumerates tile mode.
///
///
/// Available since API-level: 11
///
/// Version: 1.0
#[cfg(feature = "api-11")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-11")))]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct OH_Drawing_TileMode(pub ::core::ffi::c_uint);
extern "C" {
    /// Creates an <b>OH_Drawing_ShaderEffect</b> that generates a shader with single color.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `color` - Indicates the color used by the shader.
    ///
    /// # Returns
    ///
    /// * Returns the pointer to the <b>OH_Drawing_ShaderEffect</b> object created.
    /// If nullptr is returned, the creation fails.
    /// The possible cause of the failure is that the available memory is empty.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_ShaderEffectCreateColorShader(color: u32) -> *mut OH_Drawing_ShaderEffect;
    /// Creates an <b>OH_Drawing_ShaderEffect</b> that generates a linear gradient between the two specified points.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `startPt` - Indicates the start point for the gradient.
    ///
    /// * `endPt` - Indicates the end point for the gradient.
    ///
    /// * `colors` - Indicates the colors to be distributed between the two points.
    ///
    /// * `pos` - Indicates the relative position of each corresponding color in the colors array.
    ///
    /// * `size` - Indicates the number of colors and pos.
    ///
    /// * `tileMode` - Indicates the tile mode.
    ///
    /// # Returns
    ///
    /// * Returns the pointer to the <b>OH_Drawing_ShaderEffect</b> object created.
    ///
    /// Available since API-level: 11
    ///
    /// Version: 1.0
    #[cfg(feature = "api-11")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-11")))]
    pub fn OH_Drawing_ShaderEffectCreateLinearGradient(
        startPt: *const OH_Drawing_Point,
        endPt: *const OH_Drawing_Point,
        colors: *const u32,
        pos: *const f32,
        size: u32,
        tileMode: OH_Drawing_TileMode,
    ) -> *mut OH_Drawing_ShaderEffect;
    /// Creates an <b>OH_Drawing_ShaderEffect</b> that generates a linear gradient between the two specified points.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `startPt` - Indicates the start point for the gradient.
    ///
    /// * `endPt` - Indicates the end point for the gradient.
    ///
    /// * `colors` - Indicates the colors to be distributed between the two points.
    ///
    /// * `pos` - Indicates the relative position of each corresponding color in the colors array.
    /// If pos is nullptr, the colors are evenly distributed between the start and end point.
    ///
    /// * `size` - Indicates the number of colors and pos(if pos is not nullptr).
    ///
    /// * `tileMode` - Indicates the tile mode.
    ///
    /// * `matrix` - Indicates the pointer to an <b>OH_Drawing_Matrix</b> object,
    /// which represents the local matrix of the created <b>OH_Drawing_ShaderEffect</b> object.
    /// If matrix is nullptr, defaults to the identity matrix.
    ///
    /// # Returns
    ///
    /// * Returns the pointer to the <b>OH_Drawing_ShaderEffect</b> object created.
    /// If nullptr is returned, the creation fails.
    /// The possible cause of the failure is any of startPt, endPt, colors and pos is nullptr.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_ShaderEffectCreateLinearGradientWithLocalMatrix(
        startPt: *const OH_Drawing_Point2D,
        endPt: *const OH_Drawing_Point2D,
        colors: *const u32,
        pos: *const f32,
        size: u32,
        tileMode: OH_Drawing_TileMode,
        matrix: *const OH_Drawing_Matrix,
    ) -> *mut OH_Drawing_ShaderEffect;
    /// Creates an <b>OH_Drawing_ShaderEffect</b> that generates a radial gradient given the center and radius.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `centerPt` - Indicates the center of the circle for the gradient.
    ///
    /// * `radius` - Indicates the radius of the circle for this gradient.
    ///
    /// * `colors` - Indicates the colors to be distributed between the two points.
    ///
    /// * `pos` - Indicates the relative position of each corresponding color in the colors array.
    ///
    /// * `size` - Indicates the number of colors and pos.
    ///
    /// * `tileMode` - Indicates the tile mode.
    ///
    /// # Returns
    ///
    /// * Returns the pointer to the <b>OH_Drawing_ShaderEffect</b> object created.
    ///
    /// Available since API-level: 11
    ///
    /// Version: 1.0
    #[cfg(feature = "api-11")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-11")))]
    pub fn OH_Drawing_ShaderEffectCreateRadialGradient(
        centerPt: *const OH_Drawing_Point,
        radius: f32,
        colors: *const u32,
        pos: *const f32,
        size: u32,
        tileMode: OH_Drawing_TileMode,
    ) -> *mut OH_Drawing_ShaderEffect;
    /// Creates an <b>OH_Drawing_ShaderEffect</b> that generates a radial gradient given the center and radius.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `centerPt` - Indicates the center of the circle for the gradient.
    ///
    /// * `radius` - Indicates the radius of the circle for this gradient.
    ///
    /// * `colors` - Indicates the colors to be distributed between the two points.
    ///
    /// * `pos` - Indicates the relative position of each corresponding color in the colors array.
    ///
    /// * `size` - Indicates the number of colors and pos.
    ///
    /// * `tileMode` - Indicates the tile mode.
    ///
    /// * `matrix` - Indicates the pointer to an <b>OH_Drawing_Matrix</b> object,
    /// which represents the local matrix of the created <b>OH_Drawing_ShaderEffect</b> object.
    /// If matrix is nullptr, defaults to the identity matrix.
    ///
    /// # Returns
    ///
    /// * Returns the pointer to the <b>OH_Drawing_ShaderEffect</b> object created.
    /// If nullptr is returned, the creation fails.
    /// The possible cause of the failure is any of centerPt, colors and pos is nullptr.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_ShaderEffectCreateRadialGradientWithLocalMatrix(
        centerPt: *const OH_Drawing_Point2D,
        radius: f32,
        colors: *const u32,
        pos: *const f32,
        size: u32,
        tileMode: OH_Drawing_TileMode,
        matrix: *const OH_Drawing_Matrix,
    ) -> *mut OH_Drawing_ShaderEffect;
    /// Creates an <b>OH_Drawing_ShaderEffect</b> that generates a sweep gradient given a center.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `centerPt` - Indicates the center of the circle for the gradient.
    ///
    /// * `colors` - Indicates the colors to be distributed between the two points.
    ///
    /// * `pos` - Indicates the relative position of each corresponding color in the colors array.
    ///
    /// * `size` - Indicates the number of colors and pos.
    ///
    /// * `tileMode` - Indicates the tile mode.
    ///
    /// # Returns
    ///
    /// * Returns the pointer to the <b>OH_Drawing_ShaderEffect</b> object created.
    ///
    /// Available since API-level: 11
    ///
    /// Version: 1.0
    #[cfg(feature = "api-11")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-11")))]
    pub fn OH_Drawing_ShaderEffectCreateSweepGradient(
        centerPt: *const OH_Drawing_Point,
        colors: *const u32,
        pos: *const f32,
        size: u32,
        tileMode: OH_Drawing_TileMode,
    ) -> *mut OH_Drawing_ShaderEffect;
    /// Creates an <b>OH_Drawing_ShaderEffect</b> that generates a image shader.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `image` - Indicates the pointer to an <b>OH_Drawing_Image</b> object.
    ///
    /// * `tileX` - Indicates the tileX.
    ///
    /// * `tileY` - Indicates the tileY.
    ///
    /// * `samplingOptions` - Indicates the pointer to an <b>OH_Drawing_SamplingOptions</b> object.
    ///
    /// * `matrix` - Indicates the pointer to an <b>OH_Drawing_Matrix</b> object.
    /// If matrix is nullptr, defaults to the identity matrix.
    ///
    /// # Returns
    ///
    /// * Returns the pointer to the <b>OH_Drawing_ShaderEffect</b> object created.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_ShaderEffectCreateImageShader(
        image: *mut OH_Drawing_Image,
        tileX: OH_Drawing_TileMode,
        tileY: OH_Drawing_TileMode,
        samplingOptions: *const OH_Drawing_SamplingOptions,
        matrix: *const OH_Drawing_Matrix,
    ) -> *mut OH_Drawing_ShaderEffect;
    /// Creates an <b>OH_Drawing_ShaderEffect</b> that generates a conical gradient given two circles.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `startPt` - Indicates the center of the start circle for the gradient.
    ///
    /// * `startRadius` - Indicates the radius of the start circle for this gradient.
    ///
    /// * `endPt` - Indicates the center of the start circle for the gradient.
    ///
    /// * `endRadius` - Indicates the radius of the start circle for this gradient.
    ///
    /// * `colors` - Indicates the colors to be distributed between the two points.
    ///
    /// * `pos` - Indicates the relative position of each corresponding color in the colors array.
    ///
    /// * `size` - Indicates the number of colors and pos.
    ///
    /// * `tileMode` - Indicates the tile mode.
    ///
    /// * `matrix` - Indicates the pointer to an <b>OH_Drawing_Matrix</b> object,
    /// which represents the local matrix of the created <b>OH_Drawing_ShaderEffect</b> object.
    /// If matrix is nullptr, defaults to the identity matrix.
    ///
    /// # Returns
    ///
    /// * Returns the pointer to the <b>OH_Drawing_ShaderEffect</b> object created.
    /// If nullptr is returned, the creation fails.
    /// The possible cause of the failure is any of startPt, endPt, colors and pos is nullptr.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_ShaderEffectCreateTwoPointConicalGradient(
        startPt: *const OH_Drawing_Point2D,
        startRadius: f32,
        endPt: *const OH_Drawing_Point2D,
        endRadius: f32,
        colors: *const u32,
        pos: *const f32,
        size: u32,
        tileMode: OH_Drawing_TileMode,
        matrix: *const OH_Drawing_Matrix,
    ) -> *mut OH_Drawing_ShaderEffect;
    /// Destroys an <b>OH_Drawing_ShaderEffect</b> object and reclaims the memory occupied by the object.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `shaderEffect` - Indicates the pointer to an <b>OH_Drawing_ShaderEffect</b> object.
    ///
    /// Available since API-level: 11
    ///
    /// Version: 1.0
    #[cfg(feature = "api-11")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-11")))]
    pub fn OH_Drawing_ShaderEffectDestroy(shaderEffect: *mut OH_Drawing_ShaderEffect);
}
