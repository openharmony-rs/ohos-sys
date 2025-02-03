// automatically generated by rust-bindgen 0.71.1

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use crate::native_image::common::*;
#[cfg(feature = "api-13")]
use crate::native_image::picture::OH_PictureNative;
#[cfg(feature = "api-12")]
use ohos_sys_opaque_types::OH_ImageSourceNative;
use ohos_sys_opaque_types::OH_PixelmapNative;

/// Define a ImagePacker struct type, used for ImagePacker pointer controls.
///
///
/// Available since API-level: 12
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
#[repr(C)]
pub struct OH_ImagePackerNative {
    _unused: [u8; 0],
}
/// Defines the image packing options.
///
///
/// Available since API-level: 12
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
#[repr(C)]
pub struct OH_PackingOptions {
    _unused: [u8; 0],
}
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
impl IMAGE_PACKER_DYNAMIC_RANGE {
    pub const IMAGE_PACKER_DYNAMIC_RANGE_AUTO: IMAGE_PACKER_DYNAMIC_RANGE =
        IMAGE_PACKER_DYNAMIC_RANGE(0);
}
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
impl IMAGE_PACKER_DYNAMIC_RANGE {
    pub const IMAGE_PACKER_DYNAMIC_RANGE_SDR: IMAGE_PACKER_DYNAMIC_RANGE =
        IMAGE_PACKER_DYNAMIC_RANGE(1);
}
#[repr(transparent)]
/// Enumerates packing dynamic range.
///
///
/// Available since API-level: 12
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct IMAGE_PACKER_DYNAMIC_RANGE(pub ::core::ffi::c_uint);
extern "C" {
    /// Create a pointer for PackingOptions struct.
    ///
    /// # Arguments
    ///
    /// * `options` - The PackingOptions pointer will be operated.
    ///
    /// # Returns
    ///
    /// * Returns [`Image_ErrorCode`]
    ///
    /// Available since API-level: 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_PackingOptions_Create(options: *mut *mut OH_PackingOptions) -> ImageResult;
    /// Get mime type for OH_PackingOptions struct.
    ///
    /// # Arguments
    ///
    /// * `options` - The OH_PackingOptions pointer will be operated.
    ///
    /// * `format` - the number of image format.The user can pass in a null pointer and zero size, we will allocate memory,
    /// but user must free memory after use.
    ///
    /// # Returns
    ///
    /// * Returns [`Image_ErrorCode`]
    ///
    /// Available since API-level: 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_PackingOptions_GetMimeType(
        options: *mut OH_PackingOptions,
        format: *mut Image_MimeType,
    ) -> ImageResult;
    /// Set format number for OH_PackingOptions struct.
    ///
    /// # Arguments
    ///
    /// * `options` - The OH_PackingOptions pointer will be operated.
    ///
    /// * `format` - the number of image format.
    ///
    /// # Returns
    ///
    /// * Returns [`Image_ErrorCode`]
    ///
    /// Available since API-level: 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_PackingOptions_SetMimeType(
        options: *mut OH_PackingOptions,
        format: *mut Image_MimeType,
    ) -> ImageResult;
    /// Get quality for OH_PackingOptions struct.
    ///
    /// # Arguments
    ///
    /// * `options` - The OH_PackingOptions pointer will be operated.
    ///
    /// * `quality` - The number of image quality.
    ///
    /// # Returns
    ///
    /// * Returns [`Image_ErrorCode`]
    ///
    /// Available since API-level: 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_PackingOptions_GetQuality(
        options: *mut OH_PackingOptions,
        quality: *mut u32,
    ) -> ImageResult;
    /// Set quality number for OH_PackingOptions struct.
    ///
    /// # Arguments
    ///
    /// * `options` - The OH_PackingOptions pointer will be operated.
    ///
    /// * `quality` - The number of image quality.
    ///
    /// # Returns
    ///
    /// * Returns [`Image_ErrorCode`]
    ///
    /// Available since API-level: 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_PackingOptions_SetQuality(
        options: *mut OH_PackingOptions,
        quality: u32,
    ) -> ImageResult;
    /// Get needsPackProperties for OH_PackingOptions struct.
    ///
    /// # Arguments
    ///
    /// * `options` - The OH_PackingOptions pointer will be operated.
    ///
    /// * `needsPackProperties` - Whether the image properties can be saved, like Exif.
    ///
    /// # Returns
    ///
    /// * Returns [`Image_ErrorCode`]
    ///
    /// Available since API-level: 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_PackingOptions_GetNeedsPackProperties(
        options: *mut OH_PackingOptions,
        needsPackProperties: *mut bool,
    ) -> ImageResult;
    /// Set needsPackProperties for OH_PackingOptions struct.
    ///
    /// # Arguments
    ///
    /// * `options` - The OH_PackingOptions pointer will be operated.
    ///
    /// * `needsPackProperties` - Whether the image properties can be saved, like Exif.
    ///
    /// # Returns
    ///
    /// * Returns [`Image_ErrorCode`]
    ///
    /// Available since API-level: 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_PackingOptions_SetNeedsPackProperties(
        options: *mut OH_PackingOptions,
        needsPackProperties: bool,
    ) -> ImageResult;
    /// Get desiredDynamicRange for PackingOptions struct.
    ///
    /// # Arguments
    ///
    /// * `options` - The PackingOptions pointer will be operated. Pointer connot be null.
    ///
    /// * `desiredDynamicRange` - The number of dynamic range [`IMAGE_PACKER_DYNAMIC_RANGE`]. Pointer connot be null.
    ///
    /// # Returns
    ///
    /// * Returns [`Image_ErrorCode`] IMAGE_SUCCESS - The operation is successful.
    /// returns [`Image_ErrorCode`] IMAGE_BAD_PARAMETER - Parameter error.Possible causes:Parameter verification failed.
    ///
    /// Available since API-level: 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_PackingOptions_GetDesiredDynamicRange(
        options: *mut OH_PackingOptions,
        desiredDynamicRange: *mut i32,
    ) -> ImageResult;
    /// Set desiredDynamicRange number for PackingOptions struct.
    ///
    /// # Arguments
    ///
    /// * `options` - The PackingOptions pointer will be operated. Pointer connot be null.
    ///
    /// * `desiredDynamicRange` - The number of dynamic range [`IMAGE_PACKER_DYNAMIC_RANGE`].
    ///
    /// # Returns
    ///
    /// * Returns [`Image_ErrorCode`] IMAGE_SUCCESS - The operation is successful.
    /// returns [`Image_ErrorCode`] IMAGE_BAD_PARAMETER - Parameter error.Possible causes:Parameter verification failed.
    ///
    /// Available since API-level: 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_PackingOptions_SetDesiredDynamicRange(
        options: *mut OH_PackingOptions,
        desiredDynamicRange: i32,
    ) -> ImageResult;
    /// delete OH_PackingOptions pointer.
    ///
    /// # Arguments
    ///
    /// * `options` - The OH_PackingOptions pointer will be operated.
    ///
    /// # Returns
    ///
    /// * Returns [`Image_ErrorCode`]
    ///
    /// Available since API-level: 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_PackingOptions_Release(options: *mut OH_PackingOptions) -> ImageResult;
    /// Create a pointer for OH_ImagePackerNative struct.
    ///
    /// # Arguments
    ///
    /// * `options` - The OH_ImagePackerNative pointer will be operated.
    ///
    /// # Returns
    ///
    /// * Returns [`Image_ErrorCode`]
    ///
    /// Available since API-level: 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_ImagePackerNative_Create(imagePacker: *mut *mut OH_ImagePackerNative) -> ImageResult;
    /// Encoding an <b>ImageSource</b> into the data with required format.
    ///
    /// # Arguments
    ///
    /// * `imagePacker` - The imagePacker to use for packing.
    ///
    /// * `options` - Indicates the encoding [`OH_PackingOptions`].
    ///
    /// * `imageSource` - The imageSource to be packed.
    ///
    /// * `outData` - The output data buffer to store the packed image.
    ///
    /// * `size` - A pointer to the size of the output data buffer.
    ///
    /// # Returns
    ///
    /// * Returns [`Image_ErrorCode`]
    ///
    /// Available since API-level: 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_ImagePackerNative_PackToDataFromImageSource(
        imagePacker: *mut OH_ImagePackerNative,
        options: *mut OH_PackingOptions,
        imageSource: *mut OH_ImageSourceNative,
        outData: *mut u8,
        size: *mut usize,
    ) -> ImageResult;
    /// Encoding a <b>Pixelmap</b> into the data with required format.
    ///
    /// # Arguments
    ///
    /// * `imagePacker` - The imagePacker to use for packing.
    ///
    /// * `options` - Indicates the encoding [`OH_PackingOptions`].
    ///
    /// * `pixelmap` - The pixelmap to be packed.
    ///
    /// * `outData` - The output data buffer to store the packed image.
    ///
    /// * `size` - A pointer to the size of the output data buffer.
    ///
    /// # Returns
    ///
    /// * Returns [`Image_ErrorCode`]
    ///
    /// Available since API-level: 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_ImagePackerNative_PackToDataFromPixelmap(
        imagePacker: *mut OH_ImagePackerNative,
        options: *mut OH_PackingOptions,
        pixelmap: *mut OH_PixelmapNative,
        outData: *mut u8,
        size: *mut usize,
    ) -> ImageResult;
    /// Encoding a <b>Picture</b> into the data with required format.
    ///
    /// # Arguments
    ///
    /// * `imagePacker` - The imagePacker to use for packing.
    ///
    /// * `options` - Indicates the encoding [`OH_PackingOptions`].
    ///
    /// * `picture` - The picture to be packed.
    ///
    /// * `outData` - The output data buffer to store the packed image.
    ///
    /// * `size` - A pointer to the size of the output data buffer.
    ///
    /// # Returns
    ///
    /// * Image functions result code.
    /// [`IMAGE_SUCCESS`] if the execution is successful.
    /// [`IMAGE_BAD_PARAMETER`] imagePacker is nullptr, or picture is nullptr, or outData is nullptr,
    /// or size is invalid.
    /// [`IMAGE_ENCODE_FAILED`] encode failed.
    ///
    /// Available since API-level: 13
    #[cfg(feature = "api-13")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-13")))]
    pub fn OH_ImagePackerNative_PackToDataFromPicture(
        imagePacker: *mut OH_ImagePackerNative,
        options: *mut OH_PackingOptions,
        picture: *mut OH_PictureNative,
        outData: *mut u8,
        size: *mut usize,
    ) -> ImageResult;
    /// Encoding an <b>ImageSource</b> into the a file with fd with required format.
    ///
    /// # Arguments
    ///
    /// * `imagePacker` - The image packer to use for packing.
    ///
    /// * `options` - Indicates the encoding [`OH_PackingOptions`].
    ///
    /// * `imageSource` - The imageSource to be packed.
    ///
    /// * `fd` - Indicates a writable file descriptor.
    ///
    /// # Returns
    ///
    /// * Returns [`Image_ErrorCode`]
    ///
    /// Available since API-level: 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_ImagePackerNative_PackToFileFromImageSource(
        imagePacker: *mut OH_ImagePackerNative,
        options: *mut OH_PackingOptions,
        imageSource: *mut OH_ImageSourceNative,
        fd: i32,
    ) -> ImageResult;
    /// Encoding a <b>Pixelmap</b> into the a file with fd with required format
    ///
    /// # Arguments
    ///
    /// * `imagePacker` - The image packer to use for packing.
    ///
    /// * `options` - Indicates the encoding [`OH_PackingOptions`].
    ///
    /// * `pixelmap` - The pixelmap to be packed.
    ///
    /// * `fd` - Indicates a writable file descriptor.
    ///
    /// # Returns
    ///
    /// * Returns [`Image_ErrorCode`]
    ///
    /// Available since API-level: 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_ImagePackerNative_PackToFileFromPixelmap(
        imagePacker: *mut OH_ImagePackerNative,
        options: *mut OH_PackingOptions,
        pixelmap: *mut OH_PixelmapNative,
        fd: i32,
    ) -> ImageResult;
    /// Encoding a <b>Picture</b> into the a file with fd with required format.
    ///
    /// # Arguments
    ///
    /// * `imagePacker` - The imagePacker to use for packing.
    ///
    /// * `options` - Indicates the encoding [`OH_PackingOptions`].
    ///
    /// * `picture` - The picture to be packed.
    ///
    /// * `fd` - Indicates a writable file descriptor.
    ///
    /// # Returns
    ///
    /// * Image functions result code.
    /// [`IMAGE_SUCCESS`] if the execution is successful.
    /// [`IMAGE_BAD_PARAMETER`] imagePacker is nullptr, or picture is nullptr, or fd is invalid.
    /// [`IMAGE_ENCODE_FAILED`] encode failed.
    ///
    /// Available since API-level: 13
    #[cfg(feature = "api-13")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-13")))]
    pub fn OH_ImagePackerNative_PackToFileFromPicture(
        imagePacker: *mut OH_ImagePackerNative,
        options: *mut OH_PackingOptions,
        picture: *mut OH_PictureNative,
        fd: i32,
    ) -> ImageResult;
    /// Releases an imagePacker object.
    ///
    /// # Arguments
    ///
    /// * `imagePacker` - A pointer to the image packer object to be released.
    ///
    /// # Returns
    ///
    /// * Returns [`Image_ErrorCode`]
    ///
    /// Available since API-level: 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_ImagePackerNative_Release(imagePacker: *mut OH_ImagePackerNative) -> ImageResult;
}
