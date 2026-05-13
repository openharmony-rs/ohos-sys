use std::ptr;

use ohos_huks_sys as huks;

#[test]
fn link_smoke() {
    unsafe {
        // API-10 baseline (HUKS itself first available at API-9).
        let _ = huks::native_huks_api::OH_Huks_GetSdkVersion(ptr::null_mut());
        let _ = huks::native_huks_api::OH_Huks_DeleteKeyItem(ptr::null(), ptr::null());

        // Parameter-set lifecycle, all available at API-10.
        let _ = huks::native_huks_param::OH_Huks_InitParamSet(ptr::null_mut());
        let _ = huks::native_huks_param::OH_Huks_BuildParamSet(ptr::null_mut());
        huks::native_huks_param::OH_Huks_FreeParamSet(ptr::null_mut());
    }

    #[cfg(feature = "api-20")]
    unsafe {
        let _ = huks::native_huks_api::OH_Huks_WrapKey(ptr::null(), ptr::null(), ptr::null_mut());
        let _ = huks::native_huks_api::OH_Huks_UnwrapKey(ptr::null(), ptr::null(), ptr::null_mut());
    }
}
