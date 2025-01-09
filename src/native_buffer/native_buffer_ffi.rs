// automatically generated by rust-bindgen 0.71.1

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use ohos_sys_opaque_types::{OHNativeWindowBuffer, OH_NativeBuffer};

impl OH_NativeBuffer_ColorSpace {
    pub const OH_COLORSPACE_NONE: OH_NativeBuffer_ColorSpace = OH_NativeBuffer_ColorSpace(0);
}
impl OH_NativeBuffer_ColorSpace {
    pub const OH_COLORSPACE_BT601_EBU_FULL: OH_NativeBuffer_ColorSpace =
        OH_NativeBuffer_ColorSpace(1);
}
impl OH_NativeBuffer_ColorSpace {
    pub const OH_COLORSPACE_BT601_SMPTE_C_FULL: OH_NativeBuffer_ColorSpace =
        OH_NativeBuffer_ColorSpace(2);
}
impl OH_NativeBuffer_ColorSpace {
    pub const OH_COLORSPACE_BT709_FULL: OH_NativeBuffer_ColorSpace = OH_NativeBuffer_ColorSpace(3);
}
impl OH_NativeBuffer_ColorSpace {
    pub const OH_COLORSPACE_BT2020_HLG_FULL: OH_NativeBuffer_ColorSpace =
        OH_NativeBuffer_ColorSpace(4);
}
impl OH_NativeBuffer_ColorSpace {
    pub const OH_COLORSPACE_BT2020_PQ_FULL: OH_NativeBuffer_ColorSpace =
        OH_NativeBuffer_ColorSpace(5);
}
impl OH_NativeBuffer_ColorSpace {
    pub const OH_COLORSPACE_BT601_EBU_LIMIT: OH_NativeBuffer_ColorSpace =
        OH_NativeBuffer_ColorSpace(6);
}
impl OH_NativeBuffer_ColorSpace {
    pub const OH_COLORSPACE_BT601_SMPTE_C_LIMIT: OH_NativeBuffer_ColorSpace =
        OH_NativeBuffer_ColorSpace(7);
}
impl OH_NativeBuffer_ColorSpace {
    pub const OH_COLORSPACE_BT709_LIMIT: OH_NativeBuffer_ColorSpace = OH_NativeBuffer_ColorSpace(8);
}
impl OH_NativeBuffer_ColorSpace {
    pub const OH_COLORSPACE_BT2020_HLG_LIMIT: OH_NativeBuffer_ColorSpace =
        OH_NativeBuffer_ColorSpace(9);
}
impl OH_NativeBuffer_ColorSpace {
    pub const OH_COLORSPACE_BT2020_PQ_LIMIT: OH_NativeBuffer_ColorSpace =
        OH_NativeBuffer_ColorSpace(10);
}
impl OH_NativeBuffer_ColorSpace {
    pub const OH_COLORSPACE_SRGB_FULL: OH_NativeBuffer_ColorSpace = OH_NativeBuffer_ColorSpace(11);
}
impl OH_NativeBuffer_ColorSpace {
    pub const OH_COLORSPACE_P3_FULL: OH_NativeBuffer_ColorSpace = OH_NativeBuffer_ColorSpace(12);
}
impl OH_NativeBuffer_ColorSpace {
    pub const OH_COLORSPACE_P3_HLG_FULL: OH_NativeBuffer_ColorSpace =
        OH_NativeBuffer_ColorSpace(13);
}
impl OH_NativeBuffer_ColorSpace {
    pub const OH_COLORSPACE_P3_PQ_FULL: OH_NativeBuffer_ColorSpace = OH_NativeBuffer_ColorSpace(14);
}
impl OH_NativeBuffer_ColorSpace {
    pub const OH_COLORSPACE_ADOBERGB_FULL: OH_NativeBuffer_ColorSpace =
        OH_NativeBuffer_ColorSpace(15);
}
impl OH_NativeBuffer_ColorSpace {
    pub const OH_COLORSPACE_SRGB_LIMIT: OH_NativeBuffer_ColorSpace = OH_NativeBuffer_ColorSpace(16);
}
impl OH_NativeBuffer_ColorSpace {
    pub const OH_COLORSPACE_P3_LIMIT: OH_NativeBuffer_ColorSpace = OH_NativeBuffer_ColorSpace(17);
}
impl OH_NativeBuffer_ColorSpace {
    pub const OH_COLORSPACE_P3_HLG_LIMIT: OH_NativeBuffer_ColorSpace =
        OH_NativeBuffer_ColorSpace(18);
}
impl OH_NativeBuffer_ColorSpace {
    pub const OH_COLORSPACE_P3_PQ_LIMIT: OH_NativeBuffer_ColorSpace =
        OH_NativeBuffer_ColorSpace(19);
}
impl OH_NativeBuffer_ColorSpace {
    pub const OH_COLORSPACE_ADOBERGB_LIMIT: OH_NativeBuffer_ColorSpace =
        OH_NativeBuffer_ColorSpace(20);
}
impl OH_NativeBuffer_ColorSpace {
    pub const OH_COLORSPACE_LINEAR_SRGB: OH_NativeBuffer_ColorSpace =
        OH_NativeBuffer_ColorSpace(21);
}
impl OH_NativeBuffer_ColorSpace {
    pub const OH_COLORSPACE_LINEAR_BT709: OH_NativeBuffer_ColorSpace =
        OH_NativeBuffer_ColorSpace(22);
}
impl OH_NativeBuffer_ColorSpace {
    pub const OH_COLORSPACE_LINEAR_P3: OH_NativeBuffer_ColorSpace = OH_NativeBuffer_ColorSpace(23);
}
impl OH_NativeBuffer_ColorSpace {
    pub const OH_COLORSPACE_LINEAR_BT2020: OH_NativeBuffer_ColorSpace =
        OH_NativeBuffer_ColorSpace(24);
}
impl OH_NativeBuffer_ColorSpace {
    pub const OH_COLORSPACE_DISPLAY_SRGB: OH_NativeBuffer_ColorSpace =
        OH_NativeBuffer_ColorSpace(25);
}
impl OH_NativeBuffer_ColorSpace {
    pub const OH_COLORSPACE_DISPLAY_P3_SRGB: OH_NativeBuffer_ColorSpace =
        OH_NativeBuffer_ColorSpace(26);
}
impl OH_NativeBuffer_ColorSpace {
    pub const OH_COLORSPACE_DISPLAY_P3_HLG: OH_NativeBuffer_ColorSpace =
        OH_NativeBuffer_ColorSpace(27);
}
impl OH_NativeBuffer_ColorSpace {
    pub const OH_COLORSPACE_DISPLAY_P3_PQ: OH_NativeBuffer_ColorSpace =
        OH_NativeBuffer_ColorSpace(28);
}
impl OH_NativeBuffer_ColorSpace {
    pub const OH_COLORSPACE_DISPLAY_BT2020_SRGB: OH_NativeBuffer_ColorSpace =
        OH_NativeBuffer_ColorSpace(29);
}
impl OH_NativeBuffer_ColorSpace {
    pub const OH_COLORSPACE_DISPLAY_BT2020_HLG: OH_NativeBuffer_ColorSpace =
        OH_NativeBuffer_ColorSpace(30);
}
impl OH_NativeBuffer_ColorSpace {
    pub const OH_COLORSPACE_DISPLAY_BT2020_PQ: OH_NativeBuffer_ColorSpace =
        OH_NativeBuffer_ColorSpace(31);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct OH_NativeBuffer_ColorSpace(pub ::core::ffi::c_uint);
impl OH_NativeBuffer_MetadataType {
    pub const OH_VIDEO_HDR_HLG: OH_NativeBuffer_MetadataType = OH_NativeBuffer_MetadataType(0);
}
impl OH_NativeBuffer_MetadataType {
    pub const OH_VIDEO_HDR_HDR10: OH_NativeBuffer_MetadataType = OH_NativeBuffer_MetadataType(1);
}
impl OH_NativeBuffer_MetadataType {
    pub const OH_VIDEO_HDR_VIVID: OH_NativeBuffer_MetadataType = OH_NativeBuffer_MetadataType(2);
}
impl OH_NativeBuffer_MetadataType {
    pub const OH_VIDEO_NONE: OH_NativeBuffer_MetadataType = OH_NativeBuffer_MetadataType(-1);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct OH_NativeBuffer_MetadataType(pub ::core::ffi::c_int);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OH_NativeBuffer_ColorXY {
    pub x: f32,
    pub y: f32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OH_NativeBuffer_Smpte2086 {
    pub displayPrimaryRed: OH_NativeBuffer_ColorXY,
    pub displayPrimaryGreen: OH_NativeBuffer_ColorXY,
    pub displayPrimaryBlue: OH_NativeBuffer_ColorXY,
    pub whitePoint: OH_NativeBuffer_ColorXY,
    pub maxLuminance: f32,
    pub minLuminance: f32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OH_NativeBuffer_Cta861 {
    pub maxContentLightLevel: f32,
    pub maxFrameAverageLightLevel: f32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OH_NativeBuffer_StaticMetadata {
    pub smpte2086: OH_NativeBuffer_Smpte2086,
    pub cta861: OH_NativeBuffer_Cta861,
}
impl OH_NativeBuffer_MetadataKey {
    pub const OH_HDR_METADATA_TYPE: OH_NativeBuffer_MetadataKey = OH_NativeBuffer_MetadataKey(0);
}
impl OH_NativeBuffer_MetadataKey {
    pub const OH_HDR_STATIC_METADATA: OH_NativeBuffer_MetadataKey = OH_NativeBuffer_MetadataKey(1);
}
impl OH_NativeBuffer_MetadataKey {
    pub const OH_HDR_DYNAMIC_METADATA: OH_NativeBuffer_MetadataKey = OH_NativeBuffer_MetadataKey(2);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct OH_NativeBuffer_MetadataKey(pub ::core::ffi::c_uint);
impl OH_NativeBuffer_Usage {
    pub const NATIVEBUFFER_USAGE_CPU_READ: OH_NativeBuffer_Usage = OH_NativeBuffer_Usage(1);
}
impl OH_NativeBuffer_Usage {
    /// < CPU read buffer */
    pub const NATIVEBUFFER_USAGE_CPU_WRITE: OH_NativeBuffer_Usage = OH_NativeBuffer_Usage(2);
}
impl OH_NativeBuffer_Usage {
    /// < CPU write memory */
    pub const NATIVEBUFFER_USAGE_MEM_DMA: OH_NativeBuffer_Usage = OH_NativeBuffer_Usage(8);
}
impl OH_NativeBuffer_Usage {
    /// < Direct memory access (DMA) buffer */
    pub const NATIVEBUFFER_USAGE_HW_RENDER: OH_NativeBuffer_Usage = OH_NativeBuffer_Usage(256);
}
impl OH_NativeBuffer_Usage {
    /// < For GPU write case */
    pub const NATIVEBUFFER_USAGE_HW_TEXTURE: OH_NativeBuffer_Usage = OH_NativeBuffer_Usage(512);
}
impl OH_NativeBuffer_Usage {
    /// < For GPU read case */
    pub const NATIVEBUFFER_USAGE_CPU_READ_OFTEN: OH_NativeBuffer_Usage =
        OH_NativeBuffer_Usage(65536);
}
impl OH_NativeBuffer_Usage {
    /// < Often be mapped for direct CPU reads */
    pub const NATIVEBUFFER_USAGE_ALIGNMENT_512: OH_NativeBuffer_Usage =
        OH_NativeBuffer_Usage(262144);
}
impl ::core::ops::BitOr<OH_NativeBuffer_Usage> for OH_NativeBuffer_Usage {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        OH_NativeBuffer_Usage(self.0 | other.0)
    }
}
impl ::core::ops::BitOrAssign for OH_NativeBuffer_Usage {
    #[inline]
    fn bitor_assign(&mut self, rhs: OH_NativeBuffer_Usage) {
        self.0 |= rhs.0;
    }
}
impl ::core::ops::BitAnd<OH_NativeBuffer_Usage> for OH_NativeBuffer_Usage {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        OH_NativeBuffer_Usage(self.0 & other.0)
    }
}
impl ::core::ops::BitAndAssign for OH_NativeBuffer_Usage {
    #[inline]
    fn bitand_assign(&mut self, rhs: OH_NativeBuffer_Usage) {
        self.0 &= rhs.0;
    }
}
#[repr(transparent)]
/// Indicates the usage of a native buffer.
///
///
/// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeBuffer
///
/// Available since API-level: 10
///
/// Version: 1.0
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct OH_NativeBuffer_Usage(pub ::core::ffi::c_uint);
impl OH_NativeBuffer_Format {
    /// CLUT8 format
    ///
    /// Available since API-level: 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub const NATIVEBUFFER_PIXEL_FMT_CLUT8: OH_NativeBuffer_Format = OH_NativeBuffer_Format(0);
}
impl OH_NativeBuffer_Format {
    /// CLUT1 format
    ///
    /// Available since API-level: 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub const NATIVEBUFFER_PIXEL_FMT_CLUT1: OH_NativeBuffer_Format = OH_NativeBuffer_Format(1);
}
impl OH_NativeBuffer_Format {
    /// CLUT4 format
    ///
    /// Available since API-level: 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub const NATIVEBUFFER_PIXEL_FMT_CLUT4: OH_NativeBuffer_Format = OH_NativeBuffer_Format(2);
}
impl OH_NativeBuffer_Format {
    /// CLUT4 format
    ///
    /// Available since API-level: 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub const NATIVEBUFFER_PIXEL_FMT_RGB_565: OH_NativeBuffer_Format = OH_NativeBuffer_Format(3);
}
impl OH_NativeBuffer_Format {
    /// < RGB565 format */
    pub const NATIVEBUFFER_PIXEL_FMT_RGBA_5658: OH_NativeBuffer_Format = OH_NativeBuffer_Format(4);
}
impl OH_NativeBuffer_Format {
    /// < RGBA5658 format */
    pub const NATIVEBUFFER_PIXEL_FMT_RGBX_4444: OH_NativeBuffer_Format = OH_NativeBuffer_Format(5);
}
impl OH_NativeBuffer_Format {
    /// < RGBX4444 format */
    pub const NATIVEBUFFER_PIXEL_FMT_RGBA_4444: OH_NativeBuffer_Format = OH_NativeBuffer_Format(6);
}
impl OH_NativeBuffer_Format {
    /// < RGBA4444 format */
    pub const NATIVEBUFFER_PIXEL_FMT_RGB_444: OH_NativeBuffer_Format = OH_NativeBuffer_Format(7);
}
impl OH_NativeBuffer_Format {
    /// < RGB444 format */
    pub const NATIVEBUFFER_PIXEL_FMT_RGBX_5551: OH_NativeBuffer_Format = OH_NativeBuffer_Format(8);
}
impl OH_NativeBuffer_Format {
    /// < RGBX5551 format */
    pub const NATIVEBUFFER_PIXEL_FMT_RGBA_5551: OH_NativeBuffer_Format = OH_NativeBuffer_Format(9);
}
impl OH_NativeBuffer_Format {
    /// < RGBA5551 format */
    pub const NATIVEBUFFER_PIXEL_FMT_RGB_555: OH_NativeBuffer_Format = OH_NativeBuffer_Format(10);
}
impl OH_NativeBuffer_Format {
    /// < RGB555 format */
    pub const NATIVEBUFFER_PIXEL_FMT_RGBX_8888: OH_NativeBuffer_Format = OH_NativeBuffer_Format(11);
}
impl OH_NativeBuffer_Format {
    /// < RGBX8888 format */
    pub const NATIVEBUFFER_PIXEL_FMT_RGBA_8888: OH_NativeBuffer_Format = OH_NativeBuffer_Format(12);
}
impl OH_NativeBuffer_Format {
    /// < RGBA8888 format */
    pub const NATIVEBUFFER_PIXEL_FMT_RGB_888: OH_NativeBuffer_Format = OH_NativeBuffer_Format(13);
}
impl OH_NativeBuffer_Format {
    /// < RGB888 format */
    pub const NATIVEBUFFER_PIXEL_FMT_BGR_565: OH_NativeBuffer_Format = OH_NativeBuffer_Format(14);
}
impl OH_NativeBuffer_Format {
    /// < BGR565 format */
    pub const NATIVEBUFFER_PIXEL_FMT_BGRX_4444: OH_NativeBuffer_Format = OH_NativeBuffer_Format(15);
}
impl OH_NativeBuffer_Format {
    /// < BGRX4444 format */
    pub const NATIVEBUFFER_PIXEL_FMT_BGRA_4444: OH_NativeBuffer_Format = OH_NativeBuffer_Format(16);
}
impl OH_NativeBuffer_Format {
    /// < BGRA4444 format */
    pub const NATIVEBUFFER_PIXEL_FMT_BGRX_5551: OH_NativeBuffer_Format = OH_NativeBuffer_Format(17);
}
impl OH_NativeBuffer_Format {
    /// < BGRX5551 format */
    pub const NATIVEBUFFER_PIXEL_FMT_BGRA_5551: OH_NativeBuffer_Format = OH_NativeBuffer_Format(18);
}
impl OH_NativeBuffer_Format {
    /// < BGRA5551 format */
    pub const NATIVEBUFFER_PIXEL_FMT_BGRX_8888: OH_NativeBuffer_Format = OH_NativeBuffer_Format(19);
}
impl OH_NativeBuffer_Format {
    /// < BGRX8888 format */
    pub const NATIVEBUFFER_PIXEL_FMT_BGRA_8888: OH_NativeBuffer_Format = OH_NativeBuffer_Format(20);
}
impl OH_NativeBuffer_Format {
    /// < BGRA8888 format */
    /// **
    /// * YUV422 interleaved format
    /// * @since 12
    /// */
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub const NATIVEBUFFER_PIXEL_FMT_YUV_422_I: OH_NativeBuffer_Format = OH_NativeBuffer_Format(21);
}
impl OH_NativeBuffer_Format {
    /// YCBCR422 semi-plannar format
    ///
    /// Available since API-level: 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub const NATIVEBUFFER_PIXEL_FMT_YCBCR_422_SP: OH_NativeBuffer_Format =
        OH_NativeBuffer_Format(22);
}
impl OH_NativeBuffer_Format {
    /// YCRCB422 semi-plannar format
    ///
    /// Available since API-level: 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub const NATIVEBUFFER_PIXEL_FMT_YCRCB_422_SP: OH_NativeBuffer_Format =
        OH_NativeBuffer_Format(23);
}
impl OH_NativeBuffer_Format {
    /// YCBCR420 semi-plannar format
    ///
    /// Available since API-level: 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub const NATIVEBUFFER_PIXEL_FMT_YCBCR_420_SP: OH_NativeBuffer_Format =
        OH_NativeBuffer_Format(24);
}
impl OH_NativeBuffer_Format {
    /// YCRCB420 semi-plannar format
    ///
    /// Available since API-level: 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub const NATIVEBUFFER_PIXEL_FMT_YCRCB_420_SP: OH_NativeBuffer_Format =
        OH_NativeBuffer_Format(25);
}
impl OH_NativeBuffer_Format {
    /// YCBCR422 plannar format
    ///
    /// Available since API-level: 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub const NATIVEBUFFER_PIXEL_FMT_YCBCR_422_P: OH_NativeBuffer_Format =
        OH_NativeBuffer_Format(26);
}
impl OH_NativeBuffer_Format {
    /// YCRCB422 plannar format
    ///
    /// Available since API-level: 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub const NATIVEBUFFER_PIXEL_FMT_YCRCB_422_P: OH_NativeBuffer_Format =
        OH_NativeBuffer_Format(27);
}
impl OH_NativeBuffer_Format {
    /// YCBCR420 plannar format
    ///
    /// Available since API-level: 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub const NATIVEBUFFER_PIXEL_FMT_YCBCR_420_P: OH_NativeBuffer_Format =
        OH_NativeBuffer_Format(28);
}
impl OH_NativeBuffer_Format {
    /// YCRCB420 plannar format
    ///
    /// Available since API-level: 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub const NATIVEBUFFER_PIXEL_FMT_YCRCB_420_P: OH_NativeBuffer_Format =
        OH_NativeBuffer_Format(29);
}
impl OH_NativeBuffer_Format {
    /// YUYV422 packed format
    ///
    /// Available since API-level: 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub const NATIVEBUFFER_PIXEL_FMT_YUYV_422_PKG: OH_NativeBuffer_Format =
        OH_NativeBuffer_Format(30);
}
impl OH_NativeBuffer_Format {
    /// UYVY422 packed format
    ///
    /// Available since API-level: 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub const NATIVEBUFFER_PIXEL_FMT_UYVY_422_PKG: OH_NativeBuffer_Format =
        OH_NativeBuffer_Format(31);
}
impl OH_NativeBuffer_Format {
    /// YVYU422 packed format
    ///
    /// Available since API-level: 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub const NATIVEBUFFER_PIXEL_FMT_YVYU_422_PKG: OH_NativeBuffer_Format =
        OH_NativeBuffer_Format(32);
}
impl OH_NativeBuffer_Format {
    /// VYUY422 packed format
    ///
    /// Available since API-level: 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub const NATIVEBUFFER_PIXEL_FMT_VYUY_422_PKG: OH_NativeBuffer_Format =
        OH_NativeBuffer_Format(33);
}
impl OH_NativeBuffer_Format {
    /// RGBA_1010102 packed format
    ///
    /// Available since API-level: 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub const NATIVEBUFFER_PIXEL_FMT_RGBA_1010102: OH_NativeBuffer_Format =
        OH_NativeBuffer_Format(34);
}
impl OH_NativeBuffer_Format {
    /// YCBCR420 semi-planar 10bit packed format
    ///
    /// Available since API-level: 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub const NATIVEBUFFER_PIXEL_FMT_YCBCR_P010: OH_NativeBuffer_Format =
        OH_NativeBuffer_Format(35);
}
impl OH_NativeBuffer_Format {
    /// YCRCB420 semi-planar 10bit packed format
    ///
    /// Available since API-level: 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub const NATIVEBUFFER_PIXEL_FMT_YCRCB_P010: OH_NativeBuffer_Format =
        OH_NativeBuffer_Format(36);
}
impl OH_NativeBuffer_Format {
    /// Raw 10bit packed format
    ///
    /// Available since API-level: 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub const NATIVEBUFFER_PIXEL_FMT_RAW10: OH_NativeBuffer_Format = OH_NativeBuffer_Format(37);
}
impl OH_NativeBuffer_Format {
    /// vender mask format
    ///
    /// Available since API-level: 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub const NATIVEBUFFER_PIXEL_FMT_VENDER_MASK: OH_NativeBuffer_Format =
        OH_NativeBuffer_Format(2147418112);
}
impl OH_NativeBuffer_Format {
    /// vender mask format
    ///
    /// Available since API-level: 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub const NATIVEBUFFER_PIXEL_FMT_BUTT: OH_NativeBuffer_Format =
        OH_NativeBuffer_Format(2147483647);
}
#[repr(transparent)]
/// Indicates the format of a native buffer.
///
///
/// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeBuffer
///
/// Available since API-level: 10
///
/// Version: 1.0
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct OH_NativeBuffer_Format(pub ::core::ffi::c_uint);
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
impl OH_NativeBuffer_TransformType {
    /// < No rotation
    pub const NATIVEBUFFER_ROTATE_NONE: OH_NativeBuffer_TransformType =
        OH_NativeBuffer_TransformType(0);
}
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
impl OH_NativeBuffer_TransformType {
    /// < Rotation by 90 degrees
    pub const NATIVEBUFFER_ROTATE_90: OH_NativeBuffer_TransformType =
        OH_NativeBuffer_TransformType(1);
}
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
impl OH_NativeBuffer_TransformType {
    /// < Rotation by 180 degrees
    pub const NATIVEBUFFER_ROTATE_180: OH_NativeBuffer_TransformType =
        OH_NativeBuffer_TransformType(2);
}
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
impl OH_NativeBuffer_TransformType {
    /// < Rotation by 270 degrees
    pub const NATIVEBUFFER_ROTATE_270: OH_NativeBuffer_TransformType =
        OH_NativeBuffer_TransformType(3);
}
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
impl OH_NativeBuffer_TransformType {
    /// < Flip horizontally
    pub const NATIVEBUFFER_FLIP_H: OH_NativeBuffer_TransformType = OH_NativeBuffer_TransformType(4);
}
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
impl OH_NativeBuffer_TransformType {
    /// < Flip vertically
    pub const NATIVEBUFFER_FLIP_V: OH_NativeBuffer_TransformType = OH_NativeBuffer_TransformType(5);
}
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
impl OH_NativeBuffer_TransformType {
    /// < Flip horizontally and rotate 90 degrees
    pub const NATIVEBUFFER_FLIP_H_ROT90: OH_NativeBuffer_TransformType =
        OH_NativeBuffer_TransformType(6);
}
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
impl OH_NativeBuffer_TransformType {
    /// < Flip vertically and rotate 90 degrees
    pub const NATIVEBUFFER_FLIP_V_ROT90: OH_NativeBuffer_TransformType =
        OH_NativeBuffer_TransformType(7);
}
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
impl OH_NativeBuffer_TransformType {
    /// < Flip horizontally and rotate 180 degrees
    pub const NATIVEBUFFER_FLIP_H_ROT180: OH_NativeBuffer_TransformType =
        OH_NativeBuffer_TransformType(8);
}
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
impl OH_NativeBuffer_TransformType {
    /// < Flip vertically and rotate 180 degrees
    pub const NATIVEBUFFER_FLIP_V_ROT180: OH_NativeBuffer_TransformType =
        OH_NativeBuffer_TransformType(9);
}
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
impl OH_NativeBuffer_TransformType {
    /// < Flip horizontally and rotate 270 degrees
    pub const NATIVEBUFFER_FLIP_H_ROT270: OH_NativeBuffer_TransformType =
        OH_NativeBuffer_TransformType(10);
}
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
impl OH_NativeBuffer_TransformType {
    /// < Flip vertically and rotate 270 degrees
    pub const NATIVEBUFFER_FLIP_V_ROT270: OH_NativeBuffer_TransformType =
        OH_NativeBuffer_TransformType(11);
}
#[repr(transparent)]
/// Indicates the transform type of a native buffer.
///
///
/// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeBuffer
///
/// Available since API-level: 12
///
/// Version: 1.0
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct OH_NativeBuffer_TransformType(pub ::core::ffi::c_uint);
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
impl OH_NativeBuffer_ColorGamut {
    /// < Native or default
    pub const NATIVEBUFFER_COLOR_GAMUT_NATIVE: OH_NativeBuffer_ColorGamut =
        OH_NativeBuffer_ColorGamut(0);
}
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
impl OH_NativeBuffer_ColorGamut {
    /// < Standard BT601
    pub const NATIVEBUFFER_COLOR_GAMUT_STANDARD_BT601: OH_NativeBuffer_ColorGamut =
        OH_NativeBuffer_ColorGamut(1);
}
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
impl OH_NativeBuffer_ColorGamut {
    /// < Standard BT709
    pub const NATIVEBUFFER_COLOR_GAMUT_STANDARD_BT709: OH_NativeBuffer_ColorGamut =
        OH_NativeBuffer_ColorGamut(2);
}
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
impl OH_NativeBuffer_ColorGamut {
    /// < DCI P3
    pub const NATIVEBUFFER_COLOR_GAMUT_DCI_P3: OH_NativeBuffer_ColorGamut =
        OH_NativeBuffer_ColorGamut(3);
}
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
impl OH_NativeBuffer_ColorGamut {
    /// < SRGB
    pub const NATIVEBUFFER_COLOR_GAMUT_SRGB: OH_NativeBuffer_ColorGamut =
        OH_NativeBuffer_ColorGamut(4);
}
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
impl OH_NativeBuffer_ColorGamut {
    /// < Adobe RGB
    pub const NATIVEBUFFER_COLOR_GAMUT_ADOBE_RGB: OH_NativeBuffer_ColorGamut =
        OH_NativeBuffer_ColorGamut(5);
}
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
impl OH_NativeBuffer_ColorGamut {
    /// < Display P3
    pub const NATIVEBUFFER_COLOR_GAMUT_DISPLAY_P3: OH_NativeBuffer_ColorGamut =
        OH_NativeBuffer_ColorGamut(6);
}
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
impl OH_NativeBuffer_ColorGamut {
    /// < BT2020
    pub const NATIVEBUFFER_COLOR_GAMUT_BT2020: OH_NativeBuffer_ColorGamut =
        OH_NativeBuffer_ColorGamut(7);
}
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
impl OH_NativeBuffer_ColorGamut {
    /// < BT2100 PQ
    pub const NATIVEBUFFER_COLOR_GAMUT_BT2100_PQ: OH_NativeBuffer_ColorGamut =
        OH_NativeBuffer_ColorGamut(8);
}
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
impl OH_NativeBuffer_ColorGamut {
    /// < BT2100 HLG
    pub const NATIVEBUFFER_COLOR_GAMUT_BT2100_HLG: OH_NativeBuffer_ColorGamut =
        OH_NativeBuffer_ColorGamut(9);
}
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
impl OH_NativeBuffer_ColorGamut {
    /// < Display BT2020
    pub const NATIVEBUFFER_COLOR_GAMUT_DISPLAY_BT2020: OH_NativeBuffer_ColorGamut =
        OH_NativeBuffer_ColorGamut(10);
}
#[repr(transparent)]
/// Indicates the color gamut of a native buffer.
///
///
/// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeBuffer
///
/// Available since API-level: 12
///
/// Version: 1.0
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct OH_NativeBuffer_ColorGamut(pub ::core::ffi::c_uint);
/// <b>OH_NativeBuffer</b> config.
///
/// Used to allocating new <b>OH_NativeBuffer</b> andquery parameters if existing ones.
///
///
/// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeBuffer
///
/// Available since API-level: 9
///
/// Version: 1.0
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OH_NativeBuffer_Config {
    /// < Width in pixels
    pub width: i32,
    /// < Height in pixels
    pub height: i32,
    /// < One of PixelFormat
    pub format: i32,
    /// < Combination of buffer usage
    pub usage: i32,
    /// < the stride of memory
    pub stride: i32,
}
/// Holds info for a single image plane.
///
///
///
/// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeBuffer
///
/// Available since API-level: 12
///
/// Version: 1.0
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OH_NativeBuffer_Plane {
    /// < Offset in bytes of plane.
    pub offset: u64,
    /// < Distance in bytes from the first value of one row of the image to the first value of the next row.
    pub rowStride: u32,
    /// < Distance in bytes from the first value of one column of the image to the first value of the next column.
    pub columnStride: u32,
}
/// Holds all image planes.
///
///
///
/// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeBuffer
///
/// Available since API-level: 12
///
/// Version: 1.0
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OH_NativeBuffer_Planes {
    /// < Number of distinct planes.
    pub planeCount: u32,
    /// < Array of image planes.
    pub planes: [OH_NativeBuffer_Plane; 4usize],
}
extern "C" {
    /// Alloc a <b>OH_NativeBuffer</b> that matches the passed BufferRequestConfig.
    ///
    /// A new <b>OH_NativeBuffer</b> instance is created each time this function is called.
    ///
    /// This interface needs to be used in conjunction with <b>OH_NativeBuffer_Unreference<otherwise memory leaks will occur.
    ///
    /// This interface is a non-thread-safe type interface.
    ///
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeBuffer
    /// # Arguments
    ///
    /// * `config` - Indicates the pointer to a <b>BufferRequestConfig</b> instance.
    ///
    /// # Returns
    ///
    /// * Returns the pointer to the <b>OH_NativeBuffer</b> instance created if the operation is successful,
    ///
    /// returns <b>NULL</b> otherwise.
    ///
    /// Available since API-level: 9
    ///
    /// Version: 1.0
    pub fn OH_NativeBuffer_Alloc(config: *const OH_NativeBuffer_Config) -> *mut OH_NativeBuffer;
    /// Adds the reference count of a OH_NativeBuffer.
    ///
    /// This interface needs to be used in conjunction with <b>OH_NativeBuffer_Unreference<otherwise memory leaks will occur.
    ///
    /// This interface is a non-thread-safe type interface.
    ///
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeBuffer
    /// # Arguments
    ///
    /// * `buffer` - Indicates the pointer to a <b>OH_NativeBuffer</b> instance.
    ///
    /// # Returns
    ///
    /// * Returns an error code, 0 is success, otherwise, failed.
    ///
    /// Available since API-level: 9
    ///
    /// Version: 1.0
    pub fn OH_NativeBuffer_Reference(buffer: *mut OH_NativeBuffer) -> i32;
    /// Decreases the reference count of a OH_NativeBuffer and, when the reference count reaches 0,
    /// destroys this OH_NativeBuffer.
    ///
    /// This interface is a non-thread-safe type interface.
    ///
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeBuffer
    /// # Arguments
    ///
    /// * `buffer` - Indicates the pointer to a <b>OH_NativeBuffer</b> instance.
    ///
    /// # Returns
    ///
    /// * Returns an error code, 0 is success, otherwise, failed.
    ///
    /// Available since API-level: 9
    ///
    /// Version: 1.0
    pub fn OH_NativeBuffer_Unreference(buffer: *mut OH_NativeBuffer) -> i32;
    /// Return a config of the OH_NativeBuffer in the passed OHNativeBufferConfig struct.
    ///
    /// This interface is a non-thread-safe type interface.
    ///
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeBuffer
    /// # Arguments
    ///
    /// * `buffer` - Indicates the pointer to a <b>OH_NativeBuffer</b> instance.
    ///
    /// * `config` - Indicates the pointer to the <b>NativeBufferConfig</b> of the buffer.
    ///
    /// # Returns
    ///
    /// * <b>void</b>
    ///
    /// Available since API-level: 9
    ///
    /// Version: 1.0
    pub fn OH_NativeBuffer_GetConfig(
        buffer: *mut OH_NativeBuffer,
        config: *mut OH_NativeBuffer_Config,
    );
    /// Provide direct cpu access to the OH_NativeBuffer in the process's address space.
    ///
    /// This interface needs to be used in conjunction with <b>OH_NativeBuffer_Unmap<This interface is a non-thread-safe type interface.
    ///
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeBuffer
    /// # Arguments
    ///
    /// * `buffer` - Indicates the pointer to a <b>OH_NativeBuffer</b> instance.
    ///
    /// * `virAddr` - Indicates the address of the <b>OH_NativeBuffer</b> in virtual memory.
    ///
    /// # Returns
    ///
    /// * Returns an error code, 0 is success, otherwise, failed.
    ///
    /// Available since API-level: 9
    ///
    /// Version: 1.0
    pub fn OH_NativeBuffer_Map(
        buffer: *mut OH_NativeBuffer,
        virAddr: *mut *mut ::core::ffi::c_void,
    ) -> i32;
    /// Remove direct cpu access ability of the OH_NativeBuffer in the process's address space.
    ///
    /// This interface is a non-thread-safe type interface.
    ///
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeBuffer
    /// # Arguments
    ///
    /// * `buffer` - Indicates the pointer to a <b>OH_NativeBuffer</b> instance.
    ///
    /// # Returns
    ///
    /// * Returns an error code, 0 is success, otherwise, failed.
    ///
    /// Available since API-level: 9
    ///
    /// Version: 1.0
    pub fn OH_NativeBuffer_Unmap(buffer: *mut OH_NativeBuffer) -> i32;
    /// Get the systen wide unique sequence number of the OH_NativeBuffer.
    ///
    /// This interface is a non-thread-safe type interface.
    ///
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeBuffer
    /// # Arguments
    ///
    /// * `buffer` - Indicates the pointer to a <b>OH_NativeBuffer</b> instance.
    ///
    /// # Returns
    ///
    /// * Returns the sequence number, which is unique for each OH_NativeBuffer.
    ///
    /// Available since API-level: 9
    ///
    /// Version: 1.0
    pub fn OH_NativeBuffer_GetSeqNum(buffer: *mut OH_NativeBuffer) -> u32;
    /// Provide direct cpu access to the potentially multi-plannar OH_NativeBuffer in the process's address space.
    ///
    /// This interface is a non-thread-safe type interface.
    ///
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeBuffer
    /// # Arguments
    ///
    /// * `buffer` - Indicates the pointer to a <b>OH_NativeBuffer</b> instance.
    ///
    /// * `virAddr` - Indicates the address of the <b>OH_NativeBuffer</b> in virtual memory.
    ///
    /// * `outPlanes` - Indicates all image planes that contain the pixel data.
    ///
    /// # Returns
    ///
    /// * Returns an error code, 0 is sucess, otherwise, failed.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_NativeBuffer_MapPlanes(
        buffer: *mut OH_NativeBuffer,
        virAddr: *mut *mut ::core::ffi::c_void,
        outPlanes: *mut OH_NativeBuffer_Planes,
    ) -> i32;
    /// Converts an <b>OHNativeWindowBuffer</b> instance to an <b>OH_NativeBuffer</b>.
    ///
    /// This interface is a non-thread-safe type interface.
    ///
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeBuffer
    /// # Arguments
    ///
    /// * `nativeWindowBuffer` - Indicates the pointer to a <b>OHNativeWindowBuffer</b> instance.
    ///
    /// * `buffer` - Indicates the pointer to a <b>OH_NativeBuffer</b> pointer.
    ///
    /// # Returns
    ///
    /// * Returns an error code, 0 is sucess, otherwise, failed.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_NativeBuffer_FromNativeWindowBuffer(
        nativeWindowBuffer: *mut OHNativeWindowBuffer,
        buffer: *mut *mut OH_NativeBuffer,
    ) -> i32;
    /// Set the color space of the OH_NativeBuffer.
    ///
    /// This interface is a non-thread-safe type interface.
    ///
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeBuffer
    /// # Arguments
    ///
    /// * `buffer` - Indicates the pointer to a <b>OH_NativeBuffer</b> instance.
    ///
    /// * `colorSpace` - Indicates the color space of native buffer, see <b>OH_NativeBuffer_ColorSpace</b>.
    ///
    /// # Returns
    ///
    /// * Returns an error code, 0 is success, otherwise, failed.
    ///
    /// Available since API-level: 11
    ///
    /// Version: 1.0
    #[cfg(feature = "api-11")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-11")))]
    pub fn OH_NativeBuffer_SetColorSpace(
        buffer: *mut OH_NativeBuffer,
        colorSpace: OH_NativeBuffer_ColorSpace,
    ) -> i32;
    /// Get the color space of the OH_NativeBuffer.
    ///
    /// This interface is a non-thread-safe type interface.
    ///
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeBuffer
    /// # Arguments
    ///
    /// * `buffer` - Indicates the pointer to a <b>OH_NativeBuffer</b> instance.
    ///
    /// * `colorSpace` - Indicates the color space of native buffer, see <b>OH_NativeBuffer_ColorSpace</b>.
    ///
    /// # Returns
    ///
    /// * [`NATIVE_ERROR_OK`] 0 - Success.
    /// [`NATIVE_ERROR_INVALID_ARGUMENTS`] 40001000 - buffer is NULL.
    /// [`NATIVE_ERROR_BUFFER_STATE_INVALID`] 41207000 - Incorrect colorSpace state.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_NativeBuffer_GetColorSpace(
        buffer: *mut OH_NativeBuffer,
        colorSpace: *mut OH_NativeBuffer_ColorSpace,
    ) -> i32;
    /// Set the metadata type of the OH_NativeBuffer.
    ///
    /// This interface is a non-thread-safe type interface.
    ///
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeBuffer
    /// # Arguments
    ///
    /// * `buffer` - Indicates the pointer to a <b>OH_NativeBuffer</b> instance.
    ///
    /// * `metadataKey` - Indicates the metadata type of native buffer, see <b>OH_NativeBuffer_MetadataKey</b>.
    ///
    /// * `size` - Indicates the size of a uint8_t vector.
    ///
    /// * `metadata` - Indicates the pointer to a uint8_t vector.
    ///
    /// # Returns
    ///
    /// * [`NATIVE_ERROR_OK`] 0 - Success.
    /// [`NATIVE_ERROR_INVALID_ARGUMENTS`] 40001000 - buffer or metadata is NULL.
    /// [`NATIVE_ERROR_BUFFER_STATE_INVALID`] 41207000 - Incorrect metadata state.
    /// [`NATIVE_ERROR_UNSUPPORTED`] 50102000 - Unsupported metadata key.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_NativeBuffer_SetMetadataValue(
        buffer: *mut OH_NativeBuffer,
        metadataKey: OH_NativeBuffer_MetadataKey,
        size: i32,
        metadata: *mut u8,
    ) -> i32;
    /// Set the metadata type of the OH_NativeBuffer.
    ///
    /// This interface is a non-thread-safe type interface.
    ///
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeBuffer
    /// # Arguments
    ///
    /// * `buffer` - Indicates the pointer to a <b>OH_NativeBuffer</b> instance.
    ///
    /// * `metadataKey` - Indicates the metadata type of native buffer, see <b>OH_NativeBuffer_MetadataKey</b>.
    ///
    /// * `size` - Indicates the size of a uint8_t vector.
    ///
    /// * `metadata` - Indicates the pointer to a uint8_t vector.
    ///
    /// # Returns
    ///
    /// * [`NATIVE_ERROR_OK`] 0 - Success.
    /// [`NATIVE_ERROR_INVALID_ARGUMENTS`] 40001000 - buffer, metadata, or size is NULL.
    /// [`NATIVE_ERROR_BUFFER_STATE_INVALID`] 41207000 - Incorrect metadata state.
    /// [`NATIVE_ERROR_UNSUPPORTED`] 50102000 - Unsupported metadata key.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_NativeBuffer_GetMetadataValue(
        buffer: *mut OH_NativeBuffer,
        metadataKey: OH_NativeBuffer_MetadataKey,
        size: *mut i32,
        metadata: *mut *mut u8,
    ) -> i32;
}
