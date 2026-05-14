use ohos_accesstoken_sys::ability_access_control;

#[test]
fn link_smoke() {
    #[cfg(feature = "api-12")]
    unsafe {
        let permission = c"ohos.permission.LOCATION".as_ptr();
        let _ = ability_access_control::OH_AT_CheckSelfPermission(permission);
    }
}
