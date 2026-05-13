//! C interface for the HTTP module of NetworkKit netstack.

#[cfg(feature = "api-20")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-20")))]
#[link(name = "net_http")]
unsafe extern "C" {}

mod net_http_ffi;
pub use net_http_ffi::*;
