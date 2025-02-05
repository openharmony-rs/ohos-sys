// automatically generated by rust-bindgen 0.71.1

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use crate::text_typography::OH_Drawing_FontDescriptor;
use crate::types::*;

#[cfg(feature = "api-14")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-14")))]
impl OH_Drawing_SystemFontType {
    /// All font types
    pub const ALL: OH_Drawing_SystemFontType = OH_Drawing_SystemFontType(1);
    /// System generic font type
    pub const GENERIC: OH_Drawing_SystemFontType = OH_Drawing_SystemFontType(2);
    /// Stylish font type
    pub const STYLISH: OH_Drawing_SystemFontType = OH_Drawing_SystemFontType(4);
    /// Installed font types
    pub const INSTALLED: OH_Drawing_SystemFontType = OH_Drawing_SystemFontType(8);
}
#[repr(transparent)]
/// An enumeration of system font types.
///
///
/// Available since API-level: 14
#[cfg(feature = "api-14")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-14")))]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct OH_Drawing_SystemFontType(pub ::core::ffi::c_uint);
extern "C" {
    /// Get the <b>OH_Drawing_FontDescriptor</b> object by the font full name and the font type, supporting generic
    /// fonts, stylish fonts, and installed fonts.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `fullName` - Indicates the full name object <b>OH_Drawing_String</b>.
    ///
    /// * `fontType` - Indicates enumerates of system font type object <b>OH_Drawing_SystemFontType</b>.
    ///
    /// # Returns
    ///
    /// * Returns the pointer to a font descriptor object <b>OH_Drawing_FontDescriptor</b>.
    ///
    /// Available since API-level: 14
    #[cfg(feature = "api-14")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-14")))]
    pub fn OH_Drawing_GetFontDescriptorByFullName(
        fullName: *const OH_Drawing_String,
        fontType: OH_Drawing_SystemFontType,
    ) -> *mut OH_Drawing_FontDescriptor;
    /// Obtain the corresponding font full name array by the font type.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `fontType` - Indicates enumerates of system font type object <b>OH_Drawing_SystemFontType</b>.
    ///
    /// # Returns
    ///
    /// * Returns the pointer to full name array object <b>OH_Drawing_Array</b>.
    ///
    /// Available since API-level: 14
    #[cfg(feature = "api-14")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-14")))]
    pub fn OH_Drawing_GetSystemFontFullNamesByType(
        fontType: OH_Drawing_SystemFontType,
    ) -> *mut OH_Drawing_Array;
    /// Get the specified full name object <b>OH_Drawing_String</b> by index from the
    /// <b>OH_Drawing_Array</b> object.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `fullNameArray` - Indicates an array of full name object <b>OH_Drawing_Array</b>.
    ///
    /// * `index` - The index of full name.
    ///
    /// # Returns
    ///
    /// * Returns a full name object <b>OH_Drawing_String</b>.
    ///
    /// Available since API-level: 14
    #[cfg(feature = "api-14")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-14")))]
    pub fn OH_Drawing_GetSystemFontFullNameByIndex(
        fullNameArray: *mut OH_Drawing_Array,
        index: usize,
    ) -> *const OH_Drawing_String;
    /// Releases the memory occupied by an array of font full names.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `fullNameArray` - Indicates an array of full name object <b>OH_Drawing_Array</b>.
    ///
    /// Available since API-level: 14
    #[cfg(feature = "api-14")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-14")))]
    pub fn OH_Drawing_DestroySystemFontFullNames(fullNameArray: *mut OH_Drawing_Array);
}
