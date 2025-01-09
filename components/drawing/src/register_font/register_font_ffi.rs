// automatically generated by rust-bindgen 0.71.1

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use crate::types::*;

#[allow(unused_imports)]
#[cfg(feature = "api-12")]
use crate::error_code::OH_Drawing_ErrorCode;
use crate::text_declaration::*;

extern "C" {
    /// Defines an <b>OH_Drawing_RegisterFont</b>, which is used to register a customized font in the FontManager.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `OH_Drawing_FontCollection` - Indicates the pointer to an <b>OH_Drawing_FontCollection</b> object.
    ///
    /// * `fontFamily` - Indicates the family-name of the font which need to register.
    ///
    /// * `familySrc` - Indicates the path of the font file which need to register.
    ///
    /// # Returns
    ///
    /// * error code.
    ///
    /// Available since API-level: 11
    ///
    /// Version: 1.0
    #[cfg(feature = "api-11")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-11")))]
    pub fn OH_Drawing_RegisterFont(
        arg1: *mut OH_Drawing_FontCollection,
        fontFamily: *const ::core::ffi::c_char,
        familySrc: *const ::core::ffi::c_char,
    ) -> u32;
    /// Defines an <b>OH_Drawing_RegisterFontBuffer</b>, which is used to register a customized font in the
    /// FontManager.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `OH_Drawing_FontCollection` - Indicates the pointer to an <b>OH_Drawing_FontCollection</b> object.
    ///
    /// * `fontFamily` - Indicates the family-name of the font which need to register.
    ///
    /// * `fontBuffer` - Indicates the buffer of the font file which need to register.
    ///
    /// * `length` - Indicates the length of the font file which need to register.
    ///
    /// # Returns
    ///
    /// * error code.
    ///
    /// Available since API-level: 11
    ///
    /// Version: 1.0
    #[cfg(feature = "api-11")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-11")))]
    pub fn OH_Drawing_RegisterFontBuffer(
        arg1: *mut OH_Drawing_FontCollection,
        fontFamily: *const ::core::ffi::c_char,
        fontBuffer: *mut u8,
        length: usize,
    ) -> u32;
}
