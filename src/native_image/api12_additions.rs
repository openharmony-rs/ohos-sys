#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use super::OH_NativeImage;

extern "C" {
    /** @brief Obtains the transform matrix of the texture image by producer transform type.\n

    @syscap SystemCapability.Graphic.Graphic2D.NativeImage
    @param image Indicates the pointer to a <b>OH_NativeImage</b> instance.
    @param matrix Indicates the retrieved 4*4 transform matrix .
    @return 0 - Success.
        40001000 - image is NULL.
    @since 12
    @version 1.0*/
    pub fn OH_NativeImage_GetTransformMatrixV2(image: *mut OH_NativeImage, matrix: *mut f32)
        -> i32;
}
