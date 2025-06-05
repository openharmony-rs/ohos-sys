#[link(name = "native_window_manager")]
extern "C" {}
mod window_comm_ffi;
pub use window_comm_ffi::*;
