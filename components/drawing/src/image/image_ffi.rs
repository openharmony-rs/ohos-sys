// automatically generated by rust-bindgen 0.71.1

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use crate::types::*;

extern "C" {
    /// Creates an <b>OH_Drawing_Image</b> object.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    ///
    /// # Returns
    ///
    /// * Returns the pointer to the <b>OH_Drawing_Image</b> object created.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_ImageCreate() -> *mut OH_Drawing_Image;
    /// Destroys an <b>OH_Drawing_Image</b> object and reclaims the memory occupied by the object.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `image` - Indicates the pointer to an <b>OH_Drawing_Image</b> object.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_ImageDestroy(image: *mut OH_Drawing_Image);
    /// Rebuilds an <b>OH_Drawing_Image</b> object, sharing or copying bitmap pixels.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `image` - Indicates the pointer to an <b>OH_Drawing_Image</b> object.
    ///
    /// * `bitmap` - Indicates the pointer to an <b>OH_Drawing_Bitmap</b> object.
    ///
    /// # Returns
    ///
    /// * Returns true if successed.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_ImageBuildFromBitmap(
        image: *mut OH_Drawing_Image,
        bitmap: *mut OH_Drawing_Bitmap,
    ) -> bool;
    /// Gets pixel count in each row of image.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `image` - Indicates the pointer to an <b>OH_Drawing_Image</b> object.
    ///
    /// # Returns
    ///
    /// * Returns the width.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_ImageGetWidth(image: *mut OH_Drawing_Image) -> i32;
    /// Gets pixel row count of image.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `image` - Indicates the pointer to an <b>OH_Drawing_Image</b> object.
    ///
    /// # Returns
    ///
    /// * Returns the height.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_ImageGetHeight(image: *mut OH_Drawing_Image) -> i32;
    /// Gets the image info.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `image` - Indicates the pointer to an <b>OH_Drawing_Image</b> object.
    ///
    /// * `imageInfo` - Indicates the pointer to an <b>OH_Drawing_Image_Info</b> object.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_ImageGetImageInfo(
        image: *mut OH_Drawing_Image,
        imageInfo: *mut OH_Drawing_Image_Info,
    );
}
