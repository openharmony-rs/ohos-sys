#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use super::OH_NativeBuffer;

impl OH_NativeBuffer_ColorSpace {
    /// None color space
    pub const OH_COLORSPACE_NONE: OH_NativeBuffer_ColorSpace = OH_NativeBuffer_ColorSpace(0);
}
impl OH_NativeBuffer_ColorSpace {
    /// COLORPRIMARIES_BT601_P | (TRANSFUNC_BT709 << 8) | (MATRIX_BT601_P << 16) | (RANGE_FULL << 21)
    pub const OH_COLORSPACE_BT601_EBU_FULL: OH_NativeBuffer_ColorSpace =
        OH_NativeBuffer_ColorSpace(1);
}
impl OH_NativeBuffer_ColorSpace {
    /// COLORPRIMARIES_BT601_N | (TRANSFUNC_BT709 << 8) | (MATRIX_BT601_N << 16) | (RANGE_FULL << 21)
    pub const OH_COLORSPACE_BT601_SMPTE_C_FULL: OH_NativeBuffer_ColorSpace =
        OH_NativeBuffer_ColorSpace(2);
}
impl OH_NativeBuffer_ColorSpace {
    /// COLORPRIMARIES_BT709 | (TRANSFUNC_BT709 << 8) | (MATRIX_BT709 << 16) | (RANGE_FULL << 21)
    pub const OH_COLORSPACE_BT709_FULL: OH_NativeBuffer_ColorSpace = OH_NativeBuffer_ColorSpace(3);
}
impl OH_NativeBuffer_ColorSpace {
    /// COLORPRIMARIES_BT2020 | (TRANSFUNC_HLG << 8) | (MATRIX_BT2020 << 16) | (RANGE_FULL << 21)
    pub const OH_COLORSPACE_BT2020_HLG_FULL: OH_NativeBuffer_ColorSpace =
        OH_NativeBuffer_ColorSpace(4);
}
impl OH_NativeBuffer_ColorSpace {
    /// COLORPRIMARIES_BT2020 | (TRANSFUNC_PQ << 8) | (MATRIX_BT2020 << 16) | (RANGE_FULL << 21)
    pub const OH_COLORSPACE_BT2020_PQ_FULL: OH_NativeBuffer_ColorSpace =
        OH_NativeBuffer_ColorSpace(5);
}
impl OH_NativeBuffer_ColorSpace {
    /// COLORPRIMARIES_BT601_P | (TRANSFUNC_BT709 << 8) | (MATRIX_BT601_P << 16) | (RANGE_LIMITED << 21)
    pub const OH_COLORSPACE_BT601_EBU_LIMIT: OH_NativeBuffer_ColorSpace =
        OH_NativeBuffer_ColorSpace(6);
}
impl OH_NativeBuffer_ColorSpace {
    /// COLORPRIMARIES_BT601_N | (TRANSFUNC_BT709 << 8) | (MATRIX_BT601_N << 16) | (RANGE_LIMITED << 21)
    pub const OH_COLORSPACE_BT601_SMPTE_C_LIMIT: OH_NativeBuffer_ColorSpace =
        OH_NativeBuffer_ColorSpace(7);
}
impl OH_NativeBuffer_ColorSpace {
    /// COLORPRIMARIES_BT709 | (TRANSFUNC_BT709 << 8) | (MATRIX_BT709 << 16) | (RANGE_LIMITED << 21)
    pub const OH_COLORSPACE_BT709_LIMIT: OH_NativeBuffer_ColorSpace = OH_NativeBuffer_ColorSpace(8);
}
impl OH_NativeBuffer_ColorSpace {
    /// COLORPRIMARIES_BT2020 | (TRANSFUNC_HLG << 8) | (MATRIX_BT2020 << 16) | (RANGE_LIMITED << 21)
    pub const OH_COLORSPACE_BT2020_HLG_LIMIT: OH_NativeBuffer_ColorSpace =
        OH_NativeBuffer_ColorSpace(9);
}
impl OH_NativeBuffer_ColorSpace {
    /// COLORPRIMARIES_BT2020 | (TRANSFUNC_PQ << 8) | (MATRIX_BT2020 << 16) | (RANGE_LIMITED << 21)
    pub const OH_COLORSPACE_BT2020_PQ_LIMIT: OH_NativeBuffer_ColorSpace =
        OH_NativeBuffer_ColorSpace(10);
}
impl OH_NativeBuffer_ColorSpace {
    /// COLORPRIMARIES_SRGB | (TRANSFUNC_SRGB << 8) | (MATRIX_BT601_N << 16) | (RANGE_FULL << 21)
    pub const OH_COLORSPACE_SRGB_FULL: OH_NativeBuffer_ColorSpace = OH_NativeBuffer_ColorSpace(11);
}
impl OH_NativeBuffer_ColorSpace {
    /// COLORPRIMARIES_P3_D65 | (TRANSFUNC_SRGB << 8) | (MATRIX_P3 << 16) | (RANGE_FULL << 21)
    pub const OH_COLORSPACE_P3_FULL: OH_NativeBuffer_ColorSpace = OH_NativeBuffer_ColorSpace(12);
}
impl OH_NativeBuffer_ColorSpace {
    /// COLORPRIMARIES_P3_D65 | (TRANSFUNC_HLG << 8) | (MATRIX_P3 << 16) | (RANGE_FULL << 21)
    pub const OH_COLORSPACE_P3_HLG_FULL: OH_NativeBuffer_ColorSpace =
        OH_NativeBuffer_ColorSpace(13);
}
impl OH_NativeBuffer_ColorSpace {
    /// COLORPRIMARIES_P3_D65 | (TRANSFUNC_PQ << 8) | (MATRIX_P3 << 16) | (RANGE_FULL << 21)
    pub const OH_COLORSPACE_P3_PQ_FULL: OH_NativeBuffer_ColorSpace = OH_NativeBuffer_ColorSpace(14);
}
impl OH_NativeBuffer_ColorSpace {
    /// COLORPRIMARIES_ADOBERGB | (TRANSFUNC_ADOBERGB << 8) | (MATRIX_ADOBERGB << 16) | (RANGE_FULL << 21)
    pub const OH_COLORSPACE_ADOBERGB_FULL: OH_NativeBuffer_ColorSpace =
        OH_NativeBuffer_ColorSpace(15);
}
impl OH_NativeBuffer_ColorSpace {
    /// COLORPRIMARIES_SRGB | (TRANSFUNC_SRGB << 8) | (MATRIX_BT601_N << 16) | (RANGE_LIMITED << 21)
    pub const OH_COLORSPACE_SRGB_LIMIT: OH_NativeBuffer_ColorSpace = OH_NativeBuffer_ColorSpace(16);
}
impl OH_NativeBuffer_ColorSpace {
    /// COLORPRIMARIES_P3_D65 | (TRANSFUNC_SRGB << 8) | (MATRIX_P3 << 16) | (RANGE_LIMITED << 21)
    pub const OH_COLORSPACE_P3_LIMIT: OH_NativeBuffer_ColorSpace = OH_NativeBuffer_ColorSpace(17);
}
impl OH_NativeBuffer_ColorSpace {
    /// COLORPRIMARIES_P3_D65 | (TRANSFUNC_HLG << 8) | (MATRIX_P3 << 16) | (RANGE_LIMITED << 21)
    pub const OH_COLORSPACE_P3_HLG_LIMIT: OH_NativeBuffer_ColorSpace =
        OH_NativeBuffer_ColorSpace(18);
}
impl OH_NativeBuffer_ColorSpace {
    /// COLORPRIMARIES_P3_D65 | (TRANSFUNC_PQ << 8) | (MATRIX_P3 << 16) | (RANGE_LIMITED << 21)
    pub const OH_COLORSPACE_P3_PQ_LIMIT: OH_NativeBuffer_ColorSpace =
        OH_NativeBuffer_ColorSpace(19);
}
impl OH_NativeBuffer_ColorSpace {
    /// COLORPRIMARIES_ADOBERGB | (TRANSFUNC_ADOBERGB << 8) | (MATRIX_ADOBERGB << 16) | (RANGE_LIMITED << 21)
    pub const OH_COLORSPACE_ADOBERGB_LIMIT: OH_NativeBuffer_ColorSpace =
        OH_NativeBuffer_ColorSpace(20);
}
impl OH_NativeBuffer_ColorSpace {
    /// COLORPRIMARIES_SRGB | (TRANSFUNC_LINEAR << 8)
    pub const OH_COLORSPACE_LINEAR_SRGB: OH_NativeBuffer_ColorSpace =
        OH_NativeBuffer_ColorSpace(21);
}
impl OH_NativeBuffer_ColorSpace {
    /// equal to OH_COLORSPACE_LINEAR_SRGB
    pub const OH_COLORSPACE_LINEAR_BT709: OH_NativeBuffer_ColorSpace =
        OH_NativeBuffer_ColorSpace(22);
}
impl OH_NativeBuffer_ColorSpace {
    /// COLORPRIMARIES_P3_D65 | (TRANSFUNC_LINEAR << 8)
    pub const OH_COLORSPACE_LINEAR_P3: OH_NativeBuffer_ColorSpace = OH_NativeBuffer_ColorSpace(23);
}
impl OH_NativeBuffer_ColorSpace {
    /// COLORPRIMARIES_BT2020 | (TRANSFUNC_LINEAR << 8)
    pub const OH_COLORSPACE_LINEAR_BT2020: OH_NativeBuffer_ColorSpace =
        OH_NativeBuffer_ColorSpace(24);
}
impl OH_NativeBuffer_ColorSpace {
    /// equal to OH_COLORSPACE_SRGB_FULL
    pub const OH_COLORSPACE_DISPLAY_SRGB: OH_NativeBuffer_ColorSpace =
        OH_NativeBuffer_ColorSpace(25);
}
impl OH_NativeBuffer_ColorSpace {
    /// equal to OH_COLORSPACE_P3_FULL
    pub const OH_COLORSPACE_DISPLAY_P3_SRGB: OH_NativeBuffer_ColorSpace =
        OH_NativeBuffer_ColorSpace(26);
}
impl OH_NativeBuffer_ColorSpace {
    /// equal to OH_COLORSPACE_P3_HLG_FULL
    pub const OH_COLORSPACE_DISPLAY_P3_HLG: OH_NativeBuffer_ColorSpace =
        OH_NativeBuffer_ColorSpace(27);
}
impl OH_NativeBuffer_ColorSpace {
    /// equal to OH_COLORSPACE_P3_PQ_FULL
    pub const OH_COLORSPACE_DISPLAY_P3_PQ: OH_NativeBuffer_ColorSpace =
        OH_NativeBuffer_ColorSpace(28);
}
impl OH_NativeBuffer_ColorSpace {
    /// COLORPRIMARIES_BT2020 | (TRANSFUNC_SRGB << 8) | (MATRIX_BT2020 << 16) | (RANGE_FULL << 21)
    pub const OH_COLORSPACE_DISPLAY_BT2020_SRGB: OH_NativeBuffer_ColorSpace =
        OH_NativeBuffer_ColorSpace(29);
}
impl OH_NativeBuffer_ColorSpace {
    /// equal to OH_COLORSPACE_BT2020_HLG_FULL
    pub const OH_COLORSPACE_DISPLAY_BT2020_HLG: OH_NativeBuffer_ColorSpace =
        OH_NativeBuffer_ColorSpace(30);
}
impl OH_NativeBuffer_ColorSpace {
    /// equal to OH_COLORSPACE_BT2020_PQ_FULL
    pub const OH_COLORSPACE_DISPLAY_BT2020_PQ: OH_NativeBuffer_ColorSpace =
        OH_NativeBuffer_ColorSpace(31);
}
#[repr(transparent)]
/** @brief Indicates the color space of a native buffer.

@syscap SystemCapability.Graphic.Graphic2D.NativeBuffer
@since 11
@version 1.0*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct OH_NativeBuffer_ColorSpace(pub ::core::ffi::c_uint);

extern "C" {
    /** @brief Set the color space of the OH_NativeBuffer.

    @syscap SystemCapability.Graphic.Graphic2D.NativeBuffer
    @param buffer Indicates the pointer to a <b>OH_NativeBuffer</b> instance.
    @param colorSpace Indicates the color space of native buffer, see <b>OH_NativeBuffer_ColorSpace</b>.
    @return Returns an error code, 0 is success, otherwise, failed.
    @since 11
    @version 1.0*/
    pub fn OH_NativeBuffer_SetColorSpace(
        buffer: *mut OH_NativeBuffer,
        colorSpace: OH_NativeBuffer_ColorSpace,
    ) -> i32;
}
