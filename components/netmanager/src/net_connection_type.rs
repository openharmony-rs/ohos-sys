//! Data structures for the C APIs of the network connection module.

#[link(name = "net_connection")]
unsafe extern "C" {}

mod net_connection_type_ffi;
pub use net_connection_type_ffi::*;
