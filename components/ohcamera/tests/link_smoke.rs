#[cfg(feature = "api-11")]
#[test]
fn link_smoke_api_11() {
    let mut manager = core::ptr::null_mut();
    unsafe {
        let _ = ohos_ohcamera_sys::camera::OH_Camera_GetCameraManager(&mut manager);
    }
}

#[cfg(feature = "api-26")]
#[test]
fn link_smoke_api_26() {
    let mut object_type = ohos_ohcamera_sys::camera::Camera_MetadataObjectType(0);
    unsafe {
        let _ = ohos_ohcamera_sys::metadata_object_ext::OH_MetadataObjectExt_GetMetadataObjectType(
            core::ptr::null(),
            &mut object_type,
        );
    }
}
