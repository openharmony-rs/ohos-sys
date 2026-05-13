//! Data structures for the C APIs of the WebSocket client module.

#[cfg(feature = "api-11")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-11")))]
#[link(name = "net_websocket")]
unsafe extern "C" {}

mod net_websocket_type_ffi;
pub use net_websocket_type_ffi::*;
