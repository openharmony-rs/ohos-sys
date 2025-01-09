// automatically generated by rust-bindgen 0.71.1

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use crate::native_type::*;

#[repr(C)]
pub struct ArkUI_StyledString {
    _unused: [u8; 0],
}
extern "C" {
    /// Free the memory occupied by the ArkUI_StyledString object.
    ///
    /// # Arguments
    ///
    /// * `handle` - A pointer to the ArkUI_StyledString object.
    ///
    /// Available since API-level: 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_ArkUI_StyledString_Destroy(handle: *mut ArkUI_StyledString);
    /// Sets the corresponding text content based on the current format string style.
    ///
    /// # Arguments
    ///
    /// * `handle` - A pointer to the ArkUI_StyledString object.
    ///
    /// * `content` - A pointer to the text content.
    ///
    /// Available since API-level: 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_ArkUI_StyledString_AddText(
        handle: *mut ArkUI_StyledString,
        content: *const ::core::ffi::c_char,
    );
    /// Removes the top style from the stack in the current format string object.
    ///
    /// # Arguments
    ///
    /// * `handle` - A pointer to the ArkUI_StyledString object.
    ///
    /// Available since API-level: 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_ArkUI_StyledString_PopTextStyle(handle: *mut ArkUI_StyledString);
}
