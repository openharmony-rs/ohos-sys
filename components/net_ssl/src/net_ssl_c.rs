//! C interface for the SSL/TLS certificate chain verification module of NetworkKit net_ssl.

#[cfg(feature = "api-11")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-11")))]
#[link(name = "net_ssl")]
unsafe extern "C" {}

mod net_ssl_c_ffi;
pub use net_ssl_c_ffi::*;
