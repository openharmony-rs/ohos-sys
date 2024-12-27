/* automatically generated by rust-bindgen 0.71.1 */

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use crate::types::*;

#[repr(C)]
pub struct InputMethod_AttachOptions {
    _unused: [u8; 0],
}
extern "C" {
    /** @brief Create a new {@link InputMethod_AttachOptions} instance.

    @param showKeyboard Represents whether to show the keyboard.
    @return If the creation succeeds, a pointer to the newly created {@link InputMethod_AttachOptions}
    instance is returned. If the creation fails, NULL is returned, possible cause is insufficient memory.
    @since 12*/
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_AttachOptions_Create(showKeyboard: bool) -> *mut InputMethod_AttachOptions;
    /** @brief Delete a {@link InputMethod_AttachOptions} instance.

    @param options Represents a pointer to an {@link InputMethod_AttachOptions} instance which will be destroyed.
    @since 12*/
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_AttachOptions_Destroy(options: *mut InputMethod_AttachOptions);
    /** @brief Get showKeyboard value from {@link InputMethod_AttachOptions}.

    @param options Represents a pointer to an {@link InputMethod_AttachOptions} instance which will be get value from.
    @param showKeyboard  Represents showKeyboard value.
        true - need to show keyboard.
        false - no need to show keyboard.
    @return Returns a specific error code.
        {@link IME_ERR_OK} - success.
        {@link IME_ERR_NULL_POINTER} - unexpected null pointer.
    Specific error codes can be referenced {@link InputMethod_ErrorCode}.
    @since 12*/
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_AttachOptions_IsShowKeyboard(
        options: *mut InputMethod_AttachOptions,
        showKeyboard: *mut bool,
    ) -> InputMethod_ErrorCode;
}