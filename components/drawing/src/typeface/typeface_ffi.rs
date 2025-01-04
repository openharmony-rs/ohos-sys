// automatically generated by rust-bindgen 0.71.1

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use crate::types::*;

#[allow(unused_imports)]
#[cfg(feature = "api-12")]
use crate::error_code::OH_Drawing_ErrorCode;

extern "C" {
    /// Creates a default <b>OH_Drawing_Typeface</b> object.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    ///
    /// # Returns
    ///
    /// Returns the pointer to the <b>OH_Drawing_Typeface</b> object created.
    ///
    /// Available since API-level: 11
    ///
    /// Version: 1.0
    #[cfg(feature = "api-11")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-11")))]
    pub fn OH_Drawing_TypefaceCreateDefault() -> *mut OH_Drawing_Typeface;
    /// Creates a <b>OH_Drawing_Typeface</b> object by file.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// `path` - file path.
    ///
    /// `index` - file index.
    ///
    /// # Returns
    ///
    /// Returns the pointer to the <b>OH_Drawing_Typeface</b> object created.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_TypefaceCreateFromFile(
        path: *const ::core::ffi::c_char,
        index: ::core::ffi::c_int,
    ) -> *mut OH_Drawing_Typeface;
    /// Creates a <b>OH_Drawing_Typeface</b> object by given a stream. If the stream is not a valid
    /// font file, returns nullptr. Ownership of the stream is transferred, so the caller must not reference
    /// it or free it again.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// `OH_Drawing_MemoryStream` - Indicates the pointer to an <b>OH_Drawing_MemoryStream</b> object.
    ///
    /// `index` - memory stream index.
    ///
    /// # Returns
    ///
    /// Returns the pointer to the <b>OH_Drawing_Typeface</b> object created.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_TypefaceCreateFromStream(
        arg1: *mut OH_Drawing_MemoryStream,
        index: i32,
    ) -> *mut OH_Drawing_Typeface;
    /// Destroys an <b>OH_Drawing_Typeface</b> object and reclaims the memory occupied by the object.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// `OH_Drawing_Typeface` - Indicates the pointer to an <b>OH_Drawing_Typeface</b> object.
    ///
    /// Available since API-level: 11
    ///
    /// Version: 1.0
    #[cfg(feature = "api-11")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-11")))]
    pub fn OH_Drawing_TypefaceDestroy(arg1: *mut OH_Drawing_Typeface);
}
