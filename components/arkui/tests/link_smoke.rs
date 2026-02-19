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
        let _ = arkui::styled_string::OH_ArkUI_StyledString_Destroy(ptr::null_mut());
        let _ = arkui::ui_input_event::OH_ArkUI_UIInputEvent_GetType(ptr::null());
    }

    #[cfg(feature = "api-13")]
    unsafe {
        let _ =
            arkui::native_interface_accessibility::OH_ArkUI_AccessibilityProviderRegisterCallback(
                ptr::null_mut(),
                ptr::null_mut(),
            );
        let _ = arkui::native_node::OH_ArkUI_NodeUtils_AddCustomProperty(
            ptr::null_mut(),
            ptr::null(),
            ptr::null(),
        );
    }

    #[cfg(feature = "api-14")]
    unsafe {
        let _ = arkui::native_key_event::OH_ArkUI_KeyEvent_GetType(ptr::null());
        let _ = arkui::native_node::OH_ArkUI_NodeUtils_GetCustomProperty(
            ptr::null_mut(),
            ptr::null(),
            ptr::null_mut(),
        );
        let _ = arkui::native_type::OH_ArkUI_CustomProperty_Destroy(ptr::null_mut());
        let _ = arkui::styled_string::OH_ArkUI_StyledString_Descriptor_Create();
        let _ = arkui::ui_input_event::OH_ArkUI_UIInputEvent_GetDeviceId(ptr::null());
    }

    #[cfg(feature = "api-15")]
    unsafe {
        let _ = arkui::drag_and_drop::OH_ArkUI_DragEvent_StartDataLoading(
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut(),
            0,
        );
        let _ = arkui::native_gesture::OH_ArkUI_GestureInterruptInfo_GetTouchRecognizers(
            ptr::null(),
            ptr::null_mut(),
            ptr::null_mut(),
        );
        let _ = arkui::native_interface_accessibility::
            OH_ArkUI_AccessibilityProviderRegisterCallbackWithInstance(
                ptr::null(),
                ptr::null_mut(),
                ptr::null_mut(),
            );
        let _ = arkui::native_interface_focus::OH_ArkUI_FocusRequest(ptr::null_mut());
        let _ = arkui::native_key_event::OH_ArkUI_KeyEvent_Dispatch(ptr::null_mut(), ptr::null());
        let _ = arkui::native_node::OH_ArkUI_NodeEvent_GetTextChangeEvent(ptr::null_mut());
        let _ = arkui::native_type::OH_ArkUI_HostWindowInfo_GetName(ptr::null_mut());
        let _ = arkui::ui_input_event::OH_ArkUI_PointerEvent_GetChangedPointerId(
            ptr::null(),
            ptr::null_mut(),
        );
    }

    #[cfg(feature = "api-17")]
    unsafe {
        let _ = arkui::native_type::OH_ArkUI_VisibleAreaEventOptions_Create();
        let _ =
            arkui::ui_input_event::OH_ArkUI_PointerEvent_GetRollAngle(ptr::null(), ptr::null_mut());
    }

    #[cfg(feature = "api-18")]
    unsafe {
        let _ = arkui::native_gesture::OH_ArkUI_GetGestureParam_DirectMask(
            ptr::null_mut(),
            ptr::null_mut(),
        );
        let _ = arkui::native_node::OH_ArkUI_NodeUtils_MoveTo(ptr::null_mut(), ptr::null_mut(), 0);
        let _ = arkui::native_node_napi::OH_ArkUI_PostFrameCallback(
            ptr::null_mut(),
            ptr::null_mut(),
            None,
        );
        let _ = arkui::native_type::OH_ArkUI_AccessibilityValue_SetRangeMin(ptr::null_mut(), 0);
    }

    #[cfg(feature = "api-19")]
    unsafe {
        let _ = arkui::drag_and_drop::OH_ArkUI_DragEvent_RequestDragEndPending(
            ptr::null_mut(),
            ptr::null_mut(),
        );
        let _ = arkui::native_animate::OH_ArkUI_KeyframeAnimateOption_SetExpectedFrameRate(
            ptr::null_mut(),
            ptr::null_mut(),
        );
        let _ = arkui::native_dialog::OH_ArkUI_CustomDialog_SetLevelMode(
            ptr::null_mut(),
            std::mem::zeroed(),
        );
        let _ = arkui::native_dialog::OH_ArkUI_CustomDialog_OpenDialog(ptr::null_mut(), None);
        let _ = arkui::native_gesture::OH_ArkUI_PanGesture_SetDistanceMap(
            ptr::null_mut(),
            0,
            ptr::null_mut(),
            ptr::null_mut(),
        );
        let _ =
            arkui::native_key_event::OH_ArkUI_KeyEvent_IsNumLockOn(ptr::null(), ptr::null_mut());
        let _ =
            arkui::native_type::OH_ArkUI_SwiperIndicator_SetIgnoreSizeOfBottom(ptr::null_mut(), 0);
    }

    #[cfg(feature = "api-20")]
    unsafe {
        let _ = arkui::drag_and_drop::OH_ArkUI_DragEvent_SetDataLoadParams(
            ptr::null_mut(),
            ptr::null_mut(),
        );
        let _ =
            arkui::native_dialog::OH_ArkUI_CustomDialog_GetState(ptr::null_mut(), ptr::null_mut());
        let _ = arkui::native_gesture::OH_ArkUI_SetTouchTestDoneCallback(
            ptr::null_mut(),
            ptr::null_mut(),
            None,
        );
        let _ = arkui::native_node::OH_ArkUI_NodeUtils_GetLayoutPositionInGlobalDisplay(
            ptr::null_mut(),
            ptr::null_mut(),
        );
        let _ = arkui::native_node_napi::OH_ArkUI_InitModuleForArkTSEnv(ptr::null_mut());
        let _ = arkui::native_render::OH_ArkUI_RenderNodeUtils_CreateNode();
        let _ = arkui::native_type::OH_ArkUI_EmbeddedComponentOption_Create();
        let _ = arkui::ui_input_event::OH_ArkUI_PointerEvent_GetGlobalDisplayX(ptr::null());
    }

    #[cfg(feature = "api-21")]
    unsafe {
        let _ = arkui::native_node::OH_ArkUI_NativeModule_InvalidateAttributes(ptr::null_mut());
        let _ = arkui::native_type::OH_ArkUI_ContentTransitionEffect_Create(0);
    }
}
