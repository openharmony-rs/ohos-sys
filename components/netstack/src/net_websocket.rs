//! C interface for the WebSocket client module of NetworkKit netstack.

#[cfg(feature = "api-11")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-11")))]
#[link(name = "net_websocket")]
unsafe extern "C" {}

mod net_websocket_ffi;
pub use net_websocket_ffi::*;
