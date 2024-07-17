/* automatically generated by rust-bindgen 0.69.4 */

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[repr(C)]
pub struct OH_NativeBuffer {
    _unused: [u8; 0],
}
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
/** @brief Indicates the usage of a native buffer.

@syscap SystemCapability.Graphic.Graphic2D.NativeBuffer
@since 10
@version 1.0*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct OH_NativeBuffer_Usage(pub ::core::ffi::c_uint);
pub mod OH_NativeBuffer_Format {
    /** @brief Indicates the format of a native buffer.

    @syscap SystemCapability.Graphic.Graphic2D.NativeBuffer
    @since 10
    @version 1.0*/
    pub type Type = ::core::ffi::c_uint;
    pub const NATIVEBUFFER_PIXEL_FMT_RGB_565: Type = 3;
    /// < RGB565 format */
    pub const NATIVEBUFFER_PIXEL_FMT_RGBA_5658: Type = 4;
    /// < RGBA5658 format */
    pub const NATIVEBUFFER_PIXEL_FMT_RGBX_4444: Type = 5;
    /// < RGBX4444 format */
    pub const NATIVEBUFFER_PIXEL_FMT_RGBA_4444: Type = 6;
    /// < RGBA4444 format */
    pub const NATIVEBUFFER_PIXEL_FMT_RGB_444: Type = 7;
    /// < RGB444 format */
    pub const NATIVEBUFFER_PIXEL_FMT_RGBX_5551: Type = 8;
    /// < RGBX5551 format */
    pub const NATIVEBUFFER_PIXEL_FMT_RGBA_5551: Type = 9;
    /// < RGBA5551 format */
    pub const NATIVEBUFFER_PIXEL_FMT_RGB_555: Type = 10;
    /// < RGB555 format */
    pub const NATIVEBUFFER_PIXEL_FMT_RGBX_8888: Type = 11;
    /// < RGBX8888 format */
    pub const NATIVEBUFFER_PIXEL_FMT_RGBA_8888: Type = 12;
    /// < RGBA8888 format */
    pub const NATIVEBUFFER_PIXEL_FMT_RGB_888: Type = 13;
    /// < RGB888 format */
    pub const NATIVEBUFFER_PIXEL_FMT_BGR_565: Type = 14;
    /// < BGR565 format */
    pub const NATIVEBUFFER_PIXEL_FMT_BGRX_4444: Type = 15;
    /// < BGRX4444 format */
    pub const NATIVEBUFFER_PIXEL_FMT_BGRA_4444: Type = 16;
    /// < BGRA4444 format */
    pub const NATIVEBUFFER_PIXEL_FMT_BGRX_5551: Type = 17;
    /// < BGRX5551 format */
    pub const NATIVEBUFFER_PIXEL_FMT_BGRA_5551: Type = 18;
    /// < BGRA5551 format */
    pub const NATIVEBUFFER_PIXEL_FMT_BGRX_8888: Type = 19;
    /// < BGRX8888 format */
    pub const NATIVEBUFFER_PIXEL_FMT_BGRA_8888: Type = 20;
    /// < BGRA8888 format */
    pub const NATIVEBUFFER_PIXEL_FMT_BUTT: Type = 2147483647;
}
pub mod OH_NativeBuffer_ColorSpace {
    /** @brief Indicates the color space of a native buffer.

    @syscap SystemCapability.Graphic.Graphic2D.NativeBuffer
    @since 11
    @version 1.0*/
    pub type Type = ::core::ffi::c_uint;
    /// None color space
    pub const OH_COLORSPACE_NONE: Type = 0;
    /// COLORPRIMARIES_BT601_P | (TRANSFUNC_BT709 << 8) | (MATRIX_BT601_P << 16) | (RANGE_FULL << 21)
    pub const OH_COLORSPACE_BT601_EBU_FULL: Type = 1;
    /// COLORPRIMARIES_BT601_N | (TRANSFUNC_BT709 << 8) | (MATRIX_BT601_N << 16) | (RANGE_FULL << 21)
    pub const OH_COLORSPACE_BT601_SMPTE_C_FULL: Type = 2;
    /// COLORPRIMARIES_BT709 | (TRANSFUNC_BT709 << 8) | (MATRIX_BT709 << 16) | (RANGE_FULL << 21)
    pub const OH_COLORSPACE_BT709_FULL: Type = 3;
    /// COLORPRIMARIES_BT2020 | (TRANSFUNC_HLG << 8) | (MATRIX_BT2020 << 16) | (RANGE_FULL << 21)
    pub const OH_COLORSPACE_BT2020_HLG_FULL: Type = 4;
    /// COLORPRIMARIES_BT2020 | (TRANSFUNC_PQ << 8) | (MATRIX_BT2020 << 16) | (RANGE_FULL << 21)
    pub const OH_COLORSPACE_BT2020_PQ_FULL: Type = 5;
    /// COLORPRIMARIES_BT601_P | (TRANSFUNC_BT709 << 8) | (MATRIX_BT601_P << 16) | (RANGE_LIMITED << 21)
    pub const OH_COLORSPACE_BT601_EBU_LIMIT: Type = 6;
    /// COLORPRIMARIES_BT601_N | (TRANSFUNC_BT709 << 8) | (MATRIX_BT601_N << 16) | (RANGE_LIMITED << 21)
    pub const OH_COLORSPACE_BT601_SMPTE_C_LIMIT: Type = 7;
    /// COLORPRIMARIES_BT709 | (TRANSFUNC_BT709 << 8) | (MATRIX_BT709 << 16) | (RANGE_LIMITED << 21)
    pub const OH_COLORSPACE_BT709_LIMIT: Type = 8;
    /// COLORPRIMARIES_BT2020 | (TRANSFUNC_HLG << 8) | (MATRIX_BT2020 << 16) | (RANGE_LIMITED << 21)
    pub const OH_COLORSPACE_BT2020_HLG_LIMIT: Type = 9;
    /// COLORPRIMARIES_BT2020 | (TRANSFUNC_PQ << 8) | (MATRIX_BT2020 << 16) | (RANGE_LIMITED << 21)
    pub const OH_COLORSPACE_BT2020_PQ_LIMIT: Type = 10;
    /// COLORPRIMARIES_SRGB | (TRANSFUNC_SRGB << 8) | (MATRIX_BT601_N << 16) | (RANGE_FULL << 21)
    pub const OH_COLORSPACE_SRGB_FULL: Type = 11;
    /// COLORPRIMARIES_P3_D65 | (TRANSFUNC_SRGB << 8) | (MATRIX_P3 << 16) | (RANGE_FULL << 21)
    pub const OH_COLORSPACE_P3_FULL: Type = 12;
    /// COLORPRIMARIES_P3_D65 | (TRANSFUNC_HLG << 8) | (MATRIX_P3 << 16) | (RANGE_FULL << 21)
    pub const OH_COLORSPACE_P3_HLG_FULL: Type = 13;
    /// COLORPRIMARIES_P3_D65 | (TRANSFUNC_PQ << 8) | (MATRIX_P3 << 16) | (RANGE_FULL << 21)
    pub const OH_COLORSPACE_P3_PQ_FULL: Type = 14;
    /// COLORPRIMARIES_ADOBERGB | (TRANSFUNC_ADOBERGB << 8) | (MATRIX_ADOBERGB << 16) | (RANGE_FULL << 21)
    pub const OH_COLORSPACE_ADOBERGB_FULL: Type = 15;
    /// COLORPRIMARIES_SRGB | (TRANSFUNC_SRGB << 8) | (MATRIX_BT601_N << 16) | (RANGE_LIMITED << 21)
    pub const OH_COLORSPACE_SRGB_LIMIT: Type = 16;
    /// COLORPRIMARIES_P3_D65 | (TRANSFUNC_SRGB << 8) | (MATRIX_P3 << 16) | (RANGE_LIMITED << 21)
    pub const OH_COLORSPACE_P3_LIMIT: Type = 17;
    /// COLORPRIMARIES_P3_D65 | (TRANSFUNC_HLG << 8) | (MATRIX_P3 << 16) | (RANGE_LIMITED << 21)
    pub const OH_COLORSPACE_P3_HLG_LIMIT: Type = 18;
    /// COLORPRIMARIES_P3_D65 | (TRANSFUNC_PQ << 8) | (MATRIX_P3 << 16) | (RANGE_LIMITED << 21)
    pub const OH_COLORSPACE_P3_PQ_LIMIT: Type = 19;
    /// COLORPRIMARIES_ADOBERGB | (TRANSFUNC_ADOBERGB << 8) | (MATRIX_ADOBERGB << 16) | (RANGE_LIMITED << 21)
    pub const OH_COLORSPACE_ADOBERGB_LIMIT: Type = 20;
    /// COLORPRIMARIES_SRGB | (TRANSFUNC_LINEAR << 8)
    pub const OH_COLORSPACE_LINEAR_SRGB: Type = 21;
    /// equal to OH_COLORSPACE_LINEAR_SRGB
    pub const OH_COLORSPACE_LINEAR_BT709: Type = 22;
    /// COLORPRIMARIES_P3_D65 | (TRANSFUNC_LINEAR << 8)
    pub const OH_COLORSPACE_LINEAR_P3: Type = 23;
    /// COLORPRIMARIES_BT2020 | (TRANSFUNC_LINEAR << 8)
    pub const OH_COLORSPACE_LINEAR_BT2020: Type = 24;
    /// equal to OH_COLORSPACE_SRGB_FULL
    pub const OH_COLORSPACE_DISPLAY_SRGB: Type = 25;
    /// equal to OH_COLORSPACE_P3_FULL
    pub const OH_COLORSPACE_DISPLAY_P3_SRGB: Type = 26;
    /// equal to OH_COLORSPACE_P3_HLG_FULL
    pub const OH_COLORSPACE_DISPLAY_P3_HLG: Type = 27;
    /// equal to OH_COLORSPACE_P3_PQ_FULL
    pub const OH_COLORSPACE_DISPLAY_P3_PQ: Type = 28;
    /// COLORPRIMARIES_BT2020 | (TRANSFUNC_SRGB << 8) | (MATRIX_BT2020 << 16) | (RANGE_FULL << 21)
    pub const OH_COLORSPACE_DISPLAY_BT2020_SRGB: Type = 29;
    /// equal to OH_COLORSPACE_BT2020_HLG_FULL
    pub const OH_COLORSPACE_DISPLAY_BT2020_HLG: Type = 30;
    /// equal to OH_COLORSPACE_BT2020_PQ_FULL
    pub const OH_COLORSPACE_DISPLAY_BT2020_PQ: Type = 31;
}
/** @brief <b>OH_NativeBuffer</b> config. \n
Used to allocating new <b>OH_NativeBuffer</b> andquery parameters if existing ones.

@syscap SystemCapability.Graphic.Graphic2D.NativeBuffer
@since 9
@version 1.0*/
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OH_NativeBuffer_Config {
    ///< Width in pixels
    pub width: i32,
    ///< Height in pixels
    pub height: i32,
    ///< One of PixelFormat
    pub format: i32,
    ///< Combination of buffer usage
    pub usage: i32,
    ///< the stride of memory
    pub stride: i32,
}
extern "C" {
    /** @brief Alloc a <b>OH_NativeBuffer</b> that matches the passed BufferRequestConfig. \n
    A new <b>OH_NativeBuffer</b> instance is created each time this function is called.

    @syscap SystemCapability.Graphic.Graphic2D.NativeBuffer
    @param config Indicates the pointer to a <b>BufferRequestConfig</b> instance.
    @return Returns the pointer to the <b>OH_NativeBuffer</b> instance created if the operation is successful, \n
    returns <b>NULL</b> otherwise.
    @since 9
    @version 1.0*/
    pub fn OH_NativeBuffer_Alloc(config: *const OH_NativeBuffer_Config) -> *mut OH_NativeBuffer;
}
extern "C" {
    /** @brief Adds the reference count of a OH_NativeBuffer.

    @syscap SystemCapability.Graphic.Graphic2D.NativeBuffer
    @param buffer Indicates the pointer to a <b>OH_NativeBuffer</b> instance.
    @return Returns an error code, 0 is success, otherwise, failed.
    @since 9
    @version 1.0*/
    pub fn OH_NativeBuffer_Reference(buffer: *mut OH_NativeBuffer) -> i32;
}
extern "C" {
    /** @brief Decreases the reference count of a OH_NativeBuffer and, when the reference count reaches 0, \n
    destroys this OH_NativeBuffer.

    @syscap SystemCapability.Graphic.Graphic2D.NativeBuffer
    @param buffer Indicates the pointer to a <b>OH_NativeBuffer</b> instance.
    @return Returns an error code, 0 is success, otherwise, failed.
    @since 9
    @version 1.0*/
    pub fn OH_NativeBuffer_Unreference(buffer: *mut OH_NativeBuffer) -> i32;
}
extern "C" {
    /** @brief Return a config of the OH_NativeBuffer in the passed OHNativeBufferConfig struct.

    @syscap SystemCapability.Graphic.Graphic2D.NativeBuffer
    @param buffer Indicates the pointer to a <b>OH_NativeBuffer</b> instance.
    @param config Indicates the pointer to the <b>NativeBufferConfig</b> of the buffer.
    @return <b>void</b>
    @since 9
    @version 1.0*/
    pub fn OH_NativeBuffer_GetConfig(
        buffer: *mut OH_NativeBuffer,
        config: *mut OH_NativeBuffer_Config,
    );
}
extern "C" {
    /** @brief Provide direct cpu access to the OH_NativeBuffer in the process's address space.

    @syscap SystemCapability.Graphic.Graphic2D.NativeBuffer
    @param buffer Indicates the pointer to a <b>OH_NativeBuffer</b> instance.
    @param virAddr Indicates the address of the <b>OH_NativeBuffer</b> in virtual memory.
    @return Returns an error code, 0 is success, otherwise, failed.
    @since 9
    @version 1.0*/
    pub fn OH_NativeBuffer_Map(
        buffer: *mut OH_NativeBuffer,
        virAddr: *mut *mut ::core::ffi::c_void,
    ) -> i32;
}
extern "C" {
    /** @brief Remove direct cpu access ability of the OH_NativeBuffer in the process's address space.

    @syscap SystemCapability.Graphic.Graphic2D.NativeBuffer
    @param buffer Indicates the pointer to a <b>OH_NativeBuffer</b> instance.
    @return Returns an error code, 0 is success, otherwise, failed.
    @since 9
    @version 1.0*/
    pub fn OH_NativeBuffer_Unmap(buffer: *mut OH_NativeBuffer) -> i32;
}
extern "C" {
    /** @brief Get the systen wide unique sequence number of the OH_NativeBuffer.

    @syscap SystemCapability.Graphic.Graphic2D.NativeBuffer
    @param buffer Indicates the pointer to a <b>OH_NativeBuffer</b> instance.
    @return Returns the sequence number, which is unique for each OH_NativeBuffer.
    @since 9
    @version 1.0*/
    pub fn OH_NativeBuffer_GetSeqNum(buffer: *mut OH_NativeBuffer) -> u32;
}
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
        colorSpace: OH_NativeBuffer_ColorSpace::Type,
    ) -> i32;
}
