#[test]
fn link_smoke_createparamlist() {
    unsafe {
        let _ = ohos_hiappevent_sys::hiappevent::OH_HiAppEvent_CreateParamList();
    }
}

#[cfg(feature = "api-12")]
#[test]
fn link_smoke_api_12() {
    use core::ptr;
    unsafe {
        let _ = ohos_hiappevent_sys::hiappevent::OH_HiAppEvent_CreateWatcher(ptr::null());
    }
}

#[cfg(feature = "api-15")]
#[test]
fn link_smoke_api_15() {
    unsafe {
        let _ = ohos_hiappevent_sys::hiappevent::OH_HiAppEvent_CreateConfig();
    }
}

#[cfg(feature = "api-18")]
#[test]
fn link_smoke_api_18() {
    use core::ptr;
    unsafe {
        let _ = ohos_hiappevent_sys::hiappevent::OH_HiAppEvent_AddProcessor(ptr::null_mut());
    }
}

#[cfg(feature = "api-20")]
#[test]
fn link_smoke_api_20() {
    use core::ptr;
    unsafe {
        let _ = ohos_hiappevent_sys::hiappevent::OH_HiAppEvent_SetConfigName(
            ptr::null_mut(),
            ptr::null(),
        );
    }
}

#[cfg(feature = "api-26")]
#[test]
fn link_smoke_api_26() {
    use core::ptr;
    use ohos_hiappevent_sys::hiappevent::OH_HiAppEvent_FrameworkType;
    unsafe {
        let _ = ohos_hiappevent_sys::hiappevent::OH_HiAppEvent_ReportFrameworkMemAnomaly(
            OH_HiAppEvent_FrameworkType::OH_FLUTTER_DART,
            ptr::null(),
            ptr::null(),
        );
    }
}
