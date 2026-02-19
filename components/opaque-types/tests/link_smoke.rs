use ohos_sys_opaque_types as opaque;

fn touch_type<T>() {
    let _ = std::mem::size_of::<T>();
}

#[test]
fn link_smoke() {
    touch_type::<opaque::OH_NativeBuffer>();
    touch_type::<opaque::OH_NativeImage>();
    touch_type::<opaque::OH_PixelmapNative>();
    touch_type::<opaque::Input_KeyEvent>();
    touch_type::<opaque::OH_UdmfData>();
    touch_type::<opaque::ArkUI_Node>();
}
