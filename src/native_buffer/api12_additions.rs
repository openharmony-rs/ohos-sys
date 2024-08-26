#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::native_buffer::{
    OH_NativeBuffer, OH_NativeBuffer_ColorSpace, OH_NativeBuffer_Format, OH_NativeBuffer_Usage,
};
use crate::native_window::OHNativeWindowBuffer;

impl OH_NativeBuffer_MetadataType {
    pub const OH_VIDEO_HDR_HLG: OH_NativeBuffer_MetadataType = OH_NativeBuffer_MetadataType(0);
}
impl OH_NativeBuffer_MetadataType {
    pub const OH_VIDEO_HDR_HDR10: OH_NativeBuffer_MetadataType = OH_NativeBuffer_MetadataType(1);
}
impl OH_NativeBuffer_MetadataType {
    pub const OH_VIDEO_HDR_VIVID: OH_NativeBuffer_MetadataType = OH_NativeBuffer_MetadataType(2);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct OH_NativeBuffer_MetadataType(pub ::core::ffi::c_uint);
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

impl OH_NativeBuffer_Format {
    /** CLUT8 format
    @since 12*/
    pub const NATIVEBUFFER_PIXEL_FMT_CLUT8: OH_NativeBuffer_Format = OH_NativeBuffer_Format(0);
}
impl OH_NativeBuffer_Format {
    /** CLUT1 format
    @since 12*/
    pub const NATIVEBUFFER_PIXEL_FMT_CLUT1: OH_NativeBuffer_Format = OH_NativeBuffer_Format(1);
}
impl OH_NativeBuffer_Format {
    /** CLUT4 format
    @since 12*/
    pub const NATIVEBUFFER_PIXEL_FMT_CLUT4: OH_NativeBuffer_Format = OH_NativeBuffer_Format(2);
}

impl OH_NativeBuffer_Format {
    #[doc = " < BGRA8888 format */\n**\n* YUV422 interleaved format\n* @since 12\n*/"]
    pub const NATIVEBUFFER_PIXEL_FMT_YUV_422_I: OH_NativeBuffer_Format = OH_NativeBuffer_Format(21);
}
impl OH_NativeBuffer_Format {
    /** YCBCR422 semi-plannar format
    @since 12*/
    pub const NATIVEBUFFER_PIXEL_FMT_YCBCR_422_SP: OH_NativeBuffer_Format =
        OH_NativeBuffer_Format(22);
}
impl OH_NativeBuffer_Format {
    /** YCRCB422 semi-plannar format
    @since 12*/
    pub const NATIVEBUFFER_PIXEL_FMT_YCRCB_422_SP: OH_NativeBuffer_Format =
        OH_NativeBuffer_Format(23);
}
impl OH_NativeBuffer_Format {
    /** YCBCR420 semi-plannar format
    @since 12*/
    pub const NATIVEBUFFER_PIXEL_FMT_YCBCR_420_SP: OH_NativeBuffer_Format =
        OH_NativeBuffer_Format(24);
}
impl OH_NativeBuffer_Format {
    /** YCRCB420 semi-plannar format
    @since 12*/
    pub const NATIVEBUFFER_PIXEL_FMT_YCRCB_420_SP: OH_NativeBuffer_Format =
        OH_NativeBuffer_Format(25);
}
impl OH_NativeBuffer_Format {
    /** YCBCR422 plannar format
    @since 12*/
    pub const NATIVEBUFFER_PIXEL_FMT_YCBCR_422_P: OH_NativeBuffer_Format =
        OH_NativeBuffer_Format(26);
}
impl OH_NativeBuffer_Format {
    /** YCRCB422 plannar format
    @since 12*/
    pub const NATIVEBUFFER_PIXEL_FMT_YCRCB_422_P: OH_NativeBuffer_Format =
        OH_NativeBuffer_Format(27);
}
impl OH_NativeBuffer_Format {
    /** YCBCR420 plannar format
    @since 12*/
    pub const NATIVEBUFFER_PIXEL_FMT_YCBCR_420_P: OH_NativeBuffer_Format =
        OH_NativeBuffer_Format(28);
}
impl OH_NativeBuffer_Format {
    /** YCRCB420 plannar format
    @since 12*/
    pub const NATIVEBUFFER_PIXEL_FMT_YCRCB_420_P: OH_NativeBuffer_Format =
        OH_NativeBuffer_Format(29);
}
impl OH_NativeBuffer_Format {
    /** YUYV422 packed format
    @since 12*/
    pub const NATIVEBUFFER_PIXEL_FMT_YUYV_422_PKG: OH_NativeBuffer_Format =
        OH_NativeBuffer_Format(30);
}
impl OH_NativeBuffer_Format {
    /** UYVY422 packed format
    @since 12*/
    pub const NATIVEBUFFER_PIXEL_FMT_UYVY_422_PKG: OH_NativeBuffer_Format =
        OH_NativeBuffer_Format(31);
}
impl OH_NativeBuffer_Format {
    /** YVYU422 packed format
    @since 12*/
    pub const NATIVEBUFFER_PIXEL_FMT_YVYU_422_PKG: OH_NativeBuffer_Format =
        OH_NativeBuffer_Format(32);
}
impl OH_NativeBuffer_Format {
    /** VYUY422 packed format
    @since 12*/
    pub const NATIVEBUFFER_PIXEL_FMT_VYUY_422_PKG: OH_NativeBuffer_Format =
        OH_NativeBuffer_Format(33);
}
impl OH_NativeBuffer_Format {
    /** RGBA_1010102 packed format
    @since 12*/
    pub const NATIVEBUFFER_PIXEL_FMT_RGBA_1010102: OH_NativeBuffer_Format =
        OH_NativeBuffer_Format(34);
}
impl OH_NativeBuffer_Format {
    /** YCBCR420 semi-planar 10bit packed format
    @since 12*/
    pub const NATIVEBUFFER_PIXEL_FMT_YCBCR_P010: OH_NativeBuffer_Format =
        OH_NativeBuffer_Format(35);
}
impl OH_NativeBuffer_Format {
    /** YCRCB420 semi-planar 10bit packed format
    @since 12*/
    pub const NATIVEBUFFER_PIXEL_FMT_YCRCB_P010: OH_NativeBuffer_Format =
        OH_NativeBuffer_Format(36);
}
impl OH_NativeBuffer_Format {
    /** Raw 10bit packed format
    @since 12*/
    pub const NATIVEBUFFER_PIXEL_FMT_RAW10: OH_NativeBuffer_Format = OH_NativeBuffer_Format(37);
}
impl OH_NativeBuffer_Format {
    /** vender mask format
    @since 12*/
    pub const NATIVEBUFFER_PIXEL_FMT_VENDER_MASK: OH_NativeBuffer_Format =
        OH_NativeBuffer_Format(2147418112);
}

impl OH_NativeBuffer_TransformType {
    ///< No rotation
    pub const NATIVEBUFFER_ROTATE_NONE: OH_NativeBuffer_TransformType =
        OH_NativeBuffer_TransformType(0);
}
impl OH_NativeBuffer_TransformType {
    ///< Rotation by 90 degrees
    pub const NATIVEBUFFER_ROTATE_90: OH_NativeBuffer_TransformType =
        OH_NativeBuffer_TransformType(1);
}
impl OH_NativeBuffer_TransformType {
    ///< Rotation by 180 degrees
    pub const NATIVEBUFFER_ROTATE_180: OH_NativeBuffer_TransformType =
        OH_NativeBuffer_TransformType(2);
}
impl OH_NativeBuffer_TransformType {
    ///< Rotation by 270 degrees
    pub const NATIVEBUFFER_ROTATE_270: OH_NativeBuffer_TransformType =
        OH_NativeBuffer_TransformType(3);
}
impl OH_NativeBuffer_TransformType {
    ///< Flip horizontally
    pub const NATIVEBUFFER_FLIP_H: OH_NativeBuffer_TransformType = OH_NativeBuffer_TransformType(4);
}
impl OH_NativeBuffer_TransformType {
    ///< Flip vertically
    pub const NATIVEBUFFER_FLIP_V: OH_NativeBuffer_TransformType = OH_NativeBuffer_TransformType(5);
}
impl OH_NativeBuffer_TransformType {
    ///< Flip horizontally and rotate 90 degrees
    pub const NATIVEBUFFER_FLIP_H_ROT90: OH_NativeBuffer_TransformType =
        OH_NativeBuffer_TransformType(6);
}
impl OH_NativeBuffer_TransformType {
    ///< Flip vertically and rotate 90 degrees
    pub const NATIVEBUFFER_FLIP_V_ROT90: OH_NativeBuffer_TransformType =
        OH_NativeBuffer_TransformType(7);
}
impl OH_NativeBuffer_TransformType {
    ///< Flip horizontally and rotate 180 degrees
    pub const NATIVEBUFFER_FLIP_H_ROT180: OH_NativeBuffer_TransformType =
        OH_NativeBuffer_TransformType(8);
}
impl OH_NativeBuffer_TransformType {
    ///< Flip vertically and rotate 180 degrees
    pub const NATIVEBUFFER_FLIP_V_ROT180: OH_NativeBuffer_TransformType =
        OH_NativeBuffer_TransformType(9);
}
impl OH_NativeBuffer_TransformType {
    ///< Flip horizontally and rotate 270 degrees
    pub const NATIVEBUFFER_FLIP_H_ROT270: OH_NativeBuffer_TransformType =
        OH_NativeBuffer_TransformType(10);
}
impl OH_NativeBuffer_TransformType {
    ///< Flip vertically and rotate 270 degrees
    pub const NATIVEBUFFER_FLIP_V_ROT270: OH_NativeBuffer_TransformType =
        OH_NativeBuffer_TransformType(11);
}
#[repr(transparent)]
/** @brief Indicates the transform type of a native buffer.

@syscap SystemCapability.Graphic.Graphic2D.NativeBuffer
@since 12
@version 1.0*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct OH_NativeBuffer_TransformType(pub ::core::ffi::c_uint);

impl OH_NativeBuffer_ColorGamut {
    ///< Native or default
    pub const NATIVEBUFFER_COLOR_GAMUT_NATIVE: OH_NativeBuffer_ColorGamut =
        OH_NativeBuffer_ColorGamut(0);
}
impl OH_NativeBuffer_ColorGamut {
    ///< Standard BT601
    pub const NATIVEBUFFER_COLOR_GAMUT_STANDARD_BT601: OH_NativeBuffer_ColorGamut =
        OH_NativeBuffer_ColorGamut(1);
}
impl OH_NativeBuffer_ColorGamut {
    ///< Standard BT709
    pub const NATIVEBUFFER_COLOR_GAMUT_STANDARD_BT709: OH_NativeBuffer_ColorGamut =
        OH_NativeBuffer_ColorGamut(2);
}
impl OH_NativeBuffer_ColorGamut {
    ///< DCI P3
    pub const NATIVEBUFFER_COLOR_GAMUT_DCI_P3: OH_NativeBuffer_ColorGamut =
        OH_NativeBuffer_ColorGamut(3);
}
impl OH_NativeBuffer_ColorGamut {
    ///< SRGB
    pub const NATIVEBUFFER_COLOR_GAMUT_SRGB: OH_NativeBuffer_ColorGamut =
        OH_NativeBuffer_ColorGamut(4);
}
impl OH_NativeBuffer_ColorGamut {
    ///< Adobe RGB
    pub const NATIVEBUFFER_COLOR_GAMUT_ADOBE_RGB: OH_NativeBuffer_ColorGamut =
        OH_NativeBuffer_ColorGamut(5);
}
impl OH_NativeBuffer_ColorGamut {
    ///< Display P3
    pub const NATIVEBUFFER_COLOR_GAMUT_DISPLAY_P3: OH_NativeBuffer_ColorGamut =
        OH_NativeBuffer_ColorGamut(6);
}
impl OH_NativeBuffer_ColorGamut {
    ///< BT2020
    pub const NATIVEBUFFER_COLOR_GAMUT_BT2020: OH_NativeBuffer_ColorGamut =
        OH_NativeBuffer_ColorGamut(7);
}
impl OH_NativeBuffer_ColorGamut {
    ///< BT2100 PQ
    pub const NATIVEBUFFER_COLOR_GAMUT_BT2100_PQ: OH_NativeBuffer_ColorGamut =
        OH_NativeBuffer_ColorGamut(8);
}
impl OH_NativeBuffer_ColorGamut {
    ///< BT2100 HLG
    pub const NATIVEBUFFER_COLOR_GAMUT_BT2100_HLG: OH_NativeBuffer_ColorGamut =
        OH_NativeBuffer_ColorGamut(9);
}
impl OH_NativeBuffer_ColorGamut {
    ///< Display BT2020
    pub const NATIVEBUFFER_COLOR_GAMUT_DISPLAY_BT2020: OH_NativeBuffer_ColorGamut =
        OH_NativeBuffer_ColorGamut(10);
}
#[repr(transparent)]
/** @brief Indicates the color gamut of a native buffer.

@syscap SystemCapability.Graphic.Graphic2D.NativeBuffer
@since 12
@version 1.0*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct OH_NativeBuffer_ColorGamut(pub ::core::ffi::c_uint);

/** @brief Holds info for a single image plane. \n

@syscap SystemCapability.Graphic.Graphic2D.NativeBuffer
@since 12
@version 1.0*/
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OH_NativeBuffer_Plane {
    ///< Offset in bytes of plane.
    pub offset: u64,
    ///< Distance in bytes from the first value of one row of the image to the first value of the next row.
    pub rowStride: u32,
    ///< Distance in bytes from the first value of one column of the image to the first value of the next column.
    pub columnStride: u32,
}
/** @brief Holds all image planes. \n

@syscap SystemCapability.Graphic.Graphic2D.NativeBuffer
@since 12
@version 1.0*/
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OH_NativeBuffer_Planes {
    ///< Number of distinct planes.
    pub planeCount: u32,
    ///< Array of image planes.
    pub planes: [OH_NativeBuffer_Plane; 4usize],
}

extern "C" {
    /** @brief Provide direct cpu access to the potentially multi-plannar OH_NativeBuffer in the process's address space.

    @syscap SystemCapability.Graphic.Graphic2D.NativeBuffer
    @param buffer Indicates the pointer to a <b>OH_NativeBuffer</b> instance.
    @param virAddr Indicates the address of the <b>OH_NativeBuffer</b> in virtual memory.
    @param outPlanes Indicates all image planes that contain the pixel data.
    @return Returns an error code, 0 is sucess, otherwise, failed.
    @since 12
    @version 1.0*/
    pub fn OH_NativeBuffer_MapPlanes(
        buffer: *mut OH_NativeBuffer,
        virAddr: *mut *mut ::core::ffi::c_void,
        outPlanes: *mut OH_NativeBuffer_Planes,
    ) -> i32;
    /** @brief Converts an <b>OHNativeWindowBuffer</b> instance to an <b>OH_NativeBuffer</b>.

    @syscap SystemCapability.Graphic.Graphic2D.NativeBuffer
    @param nativeWindowBuffer Indicates the pointer to a <b>OHNativeWindowBuffer</b> instance.
    @param buffer Indicates the pointer to a <b>OH_NativeBuffer</b> pointer.
    @return Returns an error code, 0 is sucess, otherwise, failed.
    @since 12
    @version 1.0*/
    pub fn OH_NativeBuffer_FromNativeWindowBuffer(
        nativeWindowBuffer: *mut OHNativeWindowBuffer,
        buffer: *mut *mut OH_NativeBuffer,
    ) -> i32;

    /** @brief Get the color space of the OH_NativeBuffer.

    @syscap SystemCapability.Graphic.Graphic2D.NativeBuffer
    @param buffer Indicates the pointer to a <b>OH_NativeBuffer</b> instance.
    @param colorSpace Indicates the color space of native buffer, see <b>OH_NativeBuffer_ColorSpace</b>.
    @return {@link NATIVE_ERROR_OK} 0 - Success.
        {@link NATIVE_ERROR_INVALID_ARGUMENTS} 40001000 - buffer is NULL.
        {@link NATIVE_ERROR_BUFFER_STATE_INVALID} 41207000 - Incorrect colorSpace state.
    @since 12
    @version 1.0*/
    pub fn OH_NativeBuffer_GetColorSpace(
        buffer: *mut OH_NativeBuffer,
        colorSpace: *mut OH_NativeBuffer_ColorSpace,
    ) -> i32;
    /** @brief Set the metadata type of the OH_NativeBuffer.

    @syscap SystemCapability.Graphic.Graphic2D.NativeBuffer
    @param buffer Indicates the pointer to a <b>OH_NativeBuffer</b> instance.
    @param metadataKey Indicates the metadata type of native buffer, see <b>OH_NativeBuffer_MetadataKey</b>.
    @param size Indicates the size of a uint8_t vector.
    @param metadata Indicates the pointer to a uint8_t vector.
    @return {@link NATIVE_ERROR_OK} 0 - Success.
        {@link NATIVE_ERROR_INVALID_ARGUMENTS} 40001000 - buffer or metadata is NULL.
        {@link NATIVE_ERROR_BUFFER_STATE_INVALID} 41207000 - Incorrect metadata state.
        {@link NATIVE_ERROR_UNSUPPORTED} 50102000 - Unsupported metadata key.
    @since 12
    @version 1.0*/
    pub fn OH_NativeBuffer_SetMetadataValue(
        buffer: *mut OH_NativeBuffer,
        metadataKey: OH_NativeBuffer_MetadataKey,
        size: i32,
        metadata: *mut u8,
    ) -> i32;
    /** @brief Set the metadata type of the OH_NativeBuffer.

    @syscap SystemCapability.Graphic.Graphic2D.NativeBuffer
    @param buffer Indicates the pointer to a <b>OH_NativeBuffer</b> instance.
    @param metadataKey Indicates the metadata type of native buffer, see <b>OH_NativeBuffer_MetadataKey</b>.
    @param size Indicates the size of a uint8_t vector.
    @param metadata Indicates the pointer to a uint8_t vector.
    @return {@link NATIVE_ERROR_OK} 0 - Success.
        {@link NATIVE_ERROR_INVALID_ARGUMENTS} 40001000 - buffer, metadata, or size is NULL.
        {@link NATIVE_ERROR_BUFFER_STATE_INVALID} 41207000 - Incorrect metadata state.
        {@link NATIVE_ERROR_UNSUPPORTED} 50102000 - Unsupported metadata key.
    @since 12
    @version 1.0*/
    pub fn OH_NativeBuffer_GetMetadataValue(
        buffer: *mut OH_NativeBuffer,
        metadataKey: OH_NativeBuffer_MetadataKey,
        size: *mut i32,
        metadata: *mut *mut u8,
    ) -> i32;
}
