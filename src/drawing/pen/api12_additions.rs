#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::drawing::types::{
    OH_Drawing_BlendMode, OH_Drawing_Filter, OH_Drawing_Matrix, OH_Drawing_Path,
    OH_Drawing_PathEffect, OH_Drawing_Pen, OH_Drawing_Rect, OH_Drawing_ShadowLayer,
};

extern "C" {
    /** @brief Creates an <b>OH_Drawing_Pen</b> copy object.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Pen Indicates the pointer to an <b>OH_Drawing_Pen</b> object.
    @return Returns the pointer to the <b>OH_Drawing_Pen</b> object created.
            If nullptr is returned, the creation fails.
            The possible cause of the failure is that the available memory is empty or a nullptr is passed.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_PenCopy(arg1: *mut OH_Drawing_Pen) -> *mut OH_Drawing_Pen;
    /** @brief Sets the shadowLayer for a pen.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Pen Indicates the pointer to an <b>OH_Drawing_Pen</b> object.
    @param OH_Drawing_ShadowLayer Indicates the pointer to an <b>OH_Drawing_ShadowLayer</b> object.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_PenSetShadowLayer(
        arg1: *mut OH_Drawing_Pen,
        arg2: *mut OH_Drawing_ShadowLayer,
    );
    /** @brief Sets the pathEffect for a pen.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Pen Indicates the pointer to an <b>OH_Drawing_Pen</b> object.
    @param OH_Drawing_PathEffect Indicates the pointer to an <b>OH_Drawing_PathEffect</b> object.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_PenSetPathEffect(arg1: *mut OH_Drawing_Pen, arg2: *mut OH_Drawing_PathEffect);
    /** @brief Gets the filter from a pen.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Pen Indicates the pointer to an <b>OH_Drawing_Pen</b> object.
    @param OH_Drawing_Filter Indicates the pointer to an <b>OH_Drawing_Filter</b> object.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_PenGetFilter(arg1: *mut OH_Drawing_Pen, arg2: *mut OH_Drawing_Filter);
    /** @brief Sets a blender that implements the specified blendmode enum for a pen.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Pen Indicates the pointer to an <b>OH_Drawing_Pen</b> object.
    @param OH_Drawing_BlendMode Indicates the blend mode.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_PenSetBlendMode(arg1: *mut OH_Drawing_Pen, arg2: OH_Drawing_BlendMode);
    /** @brief Gets the filled equivalent of the src path.

     @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
     @param OH_Drawing_Pen Indicates the pointer to an <b>OH_Drawing_Pen</b> object.
     @param src Indicates the Path read to create a filled version.
     @param dst Indicates the resulting Path.
     @param OH_Drawing_Rect Indicates the pointer to an <b>OH_Drawing_Rect</b> object that limits the PathEffect area if
    Pen has PathEffect.
     @param OH_Drawing_Matrix Indicates the pointer to an <b>OH_Drawing_Matrix</b> object that tranfomation applied to
    PathEffect if Pen has PathEffect.
     @return Returns true if get successes; false if get fails.
     @since 12
     @version 1.0*/
    pub fn OH_Drawing_PenGetFillPath(
        arg1: *mut OH_Drawing_Pen,
        src: *const OH_Drawing_Path,
        dst: *mut OH_Drawing_Path,
        arg2: *const OH_Drawing_Rect,
        arg3: *const OH_Drawing_Matrix,
    ) -> bool;
    /** @brief Resets all pen contents to their initial values.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Pen Indicates the pointer to an <b>OH_Drawing_Pen</b> object.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_PenReset(arg1: *mut OH_Drawing_Pen);
}
