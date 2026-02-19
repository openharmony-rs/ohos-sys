#![cfg(feature = "api-12")]

use ohos_pasteboard_sys as pasteboard;

fn touch_type<T>() {
    let _ = std::mem::size_of::<T>();
}

#[test]
fn link_smoke() {
    unsafe {
        let _ = pasteboard::OH_Pasteboard_Create();
    }

    #[cfg(feature = "api-13")]
    touch_type::<pasteboard::PASTEBOARD_ErrCode>();
}
