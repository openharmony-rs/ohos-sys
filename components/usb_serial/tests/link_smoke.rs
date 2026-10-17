#[cfg(feature = "api-18")]
#[test]
fn link_smoke() {
    std::hint::black_box(ohos_usb_serial_sys::OH_UsbSerial_Init as *const ());
}
