#![cfg(feature = "api-12")]

use std::ptr;

use ohos_ipckit_sys as ipckit;

#[test]
fn link_smoke() {
    unsafe {
        let _ = ipckit::cparcel::OH_IPCParcel_Create();
        let _ = ipckit::cremote_object::OH_IPCRemoteProxy_IsRemoteDead(ptr::null());
        let _ = ipckit::cskeleton::OH_IPCSkeleton_IsHandlingTransaction();
    }
}
