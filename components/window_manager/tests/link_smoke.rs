#![cfg(feature = "api-12")]

use std::ptr;

use ohos_window_manager_sys as wm;

fn touch_type<T>() {
    let _ = std::mem::size_of::<T>();
}

#[test]
fn link_smoke() {
    unsafe {
        let _ = wm::display_manager::OH_NativeDisplayManager_GetDefaultDisplayId(ptr::null_mut());
        let _ = wm::window_event_filter::OH_NativeWindowManager_RegisterKeyEventFilter(0, None);
    }

    let _ = wm::display_info::OH_DISPLAY_NAME_LENGTH;
    touch_type::<wm::window_comm::WindowManagerErrorCode>();

    #[cfg(feature = "api-14")]
    unsafe {
        let _ =
            wm::display_capture::OH_NativeDisplayManager_CaptureScreenPixelmap(0, ptr::null_mut());
        let _ = wm::display_manager::OH_NativeDisplayManager_CreateAllDisplays(ptr::null_mut());
    }

    #[cfg(feature = "api-15")]
    unsafe {
        let _ = wm::window_event_filter::OH_NativeWindowManager_RegisterMouseEventFilter(0, None);
        let _ = wm::window::OH_WindowManager_ShowWindow(0);
    }

    #[cfg(feature = "api-17")]
    unsafe {
        let _ = wm::window::OH_WindowManager_GetAllWindowLayoutInfoList(
            0,
            ptr::null_mut(),
            ptr::null_mut(),
        );
    }

    #[cfg(feature = "api-20")]
    unsafe {
        let _ = wm::display_manager::OH_NativeDisplayManager_RegisterAvailableAreaChangeListener(
            None,
            ptr::null_mut(),
        );
        let _ = wm::window::OH_WindowManager_InjectTouchEvent(0, ptr::null_mut(), 0, 0);
        let _ = wm::window_pip::OH_PictureInPicture_CreatePipConfig(ptr::null_mut());
    }

    #[cfg(feature = "api-21")]
    unsafe {
        let _ = wm::window::OH_WindowManager_GetAllMainWindowInfo(ptr::null_mut(), ptr::null_mut());
    }
}
