//! Declares APIs for image encoding
//!
//! <https://docs.openharmony.cn/pages/v5.0/en/application-dev/reference/apis-image-kit/image__packer__native_8h.md>

#[link(name = "image_packer")]
extern "C" {}

mod image_packer_api12;
pub use image_packer_api12::*;

#[cfg(feature = "image-source")]
#[cfg_attr(docsrs, doc(cfg(feature = "image-source")))]
extern "C" {
    /** @brief Encoding an <b>ImageSource</b> into the data with required format.

    @param imagePacker The imagePacker to use for packing.
    @param options Indicates the encoding {@link OH_PackingOptions}.
    @param imageSource The imageSource to be packed.
    @param outData The output data buffer to store the packed image.
    @param size A pointer to the size of the output data buffer.
    @return Returns {@link Image_ErrorCode}
    @since 12*/
    pub fn OH_ImagePackerNative_PackToDataFromImageSource(
        imagePacker: *mut OH_ImagePackerNative,
        options: *mut OH_PackingOptions,
        imageSource: *mut crate::native_image::image_source::OH_ImageSourceNative,
        outData: *mut u8,
        size: *mut usize,
    ) -> crate::native_image::common::Image_ErrorCode;

    /** @brief Encoding an <b>ImageSource</b> into the a file with fd with required format.

    @param imagePacker The image packer to use for packing.
    @param options Indicates the encoding {@link OH_PackingOptions}.
    @param imageSource The imageSource to be packed.
    @param fd Indicates a writable file descriptor.
    @return Returns {@link Image_ErrorCode}
    @since 12*/
    pub fn OH_ImagePackerNative_PackToFileFromImageSource(
        imagePacker: *mut OH_ImagePackerNative,
        options: *mut OH_PackingOptions,
        imageSource: *mut crate::native_image::image_source::OH_ImageSourceNative,
        fd: i32,
    ) -> crate::native_image::common::Image_ErrorCode;

}

#[cfg(feature = "pixelmap")]
#[cfg_attr(docsrs, doc(cfg(feature = "pixelmap")))]
extern "C" {
    /** @brief Encoding a <b>Pixelmap</b> into the data with required format.

    @param imagePacker The imagePacker to use for packing.
    @param options Indicates the encoding {@link OH_PackingOptions}.
    @param pixelmap The pixelmap to be packed.
    @param outData The output data buffer to store the packed image.
    @param size A pointer to the size of the output data buffer.
    @return Returns {@link Image_ErrorCode}
    @since 12*/
    pub fn OH_ImagePackerNative_PackToDataFromPixelmap(
        imagePacker: *mut OH_ImagePackerNative,
        options: *mut OH_PackingOptions,
        pixelmap: *mut crate::native_image::pixelmap::OH_PixelmapNative,
        outData: *mut u8,
        size: *mut usize,
    ) -> crate::native_image::common::Image_ErrorCode;
    /** @brief Encoding a <b>Pixelmap</b> into the a file with fd with required format

    @param imagePacker The image packer to use for packing.
    @param options Indicates the encoding {@link OH_PackingOptions}.
    @param pixelmap The pixelmap to be packed.
    @param fd Indicates a writable file descriptor.
    @return Returns {@link Image_ErrorCode}
    @since 12*/
    pub fn OH_ImagePackerNative_PackToFileFromPixelmap(
        imagePacker: *mut OH_ImagePackerNative,
        options: *mut OH_PackingOptions,
        pixelmap: *mut crate::native_image::pixelmap::OH_PixelmapNative,
        fd: i32,
    ) -> crate::native_image::common::Image_ErrorCode;
}
