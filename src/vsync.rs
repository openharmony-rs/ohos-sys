//! Bindings to the native vsync APIs

#[link(name = "native_vsync")]
extern "C" {}

mod vsync_api10;
pub use vsync_api10::*;

#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
mod api12_additions;
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
pub use api12_additions::*;
