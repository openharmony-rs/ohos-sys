mod raw_dir_ffi;
pub use raw_dir_ffi::*;

#[link(name = "rawfile.z")]
extern "C" {}
