#![allow(non_camel_case_types)]

#[repr(C)]
pub struct OH_Drawing_Point {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct OH_Drawing_Rect {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct OH_Drawing_RoundRect {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct OH_Drawing_Matrix {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct OH_Drawing_ShaderEffect {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct OH_Drawing_Filter {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct OH_Drawing_MaskFilter {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct OH_Drawing_ColorFilter {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct OH_Drawing_Font {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct OH_Drawing_Typeface {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct OH_Drawing_TextBlob {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct OH_Drawing_TextBlobBuilder {
    _unused: [u8; 0],
}

impl OH_Drawing_BlendMode {
    /// r = 0.
    pub const BLEND_MODE_CLEAR: OH_Drawing_BlendMode = OH_Drawing_BlendMode(0);
}
impl OH_Drawing_BlendMode {
    /// r = s.
    pub const BLEND_MODE_SRC: OH_Drawing_BlendMode = OH_Drawing_BlendMode(1);
}
impl OH_Drawing_BlendMode {
    /// r = d.
    pub const BLEND_MODE_DST: OH_Drawing_BlendMode = OH_Drawing_BlendMode(2);
}
impl OH_Drawing_BlendMode {
    /// r = s + (1-sa)*d.
    pub const BLEND_MODE_SRC_OVER: OH_Drawing_BlendMode = OH_Drawing_BlendMode(3);
}
impl OH_Drawing_BlendMode {
    /// r = d + (1-da)*s.
    pub const BLEND_MODE_DST_OVER: OH_Drawing_BlendMode = OH_Drawing_BlendMode(4);
}
impl OH_Drawing_BlendMode {
    /// r = s * da.
    pub const BLEND_MODE_SRC_IN: OH_Drawing_BlendMode = OH_Drawing_BlendMode(5);
}
impl OH_Drawing_BlendMode {
    /// r = d * sa.
    pub const BLEND_MODE_DST_IN: OH_Drawing_BlendMode = OH_Drawing_BlendMode(6);
}
impl OH_Drawing_BlendMode {
    /// r = s * (1-da).
    pub const BLEND_MODE_SRC_OUT: OH_Drawing_BlendMode = OH_Drawing_BlendMode(7);
}
impl OH_Drawing_BlendMode {
    /// r = d * (1-sa).
    pub const BLEND_MODE_DST_OUT: OH_Drawing_BlendMode = OH_Drawing_BlendMode(8);
}
impl OH_Drawing_BlendMode {
    /// r = s*da + d*(1-sa).
    pub const BLEND_MODE_SRC_ATOP: OH_Drawing_BlendMode = OH_Drawing_BlendMode(9);
}
impl OH_Drawing_BlendMode {
    /// r = d*sa + s*(1-da).
    pub const BLEND_MODE_DST_ATOP: OH_Drawing_BlendMode = OH_Drawing_BlendMode(10);
}
impl OH_Drawing_BlendMode {
    /// r = s*(1-da) + d*(1-sa).
    pub const BLEND_MODE_XOR: OH_Drawing_BlendMode = OH_Drawing_BlendMode(11);
}
impl OH_Drawing_BlendMode {
    /// r = min(s + d, 1).
    pub const BLEND_MODE_PLUS: OH_Drawing_BlendMode = OH_Drawing_BlendMode(12);
}
impl OH_Drawing_BlendMode {
    /// r = s*d.
    pub const BLEND_MODE_MODULATE: OH_Drawing_BlendMode = OH_Drawing_BlendMode(13);
}
impl OH_Drawing_BlendMode {
    /// r = s + d - s*d.
    pub const BLEND_MODE_SCREEN: OH_Drawing_BlendMode = OH_Drawing_BlendMode(14);
}
impl OH_Drawing_BlendMode {
    /// multiply or screen, depending on destination.
    pub const BLEND_MODE_OVERLAY: OH_Drawing_BlendMode = OH_Drawing_BlendMode(15);
}
impl OH_Drawing_BlendMode {
    /// rc = s + d - max(s*da, d*sa), ra = s + (1-sa)*d.
    pub const BLEND_MODE_DARKEN: OH_Drawing_BlendMode = OH_Drawing_BlendMode(16);
}
impl OH_Drawing_BlendMode {
    /// rc = s + d - min(s*da, d*sa), ra = s + (1-sa)*d.
    pub const BLEND_MODE_LIGHTEN: OH_Drawing_BlendMode = OH_Drawing_BlendMode(17);
}
impl OH_Drawing_BlendMode {
    /// brighten destination to reflect source.
    pub const BLEND_MODE_COLOR_DODGE: OH_Drawing_BlendMode = OH_Drawing_BlendMode(18);
}
impl OH_Drawing_BlendMode {
    /// darken destination to reflect source.
    pub const BLEND_MODE_COLOR_BURN: OH_Drawing_BlendMode = OH_Drawing_BlendMode(19);
}
impl OH_Drawing_BlendMode {
    /// multiply or screen, depending on source.
    pub const BLEND_MODE_HARD_LIGHT: OH_Drawing_BlendMode = OH_Drawing_BlendMode(20);
}
impl OH_Drawing_BlendMode {
    /// lighten or darken, depending on source.
    pub const BLEND_MODE_SOFT_LIGHT: OH_Drawing_BlendMode = OH_Drawing_BlendMode(21);
}
impl OH_Drawing_BlendMode {
    /// rc = s + d - 2*(min(s*da, d*sa)), ra = s + (1-sa)*d.
    pub const BLEND_MODE_DIFFERENCE: OH_Drawing_BlendMode = OH_Drawing_BlendMode(22);
}
impl OH_Drawing_BlendMode {
    /// rc = s + d - two(s*d), ra = s + (1-sa)*d.
    pub const BLEND_MODE_EXCLUSION: OH_Drawing_BlendMode = OH_Drawing_BlendMode(23);
}
impl OH_Drawing_BlendMode {
    /// r = s*(1-da) + d*(1-sa) + s*d.
    pub const BLEND_MODE_MULTIPLY: OH_Drawing_BlendMode = OH_Drawing_BlendMode(24);
}
impl OH_Drawing_BlendMode {
    /// hue of source with saturation and luminosity of destination.
    pub const BLEND_MODE_HUE: OH_Drawing_BlendMode = OH_Drawing_BlendMode(25);
}
impl OH_Drawing_BlendMode {
    /// saturation of source with hue and luminosity of destination.
    pub const BLEND_MODE_SATURATION: OH_Drawing_BlendMode = OH_Drawing_BlendMode(26);
}
impl OH_Drawing_BlendMode {
    /// hue and saturation of source with luminosity of destination.
    pub const BLEND_MODE_COLOR: OH_Drawing_BlendMode = OH_Drawing_BlendMode(27);
}
impl OH_Drawing_BlendMode {
    /// luminosity of source with hue and saturation of destination.
    pub const BLEND_MODE_LUMINOSITY: OH_Drawing_BlendMode = OH_Drawing_BlendMode(28);
}
#[repr(transparent)]
/** @brief The blending operation generates a new color for the two colors (source, target).
These operations are the same on the 4 color channels: red, green, blue, alpha.
For these, we use alpha channel as an example, rather than naming each channel individually.

For brevity, we use the following abbreviations.
s  : source
d  : destination
sa : source alpha
da : destination alpha

Results are abbreviated
r  : if all 4 channels are computed in the same manner
ra : result alpha channel
rc : result "color": red, green, blue channels

@since 11
@version 1.0*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct OH_Drawing_BlendMode(pub ::core::ffi::c_uint);
