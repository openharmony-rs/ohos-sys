//! Data structures for the C APIs of the SSL/TLS certificate chain verification module.

#[cfg(feature = "api-11")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-11")))]
#[link(name = "net_ssl")]
unsafe extern "C" {}

mod net_ssl_c_type_ffi;
pub use net_ssl_c_type_ffi::*;
