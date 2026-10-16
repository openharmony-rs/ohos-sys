#[cfg(feature = "api-12")]
#[test]
fn link_smoke() {
    unsafe {
        let _ = ohos_native_display_soloist_sys::OH_DisplaySoloist_Create(false);
    }
}
