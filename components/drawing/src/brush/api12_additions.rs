#![allow(non_snake_case)]

use crate::types::{
    OH_Drawing_BlendMode, OH_Drawing_Brush, OH_Drawing_Filter, OH_Drawing_ShadowLayer,
};

extern "C" {
    /** @brief Creates an <b>OH_Drawing_Brush</b> copy object.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Brush Indicates the pointer to an <b>OH_Drawing_Brush</b> object.
    @return Returns the pointer to the <b>OH_Drawing_Brush</b> object created.
            If nullptr is returned, the creation fails.
            The possible cause of the failure is that the available memory is empty or a nullptr is passed.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_BrushCopy(arg1: *mut OH_Drawing_Brush) -> *mut OH_Drawing_Brush;
    /** @brief Sets the shadowLayer for a brush.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Brush Indicates the pointer to an <b>OH_Drawing_Brush</b> object.
    @param OH_Drawing_ShadowLayer Indicates the pointer to an <b>OH_Drawing_ShadowLayer</b> object.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_BrushSetShadowLayer(
        arg1: *mut OH_Drawing_Brush,
        arg2: *mut OH_Drawing_ShadowLayer,
    );
    /** @brief Gets the filter from a brush.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Brush Indicates the pointer to an <b>OH_Drawing_Brush</b> object.
    @param OH_Drawing_Filter Indicates the pointer to an <b>OH_Drawing_Filter</b> object.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_BrushGetFilter(arg1: *mut OH_Drawing_Brush, arg2: *mut OH_Drawing_Filter);
    /** @brief Sets a blender that implements the specified blendmode enum for a brush.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Pen Indicates the pointer to an <b>OH_Drawing_Brush</b> object.
    @param OH_Drawing_BlendMode Indicates the blend mode.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_BrushSetBlendMode(arg1: *mut OH_Drawing_Brush, arg2: OH_Drawing_BlendMode);
    /** @brief Resets all brush contents to their initial values.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Brush Indicates the pointer to an <b>OH_Drawing_Brush</b> object.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_BrushReset(arg1: *mut OH_Drawing_Brush);
}
