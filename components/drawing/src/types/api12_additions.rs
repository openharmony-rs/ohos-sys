#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use super::{OH_Drawing_AlphaFormat, OH_Drawing_ColorFormat};

#[repr(C)]
pub struct OH_Drawing_Region {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct OH_Drawing_PixelMap {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct OH_Drawing_ColorSpace {
    _unused: [u8; 0],
}
/** @brief Defines a point of 2d.

@since 12
@version 1.0*/
#[repr(C)]
pub struct OH_Drawing_Point2D {
    pub x: f32,
    pub y: f32,
}
/** @brief Defines a corner radii, which is on x-axis and y-axis.

@since 12
@version 1.0*/
pub type OH_Drawing_Corner_Radii = OH_Drawing_Point2D;
/** @brief Defines a point of 3d, which is used to describe the coordinate point.

@since 12
@version 1.0*/
#[repr(C)]
pub struct OH_Drawing_Point3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
#[repr(C)]
pub struct OH_Drawing_PathEffect {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct OH_Drawing_ShadowLayer {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct OH_Drawing_MemoryStream {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct OH_Drawing_Image {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct OH_Drawing_ImageFilter {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct OH_Drawing_SamplingOptions {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct OH_Drawing_GpuContext {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct OH_Drawing_Surface {
    _unused: [u8; 0],
}

/** @brief Defines image info struct.

@since 12
@version 1.0*/
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OH_Drawing_Image_Info {
    /// storage for width of image
    pub width: i32,
    /// storage for height of image
    pub height: i32,
    /// storage for color formats
    pub colorType: OH_Drawing_ColorFormat,
    /// storage for alpha formats
    pub alphaType: OH_Drawing_AlphaFormat,
}
/** @brief Defines rectstyle info struct.

@since 12
@version 1.0*/
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OH_Drawing_RectStyle_Info {
    /// color of rectstyle
    pub color: u32,
    /// radius in left top of rectstyle
    pub leftTopRadius: f64,
    /// radius in right top of rectstyle
    pub rightTopRadius: f64,
    /// radius in right bottom of rectstyle
    pub rightBottomRadius: f64,
    /// radius in left bottom of rectstyle
    pub leftBottomRadius: f64,
}
impl OH_Drawing_TextEncoding {
    /// uses bytes to represent UTF-8 or ASCII
    pub const TEXT_ENCODING_UTF8: OH_Drawing_TextEncoding = OH_Drawing_TextEncoding(0);
}
impl OH_Drawing_TextEncoding {
    /// uses two byte words to represent most of Unicode
    pub const TEXT_ENCODING_UTF16: OH_Drawing_TextEncoding = OH_Drawing_TextEncoding(1);
}
impl OH_Drawing_TextEncoding {
    /// uses four byte words to represent all of Unicode
    pub const TEXT_ENCODING_UTF32: OH_Drawing_TextEncoding = OH_Drawing_TextEncoding(2);
}
impl OH_Drawing_TextEncoding {
    /// uses two byte words to represent glyph indices
    pub const TEXT_ENCODING_GLYPH_ID: OH_Drawing_TextEncoding = OH_Drawing_TextEncoding(3);
}
#[repr(transparent)]
/** @brief Enumerates text encoding types.
@since 12
@version 1.0*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct OH_Drawing_TextEncoding(pub ::core::ffi::c_uint);
#[repr(C)]
pub struct OH_Drawing_FontMgr {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct OH_Drawing_FontStyleSet {
    _unused: [u8; 0],
}
