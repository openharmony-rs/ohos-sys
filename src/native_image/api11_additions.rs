#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use super::OH_NativeImage;

/** @brief The callback function of frame available.

@syscap SystemCapability.Graphic.Graphic2D.NativeImage
@param context User defined context, returned to the user in the callback function
@since 11
@version 1.0*/
pub type OH_OnFrameAvailable =
    ::core::option::Option<unsafe extern "C" fn(context: *mut ::core::ffi::c_void)>;
/** @brief A listener for native image, use <b>OH_NativeImage_SetOnFrameAvailableListener</b> to register \n
the listener object to <b>OH_NativeImage</b>, the callback will be triggered when there is available frame

@since 11
@version 1.0*/
#[repr(C)]
#[derive(Debug)]
pub struct OH_OnFrameAvailableListener {
    /// User defined context, returned to the user in the callback function
    pub context: *mut ::core::ffi::c_void,
    /// The callback function of frame available.
    pub onFrameAvailable: OH_OnFrameAvailable,
}

extern "C" {
    /** @brief Return the native image's surface id.

    @syscap SystemCapability.Graphic.Graphic2D.NativeImage
    @param image Indicates the pointer to a <b>OH_NativeImage</b> instance.
    @param surfaceId Indicates the surface id.
    @return Returns an error code, 0 is success, otherwise, failed.
    @since 11
    @version 1.0*/
    pub fn OH_NativeImage_GetSurfaceId(image: *mut OH_NativeImage, surfaceId: *mut u64) -> i32;
    /** @brief Set the frame available callback.

    @syscap SystemCapability.Graphic.Graphic2D.NativeImage
    @param image Indicates the pointer to a <b>OH_NativeImage</b> instance.
    @param listener Indicates the callback function.
    @return Returns an error code, 0 is success, otherwise, failed.
    @since 11
    @version 1.0*/
    pub fn OH_NativeImage_SetOnFrameAvailableListener(
        image: *mut OH_NativeImage,
        listener: OH_OnFrameAvailableListener,
    ) -> i32;
    /** @brief Unset the frame available callback.

    @syscap SystemCapability.Graphic.Graphic2D.NativeImage
    @param image Indicates the pointer to a <b>OH_NativeImage</b> instance.
    @return Returns an error code, 0 is success, otherwise, failed.
    @since 11
    @version 1.0*/
    pub fn OH_NativeImage_UnsetOnFrameAvailableListener(image: *mut OH_NativeImage) -> i32;
}
