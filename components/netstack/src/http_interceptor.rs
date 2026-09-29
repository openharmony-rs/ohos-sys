//! C interface for the HTTP interceptor module of NetworkKit netstack.

#[link(name = "http_interceptor")]
unsafe extern "C" {}

mod http_interceptor_ffi;
pub use http_interceptor_ffi::*;
