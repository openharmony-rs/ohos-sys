#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::vsync::{OH_NativeVSync, OH_NativeVSync_FrameCallback};

extern "C" {
    /** @brief Request next vsync with callback.
    If this function is called multiple times in one vsync period, all these callbacks and dataset will be called.

    @syscap SystemCapability.Graphic.Graphic2D.NativeVsync
    @param nativeVsync Indicates the pointer to a NativeVsync.
    @param callback Indicates the OH_NativeVSync_FrameCallback which will be called when next vsync coming.
    @param data Indicates data whick will be used in callback.
    @return {@link NATIVE_ERROR_OK} 0 - Success.
        {@link NATIVE_ERROR_INVALID_ARGUMENTS} 40001000 - the parameter nativeVsync is NULL or callback is NULL.
        {@link NATIVE_ERROR_BINDER_ERROR} 50401000 - ipc send failed.
    @since 12
    @version 1.0*/
    pub fn OH_NativeVSync_RequestFrameWithMultiCallback(
        nativeVsync: *mut OH_NativeVSync,
        callback: OH_NativeVSync_FrameCallback,
        data: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
}
