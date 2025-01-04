// automatically generated by rust-bindgen 0.71.1

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use crate::cursor_info::InputMethod_CursorInfo;
use crate::text_avoid_info::InputMethod_TextAvoidInfo;
use crate::types::*;

#[repr(C)]
pub struct InputMethod_TextConfig {
    _unused: [u8; 0],
}
extern "C" {
    /// Create a new [`InputMethod_TextConfig`] instance.
    ///
    ///
    /// # Returns
    ///
    /// If the creation succeeds, a pointer to the newly created [`InputMethod_TextConfig`]
    /// instance is returned. If the creation fails, NULL is returned, possible cause is insufficient memory.
    ///
    /// Available since API-level: 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_TextConfig_Create() -> *mut InputMethod_TextConfig;
    /// Destroy a [`InputMethod_TextConfig`] instance.
    ///
    /// # Arguments
    ///
    /// `config` - Represents a pointer to an [`InputMethod_TextConfig`] instance which will be destroyed.
    ///
    /// Available since API-level: 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_TextConfig_Destroy(config: *mut InputMethod_TextConfig);
    /// Set input type into TextConfig.
    ///
    /// # Arguments
    ///
    /// `config` - Represents a pointer to an [`InputMethod_TextConfig`] instance which will be set.
    ///
    /// `inputType` - The text input type of text Editor, which is defined in [`InputMethod_TextInputType`].
    ///
    /// # Returns
    ///
    /// Returns a specific error code.
    /// [`IME_ERR_OK`] - success.
    /// [`IME_ERR_NULL_POINTER`] - unexpected null pointer.
    /// Specific error codes can be referenced [`InputMethod_ErrorCode`].
    ///
    /// Available since API-level: 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_TextConfig_SetInputType(
        config: *mut InputMethod_TextConfig,
        inputType: InputMethod_TextInputType,
    ) -> InputMethod_ErrorCode;
    /// Set enter key type into TextConfig.
    ///
    /// # Arguments
    ///
    /// `config` - Represents a pointer to an [`InputMethod_TextConfig`] instance which will be set.
    ///
    /// `enterKeyType` - The enter key type of text Editor, which is defined in [`InputMethod_EnterKeyType`].
    ///
    /// # Returns
    ///
    /// Returns a specific error code.
    /// [`IME_ERR_OK`] - success.
    /// [`IME_ERR_NULL_POINTER`] - unexpected null pointer.
    /// Specific error codes can be referenced [`InputMethod_ErrorCode`].
    ///
    /// Available since API-level: 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_TextConfig_SetEnterKeyType(
        config: *mut InputMethod_TextConfig,
        enterKeyType: InputMethod_EnterKeyType,
    ) -> InputMethod_ErrorCode;
    /// Set preview text support into TextConfig.
    ///
    /// # Arguments
    ///
    /// `config` - Represents a pointer to an [`InputMethod_TextConfig`] instance which will be set.
    ///
    /// `supported` - Indicates whether the preview text is supported.
    ///
    /// # Returns
    ///
    /// Returns a specific error code.
    /// [`IME_ERR_OK`] - success.
    /// [`IME_ERR_NULL_POINTER`] - unexpected null pointer.
    /// Specific error codes can be referenced [`InputMethod_ErrorCode`].
    ///
    /// Available since API-level: 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_TextConfig_SetPreviewTextSupport(
        config: *mut InputMethod_TextConfig,
        supported: bool,
    ) -> InputMethod_ErrorCode;
    /// Set selection into TextConfig.
    ///
    /// # Arguments
    ///
    /// `config` - Represents a pointer to an [`InputMethod_TextConfig`] instance which will be set.
    ///
    /// `start` - The start position of selection.
    ///
    /// `end` - The end position of selection.
    ///
    /// # Returns
    ///
    /// Returns a specific error code.
    /// [`IME_ERR_OK`] - success.
    /// [`IME_ERR_NULL_POINTER`] - unexpected null pointer.
    /// Specific error codes can be referenced [`InputMethod_ErrorCode`].
    ///
    /// Available since API-level: 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_TextConfig_SetSelection(
        config: *mut InputMethod_TextConfig,
        start: i32,
        end: i32,
    ) -> InputMethod_ErrorCode;
    /// Set window id into TextConfig.
    ///
    /// # Arguments
    ///
    /// `config` - Represents a pointer to an [`InputMethod_TextConfig`] instance which will be set.
    ///
    /// `windowId` - The window ID of the application currently bound to the input method.
    ///
    /// # Returns
    ///
    /// Returns a specific error code.
    /// [`IME_ERR_OK`] - success.
    /// [`IME_ERR_NULL_POINTER`] - unexpected null pointer.
    /// Specific error codes can be referenced [`InputMethod_ErrorCode`].
    ///
    /// Available since API-level: 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_TextConfig_SetWindowId(
        config: *mut InputMethod_TextConfig,
        windowId: i32,
    ) -> InputMethod_ErrorCode;
    /// Get input type from TextConfig
    ///
    /// # Arguments
    ///
    /// `config` - Represents a pointer to an [`InputMethod_TextConfig`] instance which will be get from.
    ///
    /// `inputType` - Represents a pointer to an [`InputMethod_TextInputType`] instance.
    /// The text input type of text Editor
    ///
    /// # Returns
    ///
    /// Returns a specific error code.
    /// [`IME_ERR_OK`] - success.
    /// [`IME_ERR_NULL_POINTER`] - unexpected null pointer.
    /// Specific error codes can be referenced [`InputMethod_ErrorCode`].
    ///
    /// Available since API-level: 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_TextConfig_GetInputType(
        config: *mut InputMethod_TextConfig,
        inputType: *mut InputMethod_TextInputType,
    ) -> InputMethod_ErrorCode;
    /// Get enter key type from TextConfig
    ///
    /// # Arguments
    ///
    /// `config` - Represents a pointer to an [`InputMethod_TextConfig`] instance which will be get from.
    ///
    /// `enterKeyType` - Represents a pointer to an [`InputMethod_EnterKeyType`] instance.
    /// Indicates the enter key type of text Editor
    ///
    /// # Returns
    ///
    /// Returns a specific error code.
    /// [`IME_ERR_OK`] - success.
    /// [`IME_ERR_NULL_POINTER`] - unexpected null pointer.
    /// Specific error codes can be referenced [`InputMethod_ErrorCode`].
    ///
    /// Available since API-level: 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_TextConfig_GetEnterKeyType(
        config: *mut InputMethod_TextConfig,
        enterKeyType: *mut InputMethod_EnterKeyType,
    ) -> InputMethod_ErrorCode;
    /// Get is preview text supported from TextConfig.
    ///
    /// # Arguments
    ///
    /// `config` - Represents a pointer to an [`InputMethod_TextConfig`] instance which will be get from.
    ///
    /// `supported` - Indicates whether the preview text is supported.
    ///
    /// # Returns
    ///
    /// Returns a specific error code.
    /// [`IME_ERR_OK`] - success.
    /// [`IME_ERR_NULL_POINTER`] - unexpected null pointer.
    /// Specific error codes can be referenced [`InputMethod_ErrorCode`].
    ///
    /// Available since API-level: 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_TextConfig_IsPreviewTextSupported(
        config: *mut InputMethod_TextConfig,
        supported: *mut bool,
    ) -> InputMethod_ErrorCode;
    /// Get cursor info from TextConfig.
    ///
    /// # Arguments
    ///
    /// `config` - Represents a pointer to an [`InputMethod_TextConfig`] instance which will be get from.
    ///
    /// `cursorInfo` - Represents a pointer to an [`InputMethod_CursorInfo`] instance.
    ///
    /// # Returns
    ///
    /// Returns a specific error code.
    /// [`IME_ERR_OK`] - success.
    /// [`IME_ERR_NULL_POINTER`] - unexpected null pointer.
    /// Specific error codes can be referenced [`InputMethod_ErrorCode`].
    ///
    /// Available since API-level: 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_TextConfig_GetCursorInfo(
        config: *mut InputMethod_TextConfig,
        cursorInfo: *mut *mut InputMethod_CursorInfo,
    ) -> InputMethod_ErrorCode;
    /// Get text avoid information from text configuration.
    ///
    /// # Arguments
    ///
    /// `config` - Indicates the text configuration.
    ///
    /// `avoidInfo` - Indicates the text avoid information.
    ///
    /// # Returns
    ///
    /// Returns a specific error code.
    /// [`IME_ERR_OK`] - success.
    /// [`IME_ERR_NULL_POINTER`] - unexpected null pointer.
    /// Specific error codes can be referenced [`InputMethod_ErrorCode`].
    ///
    /// Available since API-level: 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_TextConfig_GetTextAvoidInfo(
        config: *mut InputMethod_TextConfig,
        avoidInfo: *mut *mut InputMethod_TextAvoidInfo,
    ) -> InputMethod_ErrorCode;
    /// Get selection from TextConfig.
    ///
    /// # Arguments
    ///
    /// `config` - Represents a pointer to an [`InputMethod_TextConfig`] instance which will be get from.
    ///
    /// `start` - Represents selection start position.
    ///
    /// `end` - Represents selection end position.
    ///
    /// # Returns
    ///
    /// Returns a specific error code.
    /// [`IME_ERR_OK`] - success.
    /// [`IME_ERR_NULL_POINTER`] - unexpected null pointer.
    /// Specific error codes can be referenced [`InputMethod_ErrorCode`].
    ///
    /// Available since API-level: 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_TextConfig_GetSelection(
        config: *mut InputMethod_TextConfig,
        start: *mut i32,
        end: *mut i32,
    ) -> InputMethod_ErrorCode;
    /// Get window id from TextConfig.
    ///
    /// # Arguments
    ///
    /// `config` - Represents a pointer to an [`InputMethod_TextConfig`] instance which will be get from.
    ///
    /// `windowId` - The window ID of the application currently bound to the input method.
    ///
    /// # Returns
    ///
    /// Returns a specific error code.
    /// [`IME_ERR_OK`] - success.
    /// [`IME_ERR_NULL_POINTER`] - unexpected null pointer.
    /// Specific error codes can be referenced [`InputMethod_ErrorCode`].
    ///
    /// Available since API-level: 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_TextConfig_GetWindowId(
        config: *mut InputMethod_TextConfig,
        windowId: *mut i32,
    ) -> InputMethod_ErrorCode;
}
