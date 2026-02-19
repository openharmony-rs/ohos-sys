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

    #[cfg(feature = "api-13")]
    unsafe {
        let _ = image::native_image::common::OH_PictureMetadata_Create(
            std::mem::zeroed(),
            ptr::null_mut(),
        );
    }

    #[cfg(feature = "api-19")]
    unsafe {
        let _ = image::native_image::common::OH_PictureMetadata_GetPropertyWithNull(
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut(),
        );
    }

    #[cfg(feature = "image-packer")]
    unsafe {
        let _ = image::native_image::image_packer::OH_ImagePackerNative_Create(ptr::null_mut());
    }

    #[cfg(all(feature = "image-packer", feature = "api-13"))]
    unsafe {
        let _ = image::native_image::image_packer::OH_ImagePackerNative_PackToDataFromPicture(
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut(),
        );
    }

    #[cfg(all(feature = "image-packer", feature = "api-18"))]
    unsafe {
        let _ =
            image::native_image::image_packer::OH_PackingOptionsForSequence_Create(ptr::null_mut());
    }

    #[cfg(all(feature = "image-packer", feature = "api-19"))]
    unsafe {
        let _ = image::native_image::image_packer::OH_PackingOptions_GetMimeTypeWithNull(
            ptr::null_mut(),
            ptr::null_mut(),
        );
    }

    #[cfg(all(feature = "image-packer", feature = "api-20"))]
    unsafe {
        let _ = image::native_image::image_packer::OH_ImagePackerNative_GetSupportedFormats(
            ptr::null_mut(),
            ptr::null_mut(),
        );
    }

    #[cfg(feature = "image-receiver")]
    unsafe {
        let _ =
            image::native_image::image_receiver::OH_ImageReceiverOptions_Create(ptr::null_mut());
    }

    #[cfg(all(feature = "image-receiver", feature = "api-20"))]
    unsafe {
        let _ = image::native_image::image_receiver::OH_ImageReceiverNative_OnImageArrive(
            ptr::null_mut(),
            None,
            ptr::null_mut(),
        );
    }

    #[cfg(feature = "image-source")]
    unsafe {
        let _ = image::native_image::image_source::OH_ImageSourceInfo_Create(ptr::null_mut());
    }

    #[cfg(all(feature = "image-source", feature = "api-15"))]
    unsafe {
        let _ =
            image::native_image::image_source::OH_ImageSourceNative_CreatePixelmapUsingAllocator(
                ptr::null_mut(),
                ptr::null_mut(),
                std::mem::zeroed(),
                ptr::null_mut(),
            );
    }

    #[cfg(all(feature = "image-source", feature = "api-18"))]
    unsafe {
        let _ = image::native_image::image_source::OH_DecodingOptions_SetCropAndScaleStrategy(
            ptr::null_mut(),
            std::mem::zeroed(),
        );
    }

    #[cfg(all(feature = "image-source", feature = "api-19"))]
    unsafe {
        let _ = image::native_image::image_source::OH_DecodingOptions_SetCropRegion(
            ptr::null_mut(),
            std::mem::zeroed(),
        );
    }

    #[cfg(all(feature = "image-source", feature = "api-20"))]
    unsafe {
        let _ = image::native_image::image_source::OH_ImageSourceInfo_GetMimeType(
            ptr::null_mut(),
            ptr::null_mut(),
        );
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

    #[cfg(all(feature = "pixelmap", feature = "api-13"))]
    unsafe {
        let _ = image::native_image::pixelmap::OH_PixelmapNative_GetArgbPixels(
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut(),
        );
    }

    #[cfg(all(feature = "pixelmap", feature = "api-15"))]
    unsafe {
        let _ = image::native_image::pixelmap::OH_PixelmapNative_AccessPixels(
            ptr::null_mut(),
            ptr::null_mut(),
        );
    }

    #[cfg(all(feature = "pixelmap", feature = "api-18"))]
    unsafe {
        let _ = image::native_image::pixelmap::OH_PixelmapInitializationOptions_GetEditable(
            ptr::null_mut(),
            ptr::null_mut(),
        );
    }

    #[cfg(all(feature = "pixelmap", feature = "api-20"))]
    unsafe {
        let _ = image::native_image::pixelmap::OH_PixelmapImageInfo_GetAlphaMode(
            ptr::null_mut(),
            ptr::null_mut(),
        );
    }
}
