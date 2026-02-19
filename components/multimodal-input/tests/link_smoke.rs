#![cfg(feature = "api-12")]

use std::ptr;

use ohos_input_sys as input;

fn touch_type<T>() {
    let _ = std::mem::size_of::<T>();
}

#[test]
fn link_smoke() {
    unsafe {
        let _ = input::input_manager::OH_Input_CreateKeyState();
    }

    touch_type::<input::axis_type::InputEvent_AxisType>();
    touch_type::<input::key_code::Input_KeyCode>();

    #[cfg(feature = "api-13")]
    unsafe {
        let _ = input::input_manager::OH_Input_GetDeviceIds(ptr::null_mut(), 0, ptr::null_mut());
    }

    #[cfg(feature = "api-14")]
    unsafe {
        let _ = input::input_manager::OH_Input_GetIntervalSinceLastInput(ptr::null_mut());
    }

    #[cfg(feature = "api-15")]
    unsafe {
        let _ = input::input_manager::OH_Input_SetKeyEventWindowId(ptr::null_mut(), 0);
    }

    #[cfg(feature = "api-20")]
    unsafe {
        let _ = input::input_manager::OH_Input_InjectMouseEventGlobal(ptr::null());
    }

    #[cfg(feature = "api-21")]
    unsafe {
        let _ = input::input_manager::OH_Input_GetKeyEventId(ptr::null_mut(), ptr::null_mut());
    }
}
