// automatically generated by rust-bindgen 0.71.1

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use crate::types::*;

#[allow(unused_imports)]
#[cfg(feature = "api-12")]
use crate::error_code::OH_Drawing_ErrorCode;
use crate::text_typography::*;

extern "C" {
    /// Creates an <b>OH_Drawing_FontMgr</b> object.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    ///
    /// # Returns
    ///
    /// Returns the pointer to the <b>OH_Drawing_FontMgr</b> object created.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_FontMgrCreate() -> *mut OH_Drawing_FontMgr;
    /// Releases the memory occupied by an <b>OH_Drawing_FontMgr</b> object.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// `OH_Drawing_FontMgr` - Indicates the pointer to an <b>OH_Drawing_FontMgr</b> object.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_FontMgrDestroy(arg1: *mut OH_Drawing_FontMgr);
    /// Gets the count of font families.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// `OH_Drawing_FontMgr` - Indicates the pointer to an <b>OH_Drawing_FontMgr</b> object.
    ///
    /// # Returns
    ///
    /// Returns the count of font families.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_FontMgrGetFamilyCount(arg1: *mut OH_Drawing_FontMgr) -> ::core::ffi::c_int;
    /// Gets the font family name by the index.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// `OH_Drawing_FontMgr` - Indicates the pointer to an <b>OH_Drawing_FontMgr</b> object.
    ///
    /// `index` - Indicates the index to get the font family name.
    ///
    /// # Returns
    ///
    /// Returns the font family name corresponding to the index value.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_FontMgrGetFamilyName(
        arg1: *mut OH_Drawing_FontMgr,
        index: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    /// Releases the memory occupied by font family name.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// `familyName` - Indicates the font family name.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_FontMgrDestroyFamilyName(familyName: *mut ::core::ffi::c_char);
    /// Creates an <b>OH_Drawing_FontStyleSet</b> object by <b>OH_Drawing_FontMgr</b> object.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// `OH_Drawing_FontMgr` - Indicates the pointer to an <b>OH_Drawing_FontMgr</b> object.
    ///
    /// `index` - Indicates the index used to get the font style set object from the font manager object.
    ///
    /// # Returns
    ///
    /// Returns the pointer to the <b>OH_Drawing_FontStyleSet</b> object created.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_FontMgrCreateFontStyleSet(
        arg1: *mut OH_Drawing_FontMgr,
        index: ::core::ffi::c_int,
    ) -> *mut OH_Drawing_FontStyleSet;
    /// Releases the memory occupied by an <b>OH_Drawing_FontStyleSet</b> object.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// `OH_Drawing_FontStyleSet` - Indicates the pointer to an <b>OH_Drawing_FontStyleSet</b> object.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_FontMgrDestroyFontStyleSet(arg1: *mut OH_Drawing_FontStyleSet);
    /// Get the pointer to an <b>OH_Drawing_FontStyleSet</b> object for the given font style set family name.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// `OH_Drawing_FontMgr` - Indicates the pointer to an <b>OH_Drawing_FontMgr</b> object.
    ///
    /// `familyName` - Indicates the family name of a font style set to be matched.
    ///
    /// # Returns
    ///
    /// Returns the pointer to the <b>OH_Drawing_FontStyleSet</b> object matched.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_FontMgrMatchFamily(
        arg1: *mut OH_Drawing_FontMgr,
        familyName: *const ::core::ffi::c_char,
    ) -> *mut OH_Drawing_FontStyleSet;
    /// Get the pointer to an <b>OH_Drawing_Typeface</b> object based on the given font style and family name.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// `OH_Drawing_FontMgr` - Indicates the pointer to an <b>OH_Drawing_FontMgr</b> object.
    ///
    /// `familyName` - Indicates the family name of a font style set to be matched.
    ///
    /// `OH_Drawing_FontStyleStruct` - Indicates an <b>OH_Drawing_FontStyleStruct</b> object.
    ///
    /// # Returns
    ///
    /// Returns the pointer to the <b>OH_Drawing_Typeface</b> object matched.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_FontMgrMatchFamilyStyle(
        arg1: *mut OH_Drawing_FontMgr,
        familyName: *const ::core::ffi::c_char,
        fontStyle: OH_Drawing_FontStyleStruct,
    ) -> *mut OH_Drawing_Typeface;
    /// Get the pointer to an <b>OH_Drawing_Typeface</b> object for the given character.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// `OH_Drawing_FontMgr` - Indicates the pointer to an <b>OH_Drawing_FontMgr</b> object.
    ///
    /// `familyName` - Indicates the family name of a font style set to be matched.
    ///
    /// `OH_Drawing_FontStyleStruct` - Indicates an <b>OH_Drawing_FontStyleStruct</b> object.
    ///
    /// `bcp47` - Indicates an array of languages which indicate the language of character.
    ///
    /// `bcp47Count` - Indicates the array size of bcp47.
    ///
    /// `character` - Indicates a UTF8 value to be matched.
    ///
    /// # Returns
    ///
    /// Returns the pointer to the <b>OH_Drawing_Typeface</b> object matched.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_FontMgrMatchFamilyStyleCharacter(
        arg1: *mut OH_Drawing_FontMgr,
        familyName: *const ::core::ffi::c_char,
        fontStyle: OH_Drawing_FontStyleStruct,
        bcp47: *mut *const ::core::ffi::c_char,
        bcp47Count: ::core::ffi::c_int,
        character: i32,
    ) -> *mut OH_Drawing_Typeface;
    /// Create a typeface for the given index.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// `OH_Drawing_FontStyleSet` - Indicates the pointer to an <b>OH_Drawing_FontStyleSet</b> object.
    ///
    /// `index` - Indicates the index of the typeface in this fontStyleSet.
    ///
    /// # Returns
    ///
    /// If successful, return a pointer to <b>OH_Drawing_Typeface</b> object; if failed, return nullptr.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_FontStyleSetCreateTypeface(
        arg1: *mut OH_Drawing_FontStyleSet,
        index: ::core::ffi::c_int,
    ) -> *mut OH_Drawing_Typeface;
    /// Get font style for the specified typeface.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// `OH_Drawing_FontStyleSet` - Indicates the pointer to an <b>OH_Drawing_FontStyleSet</b> object.
    ///
    /// `index` - Indicates the index of the typeface in this fontStyleSet.
    ///
    /// `styleName` - Indicates the style name returned.
    ///
    /// # Returns
    ///
    /// Return the <b>OH_Drawing_FontStyleStruct<b> structure.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_FontStyleSetGetStyle(
        arg1: *mut OH_Drawing_FontStyleSet,
        index: i32,
        styleName: *mut *mut ::core::ffi::c_char,
    ) -> OH_Drawing_FontStyleStruct;
    /// Releases the memory styleName string.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// `styleName` - Indicates the pointer to a string type.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_FontStyleSetFreeStyleName(styleName: *mut *mut ::core::ffi::c_char);
    /// Get the closest matching typeface.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// `OH_Drawing_FontStyleSet` - Indicates the pointer to an <b>OH_Drawing_FontStyleSet</b> object.
    ///
    /// `fontStyleStruct` - Indicates the <b>OH_Drawing_FontStyleStruct</b> structure.
    ///
    /// # Returns
    ///
    /// A pointer to matched <b>OH_Drawing_Typeface</b>.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_FontStyleSetMatchStyle(
        arg1: *mut OH_Drawing_FontStyleSet,
        fontStyleStruct: OH_Drawing_FontStyleStruct,
    ) -> *mut OH_Drawing_Typeface;
    /// Get the count of typeface.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// `OH_Drawing_FontStyleSet` - Indicates the pointer to an <b>OH_Drawing_FontStyleSet</b> object.
    ///
    /// # Returns
    ///
    /// The count of typeface in this font style set.
    ///
    /// Available since API-level: 12
    ///
    /// Version: 1.0
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_Drawing_FontStyleSetCount(arg1: *mut OH_Drawing_FontStyleSet) -> ::core::ffi::c_int;
}
