//! Native Buffer bindings
//!
//! The native buffer module provides APIs that you can use to apply for, use, and release the
//! shared memory, and query memory properties.
//! The following scenario is common for native buffer development:
//! Use the native buffer APIs to create an OH_NativeBuffer instance, obtain memory properties,
//! and map the corresponding ION memory to the process address space.
//!
//! Source:
//!
//! [English Documentation](https://docs.openharmony.cn/pages/v5.0/en/application-dev/graphics/native-buffer-guidelines.md)
//! [Chinese Documentation](https://docs.openharmony.cn/pages/v5.0/en/application-dev/graphics/native-buffer-guidelines.md)

#[link(name = "native_buffer")]
extern "C" {}

mod native_buffer_ffi;
pub use native_buffer_ffi::*;

#[cfg(feature = "native_window")]
#[cfg_attr(docsrs, doc(cfg(feature = "native_window")))]
mod native_buffer_window_ffi;
#[cfg(feature = "native_window")]
#[cfg_attr(docsrs, doc(cfg(feature = "native_window")))]
pub use native_buffer_window_ffi::*;
