//! Image Kit
//!
//! Image development is the process of parsing, processing, and constructing image pixel data to
//! achieve the required image effect. Image development mainly involves image decoding, processing,
//! and encoding.
//!
//! See the official upstream documentation for more information:
//!
//! - [Introduction to Image Kit]
//! - [Image NativeModule]
//!
//! [Introduction to Image Kit]: https://docs.openharmony.cn/pages/v5.0/en/application-dev/media/image/image-overview.md
//! [Image NativeModule]: https://docs.openharmony.cn/pages/v5.0/en/application-dev/media/image/image-structure-c.md

#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
pub mod native_image;
