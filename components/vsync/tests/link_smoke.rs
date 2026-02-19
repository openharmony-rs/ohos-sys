use std::ptr;

use ohos_vsync_sys as vsync;

#[test]
fn link_smoke() {
    unsafe {
        let _ = vsync::OH_NativeVSync_Create(ptr::null(), 0);
    }
}
