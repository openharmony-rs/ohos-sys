//! Data structures for the C APIs of the HTTP module.

#[cfg(feature = "api-20")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-20")))]
#[link(name = "net_http")]
unsafe extern "C" {}

mod net_http_type_ffi;
pub use net_http_type_ffi::*;
