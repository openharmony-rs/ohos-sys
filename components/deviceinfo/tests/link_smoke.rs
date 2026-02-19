use ohos_deviceinfo_sys as deviceinfo;

#[test]
fn link_smoke() {
    unsafe {
        let _ = deviceinfo::OH_GetSdkApiVersion();
    }
}
