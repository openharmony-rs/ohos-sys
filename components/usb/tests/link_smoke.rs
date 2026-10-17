#[test]
fn link_api_10() {
    std::hint::black_box(ohos_usb_sys::OH_Usb_Init as *const ());
}

#[cfg(feature = "api-12")]
#[test]
fn link_api_12() {
    std::hint::black_box(ohos_usb_sys::OH_Usb_SendPipeRequestWithAshmem as *const ());
}

#[cfg(feature = "api-18")]
#[test]
fn link_api_18() {
    std::hint::black_box(ohos_usb_sys::OH_Usb_GetDevices as *const ());
}

#[cfg(feature = "api-26")]
#[test]
fn link_api_26() {
    std::hint::black_box(ohos_usb_sys::OH_Usb_GetNonRootHubs as *const ());
}
