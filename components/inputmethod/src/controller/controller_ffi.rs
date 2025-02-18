// automatically generated by rust-bindgen 0.71.1

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use crate::attach_options::InputMethod_AttachOptions;
use crate::inputmethod_proxy::InputMethod_InputMethodProxy;
use crate::text_editor_proxy::InputMethod_TextEditorProxy;
use crate::types::*;

extern "C" {
    /// Attach application to the input method service.
    ///
    /// # Arguments
    ///
    /// * `textEditorProxy` - Represents a pointer to an [`InputMethod_TextEditorProxy`] instance.
    /// The caller needs to manage the lifecycle of textEditorProxy.
    /// If the call succeeds, caller cannot release textEditorProxy until the next attach or detach call.
    ///
    /// * `options` - Represents a pointer to an [`InputMethod_AttachOptions`] instance.
    /// The options when attaching input method.
    ///
    /// * `inputMethodProxy` - Represents a pointer to an [`InputMethod_InputMethodProxy`] instance.
    /// Lifecycle is mantianed until the next attach or detach call.
    ///
    /// # Returns
    ///
    /// * Returns a specific error code.
    /// [`IME_ERR_OK`] - success.
    /// [`IME_ERR_PARAMCHECK`] - parameter check failed.
    /// [`IME_ERR_IMCLIENT`] - input method client error.
    /// [`IME_ERR_IMMS`] - input method manager service error.
    /// [`IME_ERR_NULL_POINTER`] - unexpected null pointer.
    /// Specific error codes can be referenced [`InputMethod_ErrorCode`].
    ///
    /// Available since API-level: 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_InputMethodController_Attach(
        textEditorProxy: *mut InputMethod_TextEditorProxy,
        options: *mut InputMethod_AttachOptions,
        inputMethodProxy: *mut *mut InputMethod_InputMethodProxy,
    ) -> InputMethodResult;
    /// Detach application from the input method service.
    ///
    /// # Arguments
    ///
    /// * `inputMethodProxy` - Represents a pointer to an [`InputMethod_InputMethodProxy`] instance.
    /// The inputMethodProxy is obtained from [`OH_InputMethodController_Attach`].
    ///
    /// # Returns
    ///
    /// * Returns a specific error code.
    /// [`IME_ERR_OK`] - success.
    /// [`IME_ERR_IMCLIENT`] - input method client error.
    /// [`IME_ERR_IMMS`] - input method manager service error.
    /// [`IME_ERR_NULL_POINTER`] - unexpected null pointer.
    /// Specific error codes can be referenced [`InputMethod_ErrorCode`].
    ///
    /// Available since API-level: 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_InputMethodController_Detach(
        inputMethodProxy: *mut InputMethod_InputMethodProxy,
    ) -> InputMethodResult;
}
