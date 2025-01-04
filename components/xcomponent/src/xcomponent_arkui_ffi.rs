// automatically generated by rust-bindgen 0.71.1

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use super::xcomponent_ffi::*;
use arkui_sys::native_type::ArkUI_NodeHandle;
use arkui_sys::ui_input_event::{ArkUI_UIInputEvent, ArkUI_UIInputEvent_Type, HitTestMode};

extern "C" {
    /// Attaches the UI component created through the native API of ArkUI to this <b>OH_NativeXComponent</b> instance.
    ///
    /// # Arguments
    ///
    /// `component` - Indicates the pointer to the <b>OH_NativeXComponent</b> instance.
    ///
    /// `root` - Indicates the pointer to the component instance created by the native API.
    ///
    /// # Returns
    ///
    /// Returns the error code.
    /// Returns [`ARKUI_ERROR_CODE_NO_ERROR`] if the operation is successful.
    /// Returns [`ARKUI_ERROR_CODE_PARAM_INVALID`] if a parameter error occurs.
    ///
    ///
    /// Available since API-level: 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_NativeXComponent_AttachNativeRootNode(
        component: *mut OH_NativeXComponent,
        root: ArkUI_NodeHandle,
    ) -> i32;
    /// Detaches the native component of ArkUI from this <b>OH_NativeXComponent</b> instance.
    ///
    /// # Arguments
    ///
    /// `component` - Indicates the pointer to the <b>OH_NativeXComponent</b> instance.
    ///
    /// `root` - Indicates the pointer to the component instance created by the native API.
    ///
    /// # Returns
    ///
    /// Returns the error code.
    /// Returns [`ARKUI_ERROR_CODE_NO_ERROR`] if the operation is successful.
    /// Returns [`ARKUI_ERROR_CODE_PARAM_INVALID`] if a parameter error occurs.
    ///
    ///
    /// Available since API-level: 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_NativeXComponent_DetachNativeRootNode(
        component: *mut OH_NativeXComponent,
        root: ArkUI_NodeHandle,
    ) -> i32;
    /// Registers a UI input event callback for this <b>OH_NativeXComponent</b> instance and enables the callback to
    /// be invoked when a UI input event is received.
    ///
    /// # Arguments
    ///
    /// `component` - Indicates the pointer to the <b>OH_NativeXComponent</b> instance.
    ///
    /// `callback` - Indicates the pointer to the UI input event callback.
    ///
    /// `type` - Indicates the type of the current UI input event.
    ///
    /// # Returns
    ///
    /// Returns the error code.
    /// Returns [`ARKUI_ERROR_CODE_NO_ERROR`] if the operation is successful.
    /// Returns [`ARKUI_ERROR_CODE_PARAM_INVALID`] if a parameter error occurs.
    ///
    /// Available since API-level: 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_NativeXComponent_RegisterUIInputEventCallback(
        component: *mut OH_NativeXComponent,
        callback: ::core::option::Option<
            unsafe extern "C" fn(
                component: *mut OH_NativeXComponent,
                event: *mut ArkUI_UIInputEvent,
                type_: ArkUI_UIInputEvent_Type,
            ),
        >,
        type_: ArkUI_UIInputEvent_Type,
    ) -> i32;
    /// Registers a custom event intercept callback for this <b>OH_NativeXComponent</b> and enables the callback
    /// during the hit test.
    ///
    /// # Arguments
    ///
    /// `component` - Indicates the pointer to the <b>OH_NativeXComponent</b> instance.
    ///
    /// `callback` - Indicates the pointer to the custom event intercept callback.
    ///
    /// # Returns
    ///
    /// Returns the error code.
    /// Returns [`ARKUI_ERROR_CODE_NO_ERROR`] if the operation is successful.
    /// Returns [`ARKUI_ERROR_CODE_PARAM_INVALID`] if a parameter error occurs.
    ///
    /// Available since API-level: 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_NativeXComponent_RegisterOnTouchInterceptCallback(
        component: *mut OH_NativeXComponent,
        callback: ::core::option::Option<
            unsafe extern "C" fn(
                component: *mut OH_NativeXComponent,
                event: *mut ArkUI_UIInputEvent,
            ) -> HitTestMode,
        >,
    ) -> i32;
}
