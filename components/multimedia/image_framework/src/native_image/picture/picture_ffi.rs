// automatically generated by rust-bindgen 0.71.1

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use crate::native_image::common::*;
use crate::native_image::pixelmap::PIXEL_FORMAT;
use ohos_sys_opaque_types::OH_PixelmapNative;

/// Define a Picture struct type, used for picture pointer controls.
///
///
/// Available since API-level: 13
#[cfg(feature = "api-13")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-13")))]
#[repr(C)]
pub struct OH_PictureNative {
    _unused: [u8; 0],
}
/// Define a AuxiliaryPicture struct type, used for auxiliary
/// picture pointer controls.
///
///
/// Available since API-level: 13
#[cfg(feature = "api-13")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-13")))]
#[repr(C)]
pub struct OH_AuxiliaryPictureNative {
    _unused: [u8; 0],
}
/// Define a AuxiliaryPictureInfo struct type, used for auxiliary
/// picture info controls.
///
///
/// Available since API-level: 13
#[cfg(feature = "api-13")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-13")))]
#[repr(C)]
pub struct OH_AuxiliaryPictureInfo {
    _unused: [u8; 0],
}
#[cfg(feature = "api-13")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-13")))]
impl Image_AuxiliaryPictureType {
    pub const AUXILIARY_PICTURE_TYPE_GAINMAP: Image_AuxiliaryPictureType =
        Image_AuxiliaryPictureType(1);
}
#[cfg(feature = "api-13")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-13")))]
impl Image_AuxiliaryPictureType {
    pub const AUXILIARY_PICTURE_TYPE_DEPTH_MAP: Image_AuxiliaryPictureType =
        Image_AuxiliaryPictureType(2);
}
#[cfg(feature = "api-13")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-13")))]
impl Image_AuxiliaryPictureType {
    pub const AUXILIARY_PICTURE_TYPE_UNREFOCUS_MAP: Image_AuxiliaryPictureType =
        Image_AuxiliaryPictureType(3);
}
#[cfg(feature = "api-13")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-13")))]
impl Image_AuxiliaryPictureType {
    pub const AUXILIARY_PICTURE_TYPE_LINEAR_MAP: Image_AuxiliaryPictureType =
        Image_AuxiliaryPictureType(4);
}
#[cfg(feature = "api-13")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-13")))]
impl Image_AuxiliaryPictureType {
    pub const AUXILIARY_PICTURE_TYPE_FRAGMENT_MAP: Image_AuxiliaryPictureType =
        Image_AuxiliaryPictureType(5);
}
#[repr(transparent)]
/// Define a auxiliary picture type.
///
///
/// Available since API-level: 13
#[cfg(feature = "api-13")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-13")))]
#[derive(Clone, Hash, PartialEq, Eq)]
pub struct Image_AuxiliaryPictureType(pub ::core::ffi::c_uint);
extern "C" {
    /// Create a <b>Picture</b> object.
    ///
    /// # Arguments
    ///
    /// * `mainPixelmap` - The pixel map of the main image.
    ///
    /// * `picture` - Picture pointer for created.
    ///
    /// # Returns
    ///
    /// * Image functions result code.
    /// [`IMAGE_SUCCESS`] if the execution is successful.
    /// [`IMAGE_BAD_PARAMETER`] mainPixelmap is nullptr, or picture is nullptr.
    ///
    /// Available since API-level: 13
    #[cfg(feature = "api-13")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-13")))]
    pub fn OH_PictureNative_CreatePicture(
        mainPixelmap: *mut OH_PixelmapNative,
        picture: *mut *mut OH_PictureNative,
    ) -> Image_ErrorCode;
    /// Obtains the pixel map of the main image.
    ///
    /// # Arguments
    ///
    /// * `picture` - The Picture pointer will be operated.
    ///
    /// * `mainPixelmap` - Main pixel map pointer for obtained.
    ///
    /// # Returns
    ///
    /// * Image functions result code.
    /// [`IMAGE_SUCCESS`] if the execution is successful.
    /// [`IMAGE_BAD_PARAMETER`] picture is nullptr, or mainPixelmap is nullptr.
    ///
    /// Available since API-level: 13
    #[cfg(feature = "api-13")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-13")))]
    pub fn OH_PictureNative_GetMainPixelmap(
        picture: *mut OH_PictureNative,
        mainPixelmap: *mut *mut OH_PixelmapNative,
    ) -> Image_ErrorCode;
    /// Obtains the hdr pixel map.
    ///
    /// # Arguments
    ///
    /// * `picture` - The Picture pointer will be operated.
    ///
    /// * `hdrPixelmap` - Hdr pixel map pointer for obtained.
    ///
    /// # Returns
    ///
    /// * Image functions result code.
    /// [`IMAGE_SUCCESS`] if the execution is successful.
    /// [`IMAGE_BAD_PARAMETER`] picture is nullptr, or hdrPixelmap is nullptr.
    /// [`IMAGE_UNSUPPORTED_OPERATION`] Unsupported operation, e.g. the picture does not has a gainmap
    ///
    /// Available since API-level: 13
    #[cfg(feature = "api-13")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-13")))]
    pub fn OH_PictureNative_GetHdrComposedPixelmap(
        picture: *mut OH_PictureNative,
        hdrPixelmap: *mut *mut OH_PixelmapNative,
    ) -> Image_ErrorCode;
    /// Obtains the gainmap pixel map.
    ///
    /// # Arguments
    ///
    /// * `picture` - The Picture pointer will be operated.
    ///
    /// * `gainmapPixelmap` - Gainmap pointer for obtained.
    ///
    /// # Returns
    ///
    /// * Image functions result code.
    /// [`IMAGE_SUCCESS`] if the execution is successful.
    /// [`IMAGE_BAD_PARAMETER`] picture is nullptr, or gainmapPixelmap is nullptr.
    ///
    /// Available since API-level: 13
    #[cfg(feature = "api-13")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-13")))]
    pub fn OH_PictureNative_GetGainmapPixelmap(
        picture: *mut OH_PictureNative,
        gainmapPixelmap: *mut *mut OH_PixelmapNative,
    ) -> Image_ErrorCode;
    /// Set auxiliary picture.
    ///
    /// # Arguments
    ///
    /// * `picture` - The Picture pointer will be operated.
    ///
    /// * `type` - The type of auxiliary picture.
    ///
    /// * `auxiliaryPicture` - AuxiliaryPicture object.
    ///
    /// # Returns
    ///
    /// * Image functions result code.
    /// [`IMAGE_SUCCESS`] if the execution is successful.
    /// [`IMAGE_BAD_PARAMETER`] picture is nullptr, or auxiliaryPicture is nullptr, or the type is invalid.
    ///
    /// Available since API-level: 13
    #[cfg(feature = "api-13")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-13")))]
    pub fn OH_PictureNative_SetAuxiliaryPicture(
        picture: *mut OH_PictureNative,
        type_: Image_AuxiliaryPictureType,
        auxiliaryPicture: *mut OH_AuxiliaryPictureNative,
    ) -> Image_ErrorCode;
    /// Obtains the auxiliary picture based on type.
    ///
    /// # Arguments
    ///
    /// * `picture` - The Picture pointer will be operated.
    ///
    /// * `type` - The type of auxiliary picture.
    ///
    /// * `auxiliaryPicture` - AuxiliaryPicture pointer for obtained.
    ///
    /// # Returns
    ///
    /// * Image functions result code.
    /// [`IMAGE_SUCCESS`] if the execution is successful.
    /// [`IMAGE_BAD_PARAMETER`] picture is nullptr, or auxiliaryPicture is nullptr, or the type is invalid.
    ///
    /// Available since API-level: 13
    #[cfg(feature = "api-13")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-13")))]
    pub fn OH_PictureNative_GetAuxiliaryPicture(
        picture: *mut OH_PictureNative,
        type_: Image_AuxiliaryPictureType,
        auxiliaryPicture: *mut *mut OH_AuxiliaryPictureNative,
    ) -> Image_ErrorCode;
    /// Obtains the metadata of main picture.
    ///
    /// # Arguments
    ///
    /// * `picture` - The Picture pointer will be operated.
    ///
    /// * `metadataType` - The type of metadata.
    ///
    /// * `metadata` - The metadata of main picture.
    ///
    /// # Returns
    ///
    /// * Image functions result code.
    /// [`IMAGE_SUCCESS`] if the execution is successful.
    /// [`IMAGE_BAD_PARAMETER`] picture is nullptr, or metadata is nullptr.
    /// [`IMAGE_UNSUPPORTED_METADATA`] unsupported metadata type.
    ///
    /// Available since API-level: 13
    #[cfg(feature = "api-13")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-13")))]
    pub fn OH_PictureNative_GetMetadata(
        picture: *mut OH_PictureNative,
        metadataType: Image_MetadataType,
        metadata: *mut *mut OH_PictureMetadata,
    ) -> Image_ErrorCode;
    /// Set main picture metadata.
    ///
    /// # Arguments
    ///
    /// * `picture` - The Picture pointer will be operated.
    ///
    /// * `metadataType` - The type of metadata.
    ///
    /// * `metadata` - The metadata will be set.
    ///
    /// # Returns
    ///
    /// * Image functions result code.
    /// [`IMAGE_SUCCESS`] if the execution is successful.
    /// [`IMAGE_BAD_PARAMETER`] picture is nullptr, or metadata is nullptr.
    /// [`IMAGE_UNSUPPORTED_METADATA`] unsupported metadata type.
    ///
    /// Available since API-level: 13
    #[cfg(feature = "api-13")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-13")))]
    pub fn OH_PictureNative_SetMetadata(
        picture: *mut OH_PictureNative,
        metadataType: Image_MetadataType,
        metadata: *mut OH_PictureMetadata,
    ) -> Image_ErrorCode;
    /// Releases this Picture object.
    ///
    /// # Arguments
    ///
    /// * `picture` - The Picture pointer will be operated.
    ///
    /// # Returns
    ///
    /// * Image functions result code.
    /// [`IMAGE_SUCCESS`] if the execution is successful.
    /// [`IMAGE_BAD_PARAMETER`] picture is nullptr.
    ///
    /// Available since API-level: 13
    #[cfg(feature = "api-13")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-13")))]
    pub fn OH_PictureNative_Release(picture: *mut OH_PictureNative) -> Image_ErrorCode;
    /// Create a <b>AuxiliaryPicture</b> object.
    ///
    /// # Arguments
    ///
    /// * `data` - The image data buffer.
    ///
    /// * `dataLength` - The length of data.
    ///
    /// * `size` - The size of auxiliary picture.
    ///
    /// * `type` - The type of auxiliary picture.
    ///
    /// * `auxiliaryPicture` - AuxiliaryPicture pointer for created.
    ///
    /// # Returns
    ///
    /// * Image functions result code.
    /// [`IMAGE_SUCCESS`] if the execution is successful.
    /// [`IMAGE_BAD_PARAMETER`] data is nullptr, or dataLength is invalid, or size is nullptr, or the type
    /// is invalid, or auxiliaryPicture is nullptr.
    ///
    /// Available since API-level: 13
    #[cfg(feature = "api-13")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-13")))]
    pub fn OH_AuxiliaryPictureNative_Create(
        data: *mut u8,
        dataLength: usize,
        size: *mut Image_Size,
        type_: Image_AuxiliaryPictureType,
        auxiliaryPicture: *mut *mut OH_AuxiliaryPictureNative,
    ) -> Image_ErrorCode;
    /// Write pixels to auxiliary picture.
    ///
    /// # Arguments
    ///
    /// * `auxiliaryPicture` - The AuxiliaryPicture pointer will be operated.
    ///
    /// * `source` - The pixels will be written.
    ///
    /// * `bufferSize` - The size of pixels.
    ///
    /// # Returns
    ///
    /// * Image functions result code.
    /// [`IMAGE_SUCCESS`] if the execution is successful.
    /// [`IMAGE_BAD_PARAMETER`] auxiliaryPicture is nullptr, or source is nullptr, or the bufferSize is invalid.
    /// [`IMAGE_ALLOC_FAILED`] memory alloc failed.
    /// [`IMAGE_COPY_FAILED`] memory copy failed.
    ///
    /// Available since API-level: 13
    #[cfg(feature = "api-13")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-13")))]
    pub fn OH_AuxiliaryPictureNative_WritePixels(
        auxiliaryPicture: *mut OH_AuxiliaryPictureNative,
        source: *mut u8,
        bufferSize: usize,
    ) -> Image_ErrorCode;
    /// Read pixels from auxiliary picture.
    ///
    /// # Arguments
    ///
    /// * `auxiliaryPicture` - The AuxiliaryPicture pointer will be operated.
    ///
    /// * `destination` - The pixels will be read.
    ///
    /// * `bufferSize` - The size of pixels for reading.
    ///
    /// # Returns
    ///
    /// * Image functions result code.
    /// [`IMAGE_SUCCESS`] if the execution is successful.
    /// [`IMAGE_BAD_PARAMETER`] auxiliaryPicture is nullptr, or destination is nullptr,
    /// or the bufferSize is invalid.
    /// [`IMAGE_ALLOC_FAILED`] memory alloc failed.
    /// [`IMAGE_COPY_FAILED`] memory copy failed.
    ///
    /// Available since API-level: 13
    #[cfg(feature = "api-13")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-13")))]
    pub fn OH_AuxiliaryPictureNative_ReadPixels(
        auxiliaryPicture: *mut OH_AuxiliaryPictureNative,
        destination: *mut u8,
        bufferSize: *mut usize,
    ) -> Image_ErrorCode;
    /// Obtains the type of auxiliary picture.
    ///
    /// # Arguments
    ///
    /// * `auxiliaryPicture` - The AuxiliaryPicture pointer will be operated.
    ///
    /// * `type` - The type of auxiliary picture.
    ///
    /// # Returns
    ///
    /// * Image functions result code.
    /// [`IMAGE_SUCCESS`] if the execution is successful.
    /// [`IMAGE_BAD_PARAMETER`] auxiliaryPicture is nullptr, or type is nullptr.
    ///
    /// Available since API-level: 13
    #[cfg(feature = "api-13")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-13")))]
    pub fn OH_AuxiliaryPictureNative_GetType(
        auxiliaryPicture: *mut OH_AuxiliaryPictureNative,
        type_: *mut Image_AuxiliaryPictureType,
    ) -> Image_ErrorCode;
    /// Obtains the info of auxiliary picture.
    ///
    /// # Arguments
    ///
    /// * `auxiliaryPicture` - The AuxiliaryPicture pointer will be operated.
    ///
    /// * `info` - The info of auxiliary picture.
    ///
    /// # Returns
    ///
    /// * Image functions result code.
    /// [`IMAGE_SUCCESS`] if the execution is successful.
    /// [`IMAGE_BAD_PARAMETER`] auxiliaryPicture is nullptr, or info is nullptr.
    ///
    /// Available since API-level: 13
    #[cfg(feature = "api-13")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-13")))]
    pub fn OH_AuxiliaryPictureNative_GetInfo(
        auxiliaryPicture: *mut OH_AuxiliaryPictureNative,
        info: *mut *mut OH_AuxiliaryPictureInfo,
    ) -> Image_ErrorCode;
    /// Set auxiliary picture info.
    ///
    /// # Arguments
    ///
    /// * `auxiliaryPicture` - The AuxiliaryPicture pointer will be operated.
    ///
    /// * `info` - The info will be set.
    ///
    /// # Returns
    ///
    /// * Image functions result code.
    /// [`IMAGE_SUCCESS`] if the execution is successful.
    /// [`IMAGE_BAD_PARAMETER`] auxiliaryPicture is nullptr, or info is nullptr.
    ///
    /// Available since API-level: 13
    #[cfg(feature = "api-13")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-13")))]
    pub fn OH_AuxiliaryPictureNative_SetInfo(
        auxiliaryPicture: *mut OH_AuxiliaryPictureNative,
        info: *mut OH_AuxiliaryPictureInfo,
    ) -> Image_ErrorCode;
    /// Obtains the metadata of auxiliary picture.
    ///
    /// # Arguments
    ///
    /// * `auxiliaryPicture` - The AuxiliaryPicture pointer will be operated.
    ///
    /// * `metadataType` - The type of metadata.
    ///
    /// * `metadata` - The metadata of auxiliary picture.
    ///
    /// # Returns
    ///
    /// * Image functions result code.
    /// [`IMAGE_SUCCESS`] if the execution is successful.
    /// [`IMAGE_BAD_PARAMETER`] auxiliaryPicture is nullptr, or metadata is nullptr.
    /// [`IMAGE_UNSUPPORTED_METADATA`] unsupported metadata type, or the metadata type does not match the
    /// auxiliary picture type.
    ///
    /// Available since API-level: 13
    #[cfg(feature = "api-13")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-13")))]
    pub fn OH_AuxiliaryPictureNative_GetMetadata(
        auxiliaryPicture: *mut OH_AuxiliaryPictureNative,
        metadataType: Image_MetadataType,
        metadata: *mut *mut OH_PictureMetadata,
    ) -> Image_ErrorCode;
    /// Set auxiliary picture metadata.
    ///
    /// # Arguments
    ///
    /// * `auxiliaryPicture` - The AuxiliaryPicture pointer will be operated.
    ///
    /// * `metadataType` - The type of metadata.
    ///
    /// * `metadata` - The metadata will be set.
    ///
    /// # Returns
    ///
    /// * Image functions result code.
    /// [`IMAGE_SUCCESS`] if the execution is successful.
    /// [`IMAGE_BAD_PARAMETER`] auxiliaryPicture is nullptr, or metadata is nullptr.
    /// [`IMAGE_UNSUPPORTED_METADATA`] unsupported metadata type, or the metadata type does not match the
    /// auxiliary picture type.
    ///
    /// Available since API-level: 13
    #[cfg(feature = "api-13")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-13")))]
    pub fn OH_AuxiliaryPictureNative_SetMetadata(
        auxiliaryPicture: *mut OH_AuxiliaryPictureNative,
        metadataType: Image_MetadataType,
        metadata: *mut OH_PictureMetadata,
    ) -> Image_ErrorCode;
    /// Releases this AuxiliaryPicture object.
    ///
    /// # Arguments
    ///
    /// * `picture` - The Picture pointer will be operated.
    ///
    /// # Returns
    ///
    /// * Image functions result code.
    /// [`IMAGE_SUCCESS`] if the execution is successful.
    /// [`IMAGE_BAD_PARAMETER`] picture is nullptr.
    ///
    /// Available since API-level: 13
    #[cfg(feature = "api-13")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-13")))]
    pub fn OH_AuxiliaryPictureNative_Release(
        picture: *mut OH_AuxiliaryPictureNative,
    ) -> Image_ErrorCode;
    /// Create a <b>AuxiliaryPictureInfo</b> object.
    ///
    /// # Arguments
    ///
    /// * `info` - The AuxiliaryPictureInfo pointer will be operated.
    ///
    /// # Returns
    ///
    /// * Image functions result code.
    /// [`IMAGE_SUCCESS`] if the execution is successful.
    /// [`IMAGE_BAD_PARAMETER`] info is nullptr.
    ///
    /// Available since API-level: 13
    #[cfg(feature = "api-13")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-13")))]
    pub fn OH_AuxiliaryPictureInfo_Create(
        info: *mut *mut OH_AuxiliaryPictureInfo,
    ) -> Image_ErrorCode;
    /// Obtains the type of auxiliary picture info.
    ///
    /// # Arguments
    ///
    /// * `info` - The AuxiliaryPictureInfo pointer will be operated.
    ///
    /// * `type` - The type of auxiliary picture info.
    ///
    /// # Returns
    ///
    /// * Image functions result code.
    /// [`IMAGE_SUCCESS`] if the execution is successful.
    /// [`IMAGE_BAD_PARAMETER`] info is nullptr, or type is nullptr.
    ///
    /// Available since API-level: 13
    #[cfg(feature = "api-13")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-13")))]
    pub fn OH_AuxiliaryPictureInfo_GetType(
        info: *mut OH_AuxiliaryPictureInfo,
        type_: *mut Image_AuxiliaryPictureType,
    ) -> Image_ErrorCode;
    /// Set auxiliary picture info type.
    ///
    /// # Arguments
    ///
    /// * `info` - The AuxiliaryPictureInfo pointer will be operated.
    ///
    /// * `type` - The type will be set.
    ///
    /// # Returns
    ///
    /// * Image functions result code.
    /// [`IMAGE_SUCCESS`] if the execution is successful.
    /// [`IMAGE_BAD_PARAMETER`] info is nullptr, or type is invalid.
    ///
    /// Available since API-level: 13
    #[cfg(feature = "api-13")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-13")))]
    pub fn OH_AuxiliaryPictureInfo_SetType(
        info: *mut OH_AuxiliaryPictureInfo,
        type_: Image_AuxiliaryPictureType,
    ) -> Image_ErrorCode;
    /// Obtains the size of auxiliary picture info.
    ///
    /// # Arguments
    ///
    /// * `info` - The AuxiliaryPictureInfo pointer will be operated.
    ///
    /// * `size` - The size of auxiliary picture info.
    ///
    /// # Returns
    ///
    /// * Image functions result code.
    /// [`IMAGE_SUCCESS`] if the execution is successful.
    /// [`IMAGE_BAD_PARAMETER`] info is nullptr, or size is nullptr.
    ///
    /// Available since API-level: 13
    #[cfg(feature = "api-13")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-13")))]
    pub fn OH_AuxiliaryPictureInfo_GetSize(
        info: *mut OH_AuxiliaryPictureInfo,
        size: *mut Image_Size,
    ) -> Image_ErrorCode;
    /// Set auxiliary picture info size.
    ///
    /// # Arguments
    ///
    /// * `info` - The AuxiliaryPictureInfo pointer will be operated.
    ///
    /// * `size` - The size will be set.
    ///
    /// # Returns
    ///
    /// * Image functions result code.
    /// [`IMAGE_SUCCESS`] if the execution is successful.
    /// [`IMAGE_BAD_PARAMETER`] info is nullptr, or size is nullptr.
    ///
    /// Available since API-level: 13
    #[cfg(feature = "api-13")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-13")))]
    pub fn OH_AuxiliaryPictureInfo_SetSize(
        info: *mut OH_AuxiliaryPictureInfo,
        size: *mut Image_Size,
    ) -> Image_ErrorCode;
    /// Obtains the rowStride of auxiliary picture info.
    ///
    /// # Arguments
    ///
    /// * `info` - The AuxiliaryPictureInfo pointer will be operated.
    ///
    /// * `rowStride` - The rowStride of auxiliary picture info.
    ///
    /// # Returns
    ///
    /// * Image functions result code.
    /// [`IMAGE_SUCCESS`] if the execution is successful.
    /// [`IMAGE_BAD_PARAMETER`] info is nullptr, or rowStride is nullptr.
    ///
    /// Available since API-level: 13
    #[cfg(feature = "api-13")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-13")))]
    pub fn OH_AuxiliaryPictureInfo_GetRowStride(
        info: *mut OH_AuxiliaryPictureInfo,
        rowStride: *mut u32,
    ) -> Image_ErrorCode;
    /// Set auxiliary picture info rowStride.
    ///
    /// # Arguments
    ///
    /// * `info` - The AuxiliaryPictureInfo pointer will be operated.
    ///
    /// * `rowStride` - The rowStride will be set.
    ///
    /// # Returns
    ///
    /// * Image functions result code.
    /// [`IMAGE_SUCCESS`] if the execution is successful.
    /// [`IMAGE_BAD_PARAMETER`] info is nullptr, or rowStride is nullptr.
    ///
    /// Available since API-level: 13
    #[cfg(feature = "api-13")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-13")))]
    pub fn OH_AuxiliaryPictureInfo_SetRowStride(
        info: *mut OH_AuxiliaryPictureInfo,
        rowStride: u32,
    ) -> Image_ErrorCode;
    /// Obtains the pixelFormat of auxiliary picture info.
    ///
    /// # Arguments
    ///
    /// * `info` - The AuxiliaryPictureInfo pointer will be operated.
    ///
    /// * `pixelFormat` - The pixelFormat will be get.
    ///
    /// # Returns
    ///
    /// * Image functions result code.
    /// [`IMAGE_SUCCESS`] if the execution is successful.
    /// [`IMAGE_BAD_PARAMETER`] info is nullptr, or pixelFormat is nullptr.
    ///
    /// Available since API-level: 13
    #[cfg(feature = "api-13")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-13")))]
    pub fn OH_AuxiliaryPictureInfo_GetPixelFormat(
        info: *mut OH_AuxiliaryPictureInfo,
        pixelFormat: *mut PIXEL_FORMAT,
    ) -> Image_ErrorCode;
    /// Set auxiliary picture info pixelFormat.
    ///
    /// # Arguments
    ///
    /// * `info` - The AuxiliaryPictureInfo pointer will be operated.
    ///
    /// * `pixelFormat` - The pixelFormat will be set.
    ///
    /// # Returns
    ///
    /// * Image functions result code.
    /// [`IMAGE_SUCCESS`] if the execution is successful.
    /// [`IMAGE_BAD_PARAMETER`] info is nullptr.
    ///
    /// Available since API-level: 13
    #[cfg(feature = "api-13")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-13")))]
    pub fn OH_AuxiliaryPictureInfo_SetPixelFormat(
        info: *mut OH_AuxiliaryPictureInfo,
        pixelFormat: PIXEL_FORMAT,
    ) -> Image_ErrorCode;
    /// Releases this AuxiliaryPictureInfo object.
    ///
    /// # Arguments
    ///
    /// * `info` - The AuxiliaryPictureInfo pointer will be operated.
    ///
    /// # Returns
    ///
    /// * Image functions result code.
    /// [`IMAGE_SUCCESS`] if the execution is successful.
    /// [`IMAGE_BAD_PARAMETER`] info is nullptr.
    ///
    /// Available since API-level: 13
    #[cfg(feature = "api-13")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-13")))]
    pub fn OH_AuxiliaryPictureInfo_Release(info: *mut OH_AuxiliaryPictureInfo) -> Image_ErrorCode;
}
