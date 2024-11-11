//! FFI Bindings for `native_drawing` on OpenHarmony
//!
//! The native drawing module provides APIs for drawing 2D graphics and text.
//! The graphics and text drawn by using the APIs cannot be directly displayed on the screen.
//! To display the drawn graphics and text, you'll need the capabilities provided by the <XComponent> and native window module.
//!
//! Please consult the official documentation for more details: [English docs], [Chinese docs]
//!
//! [English docs]: https://docs.openharmony.cn/pages/v5.0/en/application-dev/graphics/drawing-guidelines.md
//! [Chinese docs]: https://docs.openharmony.cn/pages/v5.0/zh-cn/application-dev/graphics/drawing-guidelines.md
//!
//! ## Feature flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[link(name = "native_drawing")]
extern "C" {}

pub mod bitmap;
pub mod brush;
pub mod canvas;
pub mod color;
pub mod font_collection;
pub mod path;
pub mod pen;
pub mod text_declaration;
pub mod text_typography;
pub mod types;

#[cfg(feature = "api-11")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-11")))]
pub mod color_filter;
#[cfg(feature = "api-11")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-11")))]
pub mod filter;
#[cfg(feature = "api-11")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-11")))]
pub mod font;
#[cfg(feature = "api-11")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-11")))]
pub mod mask_filter;
#[cfg(feature = "api-11")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-11")))]
pub mod matrix;
#[cfg(feature = "api-11")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-11")))]
pub mod point;
#[cfg(feature = "api-11")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-11")))]
pub mod rect;
#[cfg(feature = "api-11")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-11")))]
pub mod register_font;
#[cfg(feature = "api-11")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-11")))]
pub mod round_rect;
#[cfg(feature = "api-11")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-11")))]
pub mod shader_effect;
#[cfg(feature = "api-11")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-11")))]
pub mod text_blob;
#[cfg(feature = "api-11")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-11")))]
pub mod typeface;

#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
pub mod color_space;
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
pub mod error_code;
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
pub mod font_mgr;
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
pub mod gpu_context;
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
pub mod image;
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
pub mod image_filter;
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
pub mod memory_stream;
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
pub mod path_effect;
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
pub mod pixel_map;
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
pub mod region;
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
pub mod sampling_options;
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
pub mod shadow_layer;
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
pub mod surface;
