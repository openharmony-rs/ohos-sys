#[cfg(feature = "api-24")]
#[test]
fn link_smoke() {
    let mut count = 0;
    unsafe {
        let _ = ohos_ohmidi_sys::OH_MIDIClient_GetDeviceCount(core::ptr::null(), &mut count);
    }
}
