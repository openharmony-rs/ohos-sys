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
}
