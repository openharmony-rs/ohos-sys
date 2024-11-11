//! Declares APIs for image decoding
//!
//! <https://docs.openharmony.cn/pages/v5.0/en/application-dev/reference/apis-image-kit/image__source__native_8h.md>

#[link(name = "image_source")]
extern "C" {}

mod image_source_api12;
pub use image_source_api12::*;

#[cfg(feature = "pixelmap")]
#[cfg_attr(docsrs, doc(cfg(feature = "pixelmap")))]
extern "C" {
    /** @brief Decodes an void pointer
    based on the specified {@link OH_DecodingOptions} struct.

    @param source Indicates a void pointer(from ImageSource pointer convert).
    @param  options Indicates a pointer to the options for decoding the image source.
    For details, see {@link OH_DecodingOptions}.
    @param resPixMap Indicates a void pointer to the <b>Pixelmap</b> object obtained at the C++ native layer.
    @return Returns {@link Image_ErrorCode}
    @since 12*/
    pub fn OH_ImageSourceNative_CreatePixelmap(
        source: *mut OH_ImageSourceNative,
        options: *mut OH_DecodingOptions,
        pixelmap: *mut *mut crate::native_image::pixelmap::OH_PixelmapNative,
    ) -> crate::native_image::common::Image_ErrorCode;
    /** @brief Decodes an void pointer
    the <b>Pixelmap</b> objects at the C++ native layer
    based on the specified {@link OH_DecodingOptions} struct.

    @param source Indicates a void pointer(from ImageSource pointer convert).
    @param  options Indicates a pointer to the options for decoding the image source.
    For details, see {@link OH_DecodingOptions}.
    @param resVecPixMap Indicates a pointer array to the <b>Pixelmap</b> objects obtained at the C++ native layer.
    It cannot be a null pointer.
    @param size Indicates a size of resVecPixMap. User can get size from {@link OH_ImageSourceNative_GetFrameCount}.
    @return Returns {@link Image_ErrorCode}
    @since 12*/
    pub fn OH_ImageSourceNative_CreatePixelmapList(
        source: *mut OH_ImageSourceNative,
        options: *mut OH_DecodingOptions,
        resVecPixMap: *mut *mut crate::native_image::common::Image_ErrorCode,
        size: usize,
    ) -> crate::native_image::common::Image_ErrorCode;
}
