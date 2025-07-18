// automatically generated by rust-bindgen 0.71.1

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#[cfg(feature = "api-12")]
use crate::avcodec_base::OH_AVDataSource;
#[allow(unused_imports)]
use crate::averrors::OH_AVErrCode;
use crate::avformat::OH_AVFormat;

/// Forward declaration of OH_AVSource.
///
///
/// Available since API-level: 10
#[repr(C)]
pub struct OH_AVSource {
    _unused: [u8; 0],
}
extern "C" {
    /// Creates an OH_AVSource instance that models the media with dataSource.
    ///
    /// Required System Capabilities: SystemCapability.Multimedia.Media.Spliter
    /// # Arguments
    ///
    /// * `dataSource` - An Struct for a remote media resource.
    ///
    /// # Returns
    ///
    /// * Returns a pointer to an OH_AVSource instance if the execution is successful, otherwise returns nullptr.
    /// Possible failure causes:
    /// 1. dataSource is nullptr.
    /// 2. dataSource->size == 0.
    /// 3. set data source failed.
    /// 4. out of memory.
    /// 5. demuxer engine is nullptr.
    ///
    /// Available since API-level: 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_AVSource_CreateWithDataSource(dataSource: *mut OH_AVDataSource) -> *mut OH_AVSource;
    /// Creates an OH_AVSource instance that models the media at the URI.
    ///
    /// Required System Capabilities: SystemCapability.Multimedia.Media.Spliter
    /// # Arguments
    ///
    /// * `uri` - An URI for a remote media resource.
    ///
    /// # Returns
    ///
    /// * Returns a pointer to an OH_AVSource instance if the execution is successful, otherwise returns nullptr.
    /// Possible failure causes:
    /// 1. network anomaly.
    /// 2. resource is invalid.
    /// 3. file format is not supported.
    ///
    /// Available since API-level: 10
    pub fn OH_AVSource_CreateWithURI(uri: *mut ::core::ffi::c_char) -> *mut OH_AVSource;
    /// Creates an OH_AVSource instance that models the media at the FileDescriptor.
    ///
    /// Required System Capabilities: SystemCapability.Multimedia.Media.Spliter
    /// # Arguments
    ///
    /// * `fd` - The fileDescriptor of data source.
    ///
    /// * `offset` - The offset into the file to start reading.
    ///
    /// * `size` - The file size in bytes.
    ///
    /// # Returns
    ///
    /// * Returns a pointer to an OH_AVSource instance if the execution is successful, otherwise returns nullptr.
    /// Possible failure causes:
    /// 1. fd is invalid.
    /// 2. offset is not start pos of resource.
    /// 3. size error.
    /// 4. resource is invalid.
    /// 5. file format is not supported.
    ///
    /// Available since API-level: 10
    pub fn OH_AVSource_CreateWithFD(fd: i32, offset: i64, size: i64) -> *mut OH_AVSource;
    /// Destroy the OH_AVSource instance and free the internal resources.
    ///
    /// Required System Capabilities: SystemCapability.Multimedia.Media.Spliter
    /// # Arguments
    ///
    /// * `source` - Pointer to an OH_AVSource instance.
    ///
    /// # Returns
    ///
    /// * Returns AV_ERR_OK if the execution is successful,
    /// otherwise returns a specific error code, refer to [`OH_AVErrCode`]
    /// [`AV_ERR_INVALID_VAL`] source is invalid.
    ///
    /// Available since API-level: 10
    pub fn OH_AVSource_Destroy(source: *mut OH_AVSource) -> OH_AVErrCode;
    /// Get the format info of source.
    /// It should be noted that the life cycle of the OH_AVFormat instance pointed to by the return value * needs
    /// to be manually released by the caller.
    ///
    /// Required System Capabilities: SystemCapability.Multimedia.Media.Spliter
    /// # Arguments
    ///
    /// * `source` - Pointer to an OH_AVSource instance.
    ///
    /// # Returns
    ///
    /// * Returns the source's format info if the execution is successful, otherwise returns nullptr.
    /// Possible failure causes:
    /// 1. source is invalid.
    ///
    /// Available since API-level: 10
    pub fn OH_AVSource_GetSourceFormat(source: *mut OH_AVSource) -> *mut OH_AVFormat;
    /// Get the format info of track.
    /// It should be noted that the life cycle of the OH_AVFormat instance pointed to by the return value * needs
    /// to be manually released by the caller.
    ///
    /// Required System Capabilities: SystemCapability.Multimedia.Media.Spliter
    /// # Arguments
    ///
    /// * `source` - Pointer to an OH_AVSource instance.
    ///
    /// * `trackIndex` - The track index to get format.
    ///
    /// # Returns
    ///
    /// * Returns the track's format info if the execution is successful, otherwise returns nullptr.
    /// Possible failure causes:
    /// 1. source is invalid.
    /// 2. trackIndex is out of range.
    ///
    /// Available since API-level: 10
    pub fn OH_AVSource_GetTrackFormat(
        source: *mut OH_AVSource,
        trackIndex: u32,
    ) -> *mut OH_AVFormat;
    /// Get the format info of custom metadata.
    ///
    /// It should be noted that the life cycle of the OH_AVFormat instance pointed to by the return value * needs
    /// to be manually released by the caller.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Multimedia.Media.Spliter
    /// # Arguments
    ///
    /// * `source` - Pointer to an OH_AVSource instance.
    ///
    /// # Returns
    ///
    /// * Returns the metadata's format info if the execution is successful, otherwise returns nullptr.
    /// Possible failure causes:
    /// 1. source is invalid.
    ///
    /// Available since API-level: 18
    #[cfg(feature = "api-18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-18")))]
    pub fn OH_AVSource_GetCustomMetadataFormat(source: *mut OH_AVSource) -> *mut OH_AVFormat;
}
