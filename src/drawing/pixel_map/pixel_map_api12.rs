/* automatically generated by rust-bindgen 0.69.4 */

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use crate::drawing::types::*;

/** @brief Introduces the native pixel map information defined by image framework.
@since 12
@version 1.0*/
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct NativePixelMap_ {
    _unused: [u8; 0],
}
/** @brief Introduces the native pixel map information defined by image framework.
@since 12
@version 1.0*/
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OH_PixelmapNative {
    _unused: [u8; 0],
}
extern "C" {
    /** @brief Gets an <b>OH_Drawing_PixelMap</b> object.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param NativePixelMap_ Indicates a pointer to an native pixelmap supported by image framework.
    @return Returns the pointer to the <b>OH_Drawing_PixelMap</b> object.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_PixelMapGetFromNativePixelMap(
        arg1: *mut NativePixelMap_,
    ) -> *mut OH_Drawing_PixelMap;
    /** @brief Gets an <b>OH_Drawing_PixelMap</b> object.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_PixelmapNative Indicates a pointer to the <b>OH_PixelmapNative</b> object supported by image framework.
    @return Returns the pointer to the <b>OH_Drawing_PixelMap</b> object.
            If nullptr is returned, the get operation fails.
            The possible cause of the failure is that a nullptr is passed.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_PixelMapGetFromOhPixelMapNative(
        arg1: *mut OH_PixelmapNative,
    ) -> *mut OH_Drawing_PixelMap;
    /** @brief Dissolves the relationship between <b>OH_Drawing_PixelMap</b> object and <b>NativePixelMap_</b> or
    <b>OH_PixelmapNative</b> which is build by 'GetFrom' function.

     @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
     @param OH_Drawing_PixelMap Indicates a pointer to the <b>OH_Drawing_PixelMap</b>.
     @since 12
     @version 1.0*/
    pub fn OH_Drawing_PixelMapDissolve(arg1: *mut OH_Drawing_PixelMap);
}