#![allow(non_snake_case)]

use crate::drawing::shader_effect::OH_Drawing_TileMode;
use crate::drawing::types::{
    OH_Drawing_Image, OH_Drawing_Matrix, OH_Drawing_Point2D, OH_Drawing_SamplingOptions,
    OH_Drawing_ShaderEffect,
};

extern "C" {
    /** @brief Creates an <b>OH_Drawing_ShaderEffect</b> that generates a shader with single color.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param color Indicates the color used by the shader.
    @return Returns the pointer to the <b>OH_Drawing_ShaderEffect</b> object created.
            If nullptr is returned, the creation fails.
            The possible cause of the failure is that the available memory is empty.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_ShaderEffectCreateColorShader(color: u32) -> *mut OH_Drawing_ShaderEffect;
    /** @brief Creates an <b>OH_Drawing_ShaderEffect</b> that generates a linear gradient between the two specified points.

     @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
     @param startPt Indicates the start point for the gradient.
     @param endPt Indicates the end point for the gradient.
     @param colors Indicates the colors to be distributed between the two points.
     @param pos Indicates the relative position of each corresponding color in the colors array.
                If pos is nullptr, the colors are evenly distributed between the start and end point.
     @param size Indicates the number of colors and pos(if pos is not nullptr).
     @param OH_Drawing_TileMode Indicates the tile mode.
     @param OH_Drawing_Matrix Indicates the pointer to an <b>OH_Drawing_Matrix</b> object,
    which represents the local matrix of the created <b>OH_Drawing_ShaderEffect</b> object.
    If matrix is nullptr, defaults to the identity matrix.
     @return Returns the pointer to the <b>OH_Drawing_ShaderEffect</b> object created.
             If nullptr is returned, the creation fails.
             The possible cause of the failure is any of startPt, endPt, colors and pos is nullptr.
     @since 12
     @version 1.0*/
    pub fn OH_Drawing_ShaderEffectCreateLinearGradientWithLocalMatrix(
        startPt: *const OH_Drawing_Point2D,
        endPt: *const OH_Drawing_Point2D,
        colors: *const u32,
        pos: *const f32,
        size: u32,
        arg1: OH_Drawing_TileMode,
        arg2: *const OH_Drawing_Matrix,
    ) -> *mut OH_Drawing_ShaderEffect;
    /** @brief Creates an <b>OH_Drawing_ShaderEffect</b> that generates a radial gradient given the center and radius.

     @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
     @param centerPt Indicates the center of the circle for the gradient.
     @param radius Indicates the radius of the circle for this gradient.
     @param colors Indicates the colors to be distributed between the two points.
     @param pos Indicates the relative position of each corresponding color in the colors array.
     @param size Indicates the number of colors and pos.
     @param OH_Drawing_TileMode Indicates the tile mode.
     @param OH_Drawing_Matrix Indicates the pointer to an <b>OH_Drawing_Matrix</b> object,
    which represents the local matrix of the created <b>OH_Drawing_ShaderEffect</b> object.
    If matrix is nullptr, defaults to the identity matrix.
     @return Returns the pointer to the <b>OH_Drawing_ShaderEffect</b> object created.
             If nullptr is returned, the creation fails.
             The possible cause of the failure is any of centerPt, colors and pos is nullptr.
     @since 12
     @version 1.0*/
    pub fn OH_Drawing_ShaderEffectCreateRadialGradientWithLocalMatrix(
        centerPt: *const OH_Drawing_Point2D,
        radius: f32,
        colors: *const u32,
        pos: *const f32,
        size: u32,
        arg1: OH_Drawing_TileMode,
        arg2: *const OH_Drawing_Matrix,
    ) -> *mut OH_Drawing_ShaderEffect;
    /** @brief Creates an <b>OH_Drawing_ShaderEffect</b> that generates a image shader.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Image Indicates the pointer to an <b>OH_Drawing_Image</b> object.
    @param tileX Indicates the tileX.
    @param tileY Indicates the tileY.
    @param OH_Drawing_SamplingOptions Indicates the pointer to an <b>OH_Drawing_SamplingOptions</b> object.
    @param OH_Drawing_Matrix Indicates the pointer to an <b>OH_Drawing_Matrix</b> object.
                             If matrix is nullptr, defaults to the identity matrix.
    @return Returns the pointer to the <b>OH_Drawing_ShaderEffect</b> object created.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_ShaderEffectCreateImageShader(
        arg1: *mut OH_Drawing_Image,
        tileX: OH_Drawing_TileMode,
        tileY: OH_Drawing_TileMode,
        arg2: *const OH_Drawing_SamplingOptions,
        arg3: *const OH_Drawing_Matrix,
    ) -> *mut OH_Drawing_ShaderEffect;
    /** @brief Creates an <b>OH_Drawing_ShaderEffect</b> that generates a conical gradient given two circles.

     @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
     @param startPt Indicates the center of the start circle for the gradient.
     @param startRadius Indicates the radius of the start circle for this gradient.
     @param endPt Indicates the center of the start circle for the gradient.
     @param endRadius Indicates the radius of the start circle for this gradient.
     @param colors Indicates the colors to be distributed between the two points.
     @param pos Indicates the relative position of each corresponding color in the colors array.
     @param size Indicates the number of colors and pos.
     @param OH_Drawing_TileMode Indicates the tile mode.
     @param OH_Drawing_Matrix Indicates the pointer to an <b>OH_Drawing_Matrix</b> object,
    which represents the local matrix of the created <b>OH_Drawing_ShaderEffect</b> object.
    If matrix is nullptr, defaults to the identity matrix.
     @return Returns the pointer to the <b>OH_Drawing_ShaderEffect</b> object created.
             If nullptr is returned, the creation fails.
             The possible cause of the failure is any of startPt, endPt, colors and pos is nullptr.
     @since 12
     @version 1.0*/
    pub fn OH_Drawing_ShaderEffectCreateTwoPointConicalGradient(
        startPt: *const OH_Drawing_Point2D,
        startRadius: f32,
        endPt: *const OH_Drawing_Point2D,
        endRadius: f32,
        colors: *const u32,
        pos: *const f32,
        size: u32,
        arg1: OH_Drawing_TileMode,
        arg2: *const OH_Drawing_Matrix,
    ) -> *mut OH_Drawing_ShaderEffect;
}
