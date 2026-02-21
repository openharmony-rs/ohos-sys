//! Common LocationKit enums and data structures.

#[link(name = "location_ndk")]
unsafe extern "C" {}

mod location_type_ffi;
pub use location_type_ffi::*;
