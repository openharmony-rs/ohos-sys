use std::ptr;

use ohos_drawing_sys as drawing;

fn touch_type<T>() {
    let _ = std::mem::size_of::<T>();
}

#[test]
fn link_smoke() {
    unsafe {
        let _ = drawing::bitmap::OH_Drawing_BitmapCreate();
        let _ = drawing::brush::OH_Drawing_BrushCreate();
        let _ = drawing::canvas::OH_Drawing_CanvasCreate();
        let _ = drawing::color::OH_Drawing_ColorSetArgb(0, 0, 0, 0);
        let _ = drawing::font_collection::OH_Drawing_CreateFontCollection();
        let _ = drawing::path::OH_Drawing_PathCreate();
        let _ = drawing::pen::OH_Drawing_PenCreate();
        let _ = drawing::text_typography::OH_Drawing_CreateTypographyStyle();
    }

    touch_type::<drawing::text_declaration::OH_Drawing_TypographyStyle>();
    touch_type::<drawing::types::OH_Drawing_Canvas>();

    #[cfg(feature = "api-11")]
    unsafe {
        let _ = drawing::color_filter::OH_Drawing_ColorFilterCreateBlendMode(0, std::mem::zeroed());
        let _ = drawing::filter::OH_Drawing_FilterCreate();
        let _ = drawing::font::OH_Drawing_FontCreate();
        let _ =
            drawing::mask_filter::OH_Drawing_MaskFilterCreateBlur(std::mem::zeroed(), 0.0, false);
        let _ = drawing::matrix::OH_Drawing_MatrixCreate();
        let _ = drawing::point::OH_Drawing_PointCreate(0.0, 0.0);
        let _ = drawing::rect::OH_Drawing_RectCreate(0.0, 0.0, 0.0, 0.0);
        let _ = drawing::register_font::OH_Drawing_RegisterFont(
            ptr::null_mut(),
            ptr::null(),
            ptr::null(),
        );
        let _ = drawing::round_rect::OH_Drawing_RoundRectCreate(ptr::null(), 0.0, 0.0);
        let _ = drawing::shader_effect::OH_Drawing_ShaderEffectCreateColorShader(0);
        let _ = drawing::text_blob::OH_Drawing_TextBlobBuilderCreate();
        let _ = drawing::typeface::OH_Drawing_TypefaceCreateDefault();
    }

    #[cfg(feature = "api-12")]
    unsafe {
        let _ = drawing::color_space::OH_Drawing_ColorSpaceCreateSrgb();
        let _ = drawing::error_code::OH_Drawing_ErrorCodeGet();
        let _ = drawing::font_mgr::OH_Drawing_FontMgrCreate();
        #[cfg(feature = "api-16")]
        let _ = drawing::gpu_context::OH_Drawing_GpuContextCreate();
        #[cfg(all(feature = "api-12", not(feature = "api-16")))]
        #[allow(deprecated)]
        let _ = drawing::gpu_context::OH_Drawing_GpuContextCreateFromGL(std::mem::zeroed());
        let _ = drawing::image::OH_Drawing_ImageCreate();
        let _ = drawing::image_filter::OH_Drawing_ImageFilterCreateBlur(
            0.0,
            0.0,
            std::mem::zeroed(),
            ptr::null_mut(),
        );
        let _ = drawing::memory_stream::OH_Drawing_MemoryStreamCreate(ptr::null(), 0, false);
        let _ = drawing::pixel_map::OH_Drawing_PixelMapGetFromNativePixelMap(ptr::null_mut());
        let _ = drawing::region::OH_Drawing_RegionCreate();
        let _ = drawing::sampling_options::OH_Drawing_SamplingOptionsCreate(
            std::mem::zeroed(),
            std::mem::zeroed(),
        );
        let _ = drawing::shadow_layer::OH_Drawing_ShadowLayerCreate(0.0, 0.0, 0.0, 0);
        let _ = drawing::surface::OH_Drawing_SurfaceCreateFromGpuContext(
            ptr::null_mut(),
            false,
            std::mem::zeroed(),
        );
    }

    #[cfg(feature = "api-13")]
    unsafe {
        let _ = drawing::record_cmd::OH_Drawing_RecordCmdUtilsCreate();
    }

    #[cfg(feature = "api-14")]
    unsafe {
        let _ = drawing::text_font_descriptor::OH_Drawing_GetSystemFontFullNamesByType(
            std::mem::zeroed(),
        );
    }

    #[cfg(feature = "api-18")]
    unsafe {
        let _ = drawing::path_effect::OH_Drawing_CreateComposePathEffect(
            ptr::null_mut(),
            ptr::null_mut(),
        );
        let _ = drawing::text_line::OH_Drawing_TypographyGetTextLines(ptr::null_mut());
        let _ = drawing::text_line_typography::OH_Drawing_CreateLineTypography(ptr::null_mut());
        let _ = drawing::text_run::OH_Drawing_GetRunStringIndices(ptr::null_mut(), 0, 0);
    }

    #[cfg(feature = "api-20")]
    unsafe {
        let _ = drawing::text_global::OH_Drawing_SetTextHighContrast(std::mem::zeroed());
    }
}
