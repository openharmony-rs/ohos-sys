use std::ptr;

use arkui_sys as arkui;

#[test]
fn link_smoke() {
    #[cfg(feature = "api-12")]
    unsafe {
        let _ = arkui::drag_and_drop::OH_ArkUI_CreateDragPreviewOption();
        let _ = arkui::drawable_descriptor::OH_ArkUI_DrawableDescriptor_CreateFromPixelMap(
            ptr::null_mut(),
        );
        let _ = arkui::native_animate::OH_ArkUI_AnimateOption_Create();
        let _ = arkui::native_dialog::OH_ArkUI_DialogDismissEvent_GetDismissReason(ptr::null_mut());
        let _ = arkui::native_gesture::OH_ArkUI_GestureInterruptInfo_GetSystemFlag(ptr::null());
        let _ = arkui::native_interface::OH_ArkUI_QueryModuleInterfaceByName(
            arkui::native_interface::ArkUI_NativeAPIVariantKind::ARKUI_NATIVE_NODE,
            ptr::null(),
        );
        let _ = arkui::native_node::OH_ArkUI_NodeAdapter_Create();
        let _ = arkui::native_node_napi::OH_ArkUI_GetNodeHandleFromNapiValue(
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut(),
        );
        let _ = arkui::native_type::OH_ArkUI_LayoutConstraint_Create();
        let _ = arkui::styled_string::OH_ArkUI_StyledString_Descriptor_Create();
        let _ = arkui::ui_input_event::OH_ArkUI_UIInputEvent_GetType(ptr::null());
    }

    #[cfg(feature = "api-13")]
    unsafe {
        let _ = arkui::native_interface_accessibility::OH_ArkUI_CreateAccessibilityElementInfo();
    }

    #[cfg(feature = "api-14")]
    unsafe {
        let _ = arkui::native_key_event::OH_ArkUI_KeyEvent_GetType(ptr::null());
    }

    #[cfg(feature = "api-15")]
    unsafe {
        let _ = arkui::native_interface_focus::OH_ArkUI_FocusRequest(ptr::null_mut());
    }

    #[cfg(feature = "api-20")]
    unsafe {
        let _ = arkui::native_render::OH_ArkUI_RenderNodeUtils_CreateNode();
    }
}
