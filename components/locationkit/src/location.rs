//! APIs for querying and controlling location updates.

#[link(name = "location_ndk")]
unsafe extern "C" {}

mod location_ffi;
pub use location_ffi::*;
