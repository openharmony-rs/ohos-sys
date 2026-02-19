#![cfg(feature = "api-12")]

use std::ptr;

use ohos_ime_sys as ime;

#[test]
fn link_smoke() {
    unsafe {
        let _ = ime::attach_options::OH_AttachOptions_Create(true);
        let _ = ime::controller::OH_InputMethodController_Attach(
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut(),
        );
        let _ = ime::cursor_info::OH_CursorInfo_Create(0.0, 0.0, 0.0, 0.0);
        let _ = ime::inputmethod_proxy::OH_InputMethodProxy_ShowKeyboard(ptr::null_mut());
        let _ = ime::private_command::OH_PrivateCommand_Create(ptr::null_mut(), 0);
        let _ = ime::text_avoid_info::OH_TextAvoidInfo_Create(0.0, 0.0);
        let _ = ime::text_config::OH_TextConfig_Create();
        let _ = ime::text_editor_proxy::OH_TextEditorProxy_Create();
    }

    let _ = std::mem::size_of::<ime::types::InputMethod_KeyboardStatus>();

    #[cfg(feature = "api-15")]
    unsafe {
        let _ = ime::attach_options::OH_AttachOptions_CreateWithRequestKeyboardReason(
            false,
            std::mem::zeroed(),
        );
        let _ = ime::inputmethod_proxy::OH_InputMethodProxy_ShowTextInput(
            ptr::null_mut(),
            ptr::null_mut(),
        );
    }

    #[cfg(feature = "api-20")]
    unsafe {
        let _ = ime::text_config::OH_TextConfig_SetPlaceholder(ptr::null_mut(), ptr::null(), 0);
    }
}
