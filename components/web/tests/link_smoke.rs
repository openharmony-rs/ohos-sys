#![cfg(feature = "api-11")]

use std::ptr;

use arkweb_sys as web;

#[test]
fn link_smoke() {
    unsafe {
        let _ = web::native_interface_arkweb::OH_NativeArkWeb_RunJavaScript(
            ptr::null(),
            ptr::null(),
            None,
        );
    }

    #[cfg(feature = "api-12")]
    unsafe {
        let _ = web::arkweb_interface::OH_ArkWeb_GetNativeAPI(
            web::arkweb_interface::ArkWeb_NativeAPIVariantKind::ARKWEB_NATIVE_COMPONENT,
        );
        let _ = web::arkweb_scheme_handler::OH_ArkWeb_ReleaseString(ptr::null_mut());
    }

    #[cfg(feature = "api-15")]
    unsafe {
        let _ = web::native_interface_arkweb::OH_NativeArkWeb_LoadData(
            ptr::null(),
            ptr::null(),
            ptr::null(),
            ptr::null(),
            ptr::null(),
            ptr::null(),
        );
    }

    #[cfg(feature = "api-18")]
    unsafe {
        let _ = web::arkweb_interface::OH_ArkWeb_RegisterScrollCallback(
            ptr::null(),
            None,
            ptr::null_mut(),
        );
    }

    #[cfg(feature = "api-20")]
    unsafe {
        let _ = web::native_interface_arkweb::OH_ArkWebCookieManager_SaveCookieSync();
        let _ = web::arkweb_scheme_handler::OH_ArkWebHttpBodyStream_SetAsyncReadCallback(
            ptr::null_mut(),
            None,
        );
    }

    // type-only modules: ensure they are referenced
    #[cfg(feature = "api-12")]
    {
        let _ = std::mem::size_of::<web::arkweb_net_error_list::ArkWeb_NetError>();
        let _ = std::mem::size_of::<web::arkweb_type::ArkWeb_JavaScriptBridgeData>();
    }

    #[cfg(feature = "api-20")]
    {
        let _ = std::mem::size_of::<web::arkweb_error_code::ArkWeb_BlanklessErrorCode>();
    }
}
