#![cfg(feature = "api-11")]

use ohos_netmanager_sys as netmanager;

#[test]
fn link_smoke() {
    let mut has_default: i32 = 0;
    unsafe {
        let _ = netmanager::net_connection::OH_NetConn_HasDefaultNet(&mut has_default);
    }
}
