#![allow(non_snake_case)]

use crate::drawing::types::{OH_Drawing_Brush, OH_Drawing_Filter, OH_Drawing_ShaderEffect};

extern "C" {
    /** @brief Obtains the alpha of a brush. The alpha is used by the brush to fill in a shape.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Brush Indicates the pointer to an <b>OH_Drawing_Brush</b> object.
    @return Returns a 8-bit variable that describes the alpha.
    @since 11
    @version 1.0*/
    pub fn OH_Drawing_BrushGetAlpha(arg1: *const OH_Drawing_Brush) -> u8;

    /** @brief Sets the alpha for a brush. The alpha will be used by the brush to fill in a shape.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Brush Indicates the pointer to an <b>OH_Drawing_Brush</b> object.
    @param alpha Indicates the alpha to set, which is a 8-bit variable.
    @since 11
    @version 1.0*/
    pub fn OH_Drawing_BrushSetAlpha(arg1: *mut OH_Drawing_Brush, alpha: u8);

    /** @brief Sets the shaderEffect for a brush.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Brush Indicates the pointer to an <b>OH_Drawing_Brush</b> object.
    @param OH_Drawing_ShaderEffect Indicates the pointer to an <b>OH_Drawing_ShaderEffect</b> object.
    @since 11
    @version 1.0*/
    pub fn OH_Drawing_BrushSetShaderEffect(
        arg1: *mut OH_Drawing_Brush,
        arg2: *mut OH_Drawing_ShaderEffect,
    );

    /** @brief Sets the filter for a brush.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Brush Indicates the pointer to an <b>OH_Drawing_Brush</b> object.
    @param OH_Drawing_Filter Indicates the pointer to an <b>OH_Drawing_Filter</b> object.
    @since 11
    @version 1.0*/
    pub fn OH_Drawing_BrushSetFilter(arg1: *mut OH_Drawing_Brush, arg2: *mut OH_Drawing_Filter);
}
