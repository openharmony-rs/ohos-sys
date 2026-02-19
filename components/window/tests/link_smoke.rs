use std::ptr;

use ohos_window_sys as window;

#[test]
fn link_smoke() {
    unsafe {
        let _ = window::native_buffer::native_buffer::OH_NativeBuffer_Alloc(ptr::null());
        let _ = window::native_image::OH_NativeImage_Create(0, 0);
        let _ = window::native_window::OH_NativeWindow_CreateNativeWindowBufferFromNativeBuffer(
            ptr::null_mut(),
        );
    }

    #[cfg(feature = "api-11")]
    unsafe {
        let _ = window::native_buffer::native_buffer::OH_NativeBuffer_SetColorSpace(
            ptr::null_mut(),
            std::mem::zeroed(),
        );
        let _ = window::native_image::OH_NativeImage_GetSurfaceId(ptr::null_mut(), ptr::null_mut());
    }

    #[cfg(feature = "api-12")]
    unsafe {
        let _ = window::native_buffer::native_buffer::OH_NativeBuffer_MapPlanes(
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut(),
        );
        let _ = window::native_image::OH_NativeImage_GetTransformMatrixV2(
            ptr::null_mut(),
            ptr::null_mut(),
        );
        let _ = window::native_window::OH_NativeWindow_NativeWindowAttachBuffer(
            ptr::null_mut(),
            ptr::null_mut(),
        );
    }

    #[cfg(feature = "api-13")]
    unsafe {
        let _ = window::native_image::OH_ConsumerSurface_SetDefaultUsage(ptr::null_mut(), 0);
    }

    #[cfg(feature = "api-15")]
    unsafe {
        let _ =
            window::native_image::OH_NativeImage_GetBufferMatrix(ptr::null_mut(), ptr::null_mut());
    }

    #[cfg(feature = "api-17")]
    unsafe {
        let _ = window::native_image::OH_NativeImage_SetDropBufferMode(ptr::null_mut(), false);
    }

    #[cfg(feature = "api-19")]
    unsafe {
        let _ = window::native_window::OH_NativeWindow_CleanCache(ptr::null_mut());
    }

    #[cfg(feature = "api-20")]
    unsafe {
        let _ = window::native_fence::OH_NativeFence_IsValid(-1);
    }
}
