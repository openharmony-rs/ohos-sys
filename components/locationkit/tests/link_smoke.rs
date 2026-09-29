#![cfg(feature = "api-13")]

use ohos_locationkit_sys as locationkit;

#[test]
fn link_smoke() {
    let mut enabled = false;
    unsafe {
        let _ = locationkit::location::OH_Location_IsLocatingEnabled(&mut enabled);
    }

    #[cfg(feature = "api-26")]
    unsafe {
        let _ = locationkit::location_type::OH_LocationInfo_IsFromMock(std::ptr::null_mut());
    }
}
