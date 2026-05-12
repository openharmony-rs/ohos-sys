//! C interface for the data network connection module of network management.

#[link(name = "net_connection")]
unsafe extern "C" {}

mod net_connection_ffi;
pub use net_connection_ffi::*;
