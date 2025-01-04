//! Bindings for image_native.h
//!
//! <https://docs.openharmony.cn/pages/v5.0/en/application-dev/reference/apis-image-kit/image__native_8h.md>

#[link(name = "ohimage")]
extern "C" {}

mod image_ffi;
pub use image_ffi::*;

#[cfg(feature = "pixelmap")]
#[cfg_attr(docsrs, doc(cfg(feature = "pixelmap")))]
extern "C" {

    /// @brief Get byte buffer from an {@link OH_ImageNative} object by the component type.
    ///
    /// @param image Indicates the pointer to an {@link OH_ImageNative} object.
    /// @param componentType Indicates the type of component.
    /// @param nativeBuffer Indicates the pointer to the component buffer obtained.
    /// @return Returns {@link Image_ErrorCode} IMAGE_SUCCESS - if the operation is successful.
    /// returns {@link Image_ErrorCode} IMAGE_BAD_PARAMETER - if bad parameter.
    /// @since 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_ImageNative_GetByteBuffer(
        image: *mut OH_ImageNative,
        componentType: u32,
        nativeBuffer: *mut *mut crate::native_image::pixelmap::OH_NativeBuffer,
    ) -> crate::native_image::common::Image_ErrorCode;
}
