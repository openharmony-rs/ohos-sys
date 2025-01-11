mod raw_file_ffi;
pub use raw_file_ffi::*;

#[link(name = "rawfile.z")]
extern "C" {}
