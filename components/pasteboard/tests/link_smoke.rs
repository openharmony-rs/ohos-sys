#![cfg(feature = "api-12")]

use std::ptr;

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
    unsafe {
        let _ = pasteboard::OH_PasteboardObserver_Create();
    }

    #[cfg(feature = "api-14")]
    unsafe {
        let _ = pasteboard::OH_Pasteboard_GetMimeTypes(ptr::null_mut(), ptr::null_mut());
    }

    #[cfg(feature = "api-15")]
    unsafe {
        let _ = pasteboard::OH_Pasteboard_GetDataParams_Create();
    }

    #[cfg(feature = "api-18")]
    unsafe {
        let _ = pasteboard::OH_Pasteboard_GetChangeCount(ptr::null_mut());
    }

    #[cfg(feature = "api-21")]
    unsafe {
        let _ = pasteboard::OH_Pasteboard_SyncDelayedDataAsync(ptr::null_mut(), None);
    }

    #[cfg(feature = "api-13")]
    touch_type::<pasteboard::PASTEBOARD_ErrCode>();
}
