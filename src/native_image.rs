//! Bindings to `native_image`
//!
//! The native image module is used for associating a surface with an OpenGL external texture.
//! It functions as the consumer of a graphics queue.
//! You can use the APIs of this module to obtain and use a buffer, and output the buffer content to an OpenGL external texture.
//!
//! The following scenario is common for native image development:
//!
//! Use the native image APIs to create an OH_NativeImage instance as the consumer and obtain the corresponding OHNativeWindow instance (functioning as the producer).
//! Use the native window APIs to fill in and flush the buffer, and then use the native image APIs to update the buffer content to an OpenGL ES texture.
//!
//! Source: [Official Native Image documentation](https://docs.openharmony.cn/pages/v5.0/en/application-dev/graphics/native-image-guidelines.md)
//!

#[link(name = "EGL")]
#[link(name = "GLESv3")]
#[link(name = "native_image")]
extern "C" {}

mod native_image_api10;
pub use native_image_api10::*;

#[cfg(feature = "api-11")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-11")))]
mod api11_additions;
#[cfg(feature = "api-11")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-11")))]
pub use api11_additions::*;

#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
mod api12_additions;

#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
pub use api12_additions::*;
