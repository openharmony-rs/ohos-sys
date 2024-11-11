#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use super::OH_NativeImage;
#[cfg(feature = "native_window")]
use crate::native_window::OHNativeWindowBuffer;

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

#[cfg(feature = "native_window")]
extern "C" {
    /** @brief Acquire an <b>OHNativeWindowBuffer</b> through an <b>OH_NativeImage</b> instance for content consumer.\n
    This method can not be used at the same time with <b>OH_NativeImage_UpdateSurfaceImage</b>.\n
    This method will create an <b>OHNativeWindowBuffer</b>.\n
    When using <b>OHNativeWindowBuffer</b>, need to increase its reference count
    by <b>OH_NativeWindow_NativeObjectReference</b>.\n
    When the <b>OHNativeWindowBuffer</b> is used up, its reference count needs to be decremented
    by <b>OH_NativeWindow_NativeObjectUnreference</b>.\n
    This interface needs to be used in conjunction with <b>OH_NativeImage_ReleaseNativeWindowBuffer<\b>,
    otherwise memory leaks will occur.\n
    When the fenceFd is used up, you need to close it.\n

    @syscap SystemCapability.Graphic.Graphic2D.NativeImage
    @param image Indicates the pointer to a <b>OH_NativeImage</b> instance.
    @param nativeWindowBuffer Indicates the pointer to an <b>OHNativeWindowBuffer</b> point.
    @param fenceFd Indicates the pointer to a file descriptor handle.
    @return {@link NATIVE_ERROR_OK} 0 - Success.
        {@link NATIVE_ERROR_INVALID_ARGUMENTS} 40001000 - image, nativeWindowBuffer, fenceFd is NULL.
        {@link NATIVE_ERROR_NO_BUFFER} 40601000 - No buffer for consume.
    @since 12
    @version 1.0*/
    pub fn OH_NativeImage_AcquireNativeWindowBuffer(
        image: *mut OH_NativeImage,
        nativeWindowBuffer: *mut *mut OHNativeWindowBuffer,
        fenceFd: *mut ::core::ffi::c_int,
    ) -> i32;
    /** @brief Release the <b>OHNativeWindowBuffer</b> to the buffer queue through an
    <b>OH_NativeImage</b> instance for reuse.\n
    The fenceFd will be close by system.\n

    @syscap SystemCapability.Graphic.Graphic2D.NativeImage
    @param image Indicates the pointer to a <b>OH_NativeImage</b> instance.
    @param nativeWindowBuffer Indicates the pointer to an <b>OHNativeWindowBuffer</b> instance.
    @param fenceFd Indicates a file descriptor handle, which is used for timing synchronization.
    @return {@link NATIVE_ERROR_OK} 0 - Success.
        {@link NATIVE_ERROR_INVALID_ARGUMENTS} 40001000 - image, nativeWindowBuffer is NULL.
        {@link NATIVE_ERROR_BUFFER_STATE_INVALID} 41207000 - nativeWindowBuffer state invalid.
        {@link NATIVE_ERROR_BUFFER_NOT_IN_CACHE} 41210000 - nativeWindowBuffer not in cache.
    @since 12
    @version 1.0*/
    pub fn OH_NativeImage_ReleaseNativeWindowBuffer(
        image: *mut OH_NativeImage,
        nativeWindowBuffer: *mut OHNativeWindowBuffer,
        fenceFd: ::core::ffi::c_int,
    ) -> i32;
    /** @brief Create a <b>OH_NativeImage</b> as a consumerSurface. \n
    This method can not be used at the same time with <b>OH_NativeImage_UpdateSurfaceImage</b>.\n
    This interface needs to be used in conjunction with <b>OH_NativeImage_Destroy<\b>,
    otherwise memory leaks will occur.\n
    @syscap SystemCapability.Graphic.Graphic2D.NativeImage
    @return Returns the pointer to the <b>OH_NativeImage</b> instance created if the operation is successful, \n
    returns <b>NULL</b> otherwise.
    @since 12
    @version 1.0*/
    pub fn OH_ConsumerSurface_Create() -> *mut OH_NativeImage;
}
