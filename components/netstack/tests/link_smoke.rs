#![cfg(feature = "api-11")]

use ohos_netstack_sys as netstack;

#[test]
fn link_smoke() {
    unsafe {
        let websocket =
            netstack::net_websocket::OH_WebSocketClient_Constructor(None, None, None, None);
        if !websocket.is_null() {
            let _ = netstack::net_websocket::OH_WebSocketClient_Destroy(websocket);
        }

        #[cfg(feature = "api-20")]
        {
            let mut headers = netstack::net_http::OH_Http_CreateHeaders();
            if !headers.is_null() {
                netstack::net_http::OH_Http_DestroyHeaders(&mut headers);
            }
        }
    }
}
