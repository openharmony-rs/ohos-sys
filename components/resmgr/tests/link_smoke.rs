#[cfg(feature = "api-12")]
#[test]
fn link_smoke_api_12() {
    use core::ptr;
    unsafe {
        let _ = ohos_resmgr_sys::ohresmgr::OH_ResourceManager_GetSymbol(
            ptr::null(),
            0,
            ptr::null_mut(),
        );
    }
}

#[cfg(feature = "api-18")]
#[test]
fn link_smoke_api_18() {
    use core::ptr;

    unsafe {
        let _ = ohos_resmgr_sys::ohresmgr::OH_ResourceManager_GetIntPluralString(
            ptr::null(),
            0,
            0,
            ptr::null_mut(),
        );
    }
}

#[cfg(feature = "api-20")]
#[test]
fn link_smoke_api_20() {
    use core::ptr;
    unsafe {
        let _ = ohos_resmgr_sys::ohresmgr::OH_ResourceManager_GetResourceConfiguration(
            ptr::null(),
            ptr::null_mut(),
        );
    }
}
