#[link(name = "ace_ndk.z")]
#[link(name = "native_window")]
extern "C" {}

mod native_window_api10;
pub use native_window_api10::*;

mod native_window_api10_fixup;
pub use native_window_api10_fixup::NativeWindowOperation;
