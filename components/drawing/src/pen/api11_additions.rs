#![allow(non_snake_case)]

use crate::types::{OH_Drawing_Filter, OH_Drawing_Pen, OH_Drawing_ShaderEffect};

extern "C" {
    /** @brief Obtains the alpha of a pen. The alpha is used by the pen to outline a shape.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Pen Indicates the pointer to an <b>OH_Drawing_Pen</b> object.
    @return Returns a 8-bit variable that describes the alpha.
    @since 11
    @version 1.0*/
    pub fn OH_Drawing_PenGetAlpha(arg1: *const OH_Drawing_Pen) -> u8;

    /** @brief Sets the alpha for a pen. The alpha is used by the pen to outline a shape.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Pen Indicates the pointer to an <b>OH_Drawing_Pen</b> object.
    @param alpha Indicates the alpha to set, which is a 8-bit variable.
    @since 11
    @version 1.0*/
    pub fn OH_Drawing_PenSetAlpha(arg1: *mut OH_Drawing_Pen, alpha: u8);
}

extern "C" {
    /** @brief Sets the shaderEffect for a pen.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Pen Indicates the pointer to an <b>OH_Drawing_Pen</b> object.
    @param OH_Drawing_ShaderEffect Indicates the pointer to an <b>OH_Drawing_ShaderEffect</b> object.
    @since 11
    @version 1.0*/
    pub fn OH_Drawing_PenSetShaderEffect(
        arg1: *mut OH_Drawing_Pen,
        arg2: *mut OH_Drawing_ShaderEffect,
    );

    /** @brief Sets the filter for a pen.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Pen Indicates the pointer to an <b>OH_Drawing_Pen</b> object.
    @param OH_Drawing_Filter Indicates the pointer to an <b>OH_Drawing_Filter</b> object.
    @since 11
    @version 1.0*/
    pub fn OH_Drawing_PenSetFilter(arg1: *mut OH_Drawing_Pen, arg2: *mut OH_Drawing_Filter);
}
