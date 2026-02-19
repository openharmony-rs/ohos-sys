#![cfg(feature = "api-12")]

use std::ptr;

use ohos_image_kit_sys as image;

fn touch_type<T>() {
    let _ = std::mem::size_of::<T>();
}

#[test]
fn link_smoke() {
    unsafe {
        let _ = image::native_image::image::OH_ImageNative_GetImageSize(
            ptr::null_mut(),
            ptr::null_mut(),
        );
    }

    touch_type::<image::native_image::common::ImageResult>();

    #[cfg(feature = "image-packer")]
    unsafe {
        let _ = image::native_image::image_packer::OH_ImagePackerNative_Create(ptr::null_mut());
    }

    #[cfg(feature = "image-receiver")]
    unsafe {
        let _ =
            image::native_image::image_receiver::OH_ImageReceiverOptions_Create(ptr::null_mut());
    }

    #[cfg(feature = "image-source")]
    unsafe {
        let _ = image::native_image::image_source::OH_ImageSourceInfo_Create(ptr::null_mut());
    }

    #[cfg(all(feature = "api-13", feature = "pixelmap"))]
    unsafe {
        let _ = image::native_image::picture::OH_PictureNative_CreatePicture(
            ptr::null_mut(),
            ptr::null_mut(),
        );
    }

    #[cfg(feature = "pixelmap")]
    unsafe {
        let _ =
            image::native_image::pixelmap::OH_PixelmapInitializationOptions_Create(ptr::null_mut());
    }
}
