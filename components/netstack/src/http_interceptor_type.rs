//! Data structures for the C APIs of the HTTP interceptor module.

#[link(name = "http_interceptor")]
unsafe extern "C" {}

mod http_interceptor_type_ffi;
pub use http_interceptor_type_ffi::*;
