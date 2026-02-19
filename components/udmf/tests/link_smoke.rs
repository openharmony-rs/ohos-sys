#![cfg(feature = "api-12")]

use std::ptr;

use udmf_sys as udmf;

fn touch_type<T>() {
    let _ = std::mem::size_of::<T>();
}

#[test]
fn link_smoke() {
    unsafe {
        let _ = udmf::data_management_framework::OH_UdmfData_Create();
        let _ = udmf::data_struct::OH_UdsPlainText_Create();
        let _ = udmf::type_descriptor::OH_Utd_Create(ptr::null());
    }

    let _ = udmf::meta::UDMF_META_ENTITY.as_ptr();
    touch_type::<udmf::Udmf_ErrCode>();

    #[cfg(feature = "api-13")]
    unsafe {
        let _ = udmf::data_management_framework::OH_UdmfRecordProvider_Create();
        let _ = udmf::data_struct::OH_UdsFileUri_Create();
    }

    #[cfg(feature = "api-14")]
    unsafe {
        let _ = udmf::data_management_framework::OH_UdmfRecord_AddContentForm(
            ptr::null_mut(),
            ptr::null_mut(),
        );
        let _ = udmf::data_struct::OH_UdsContentForm_Create();
    }

    #[cfg(feature = "api-15")]
    unsafe {
        let _ = udmf::data_management_framework::OH_UdmfProgressInfo_GetProgress(ptr::null_mut());
    }

    #[cfg(feature = "api-20")]
    unsafe {
        let _ = udmf::data_management_framework::OH_UdmfOptions_Create();
    }
}
