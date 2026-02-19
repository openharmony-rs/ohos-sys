#![cfg(feature = "api-12")]

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
}
