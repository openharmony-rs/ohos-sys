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

    #[cfg(feature = "api-20")]
    unsafe {
        let _ = window::native_fence::OH_NativeFence_IsValid(-1);
    }
}
