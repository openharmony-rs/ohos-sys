//! Configuration file for OpenHarmony modules with multiple header files in a directory.
//!
//! Add new bindings to `get_module_bindings_config()` by appending a new `DirBindingsConf`.

use crate::DirBindingsConf;
use bindgen::callbacks::EnumVariantValue;
use bindgen::EnumVariation;
use log::{debug, info, trace};
use std::cell::{LazyCell, OnceCell};
use std::collections::HashMap;
use std::default;
use std::fmt::{Debug, Formatter};
use std::sync::LazyLock;

/// Convenience method for stripping am optional suffix and returning an owned String
fn strip_suffix(input: &str, suffix: &str) -> String {
    match input.strip_suffix(suffix) {
        None => input.to_string(),
        Some(stripped) => stripped.to_string(),
    }
}

/// Convenience method for stripping am optional suffix and returning an owned String
fn strip_prefix(input: &str, prefix: &str) -> String {
    match input.strip_prefix(prefix) {
        None => input.to_string(),
        Some(stripped) => stripped.to_string(),
    }
}

struct ResultEnumParseCallbacks {
    /// fn item_name(&self, original_item_name: &str) -> Option<String> {
    rename_item: Box<dyn Fn(&str) -> Option<String>>,
    /// Custom renaming logic for enum variants.
    ///
    /// By default, we just try to lookup the prefix in `ENUM_PREFIX_MAP` and remove that.
    rename_enum_variant: Option<Box<dyn Fn(&str, &str) -> Option<String>>>,
}

impl Default for ResultEnumParseCallbacks {
    fn default() -> Self {
        Self {
            rename_item: Box::new(|_| None),
            rename_enum_variant: None,
        }
    }
}

impl Debug for ResultEnumParseCallbacks {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("ResultEnumParseCallbacks")
    }
}

impl bindgen::callbacks::ParseCallbacks for ResultEnumParseCallbacks {
    fn item_name(&self, original_item_name: &str) -> Option<String> {
        (self.rename_item)(original_item_name)
    }
}

pub(crate) fn get_module_bindings_config() -> Vec<DirBindingsConf> {
    vec![
        DirBindingsConf {
            directory: "multimedia/player_framework".to_string(),
            output_dir: "components/multimedia/player_framework/src".to_string(),
            rename_output_file: Some(Box::new(|stem| strip_prefix(stem, "native_"))),
            set_builder_opts: Box::new(|file_stem, header_path, builder| {
                let builder = builder
                    .allowlist_file(format!("{}", header_path.to_str().unwrap()))
                    .allowlist_recursively(false)
                    .default_enum_style(EnumVariation::NewType {
                        is_bitfield: false,
                        is_global: false,
                        is_result_type: false,
                    });
                let builder = if file_stem != "averrors" {
                    builder.raw_line("#[allow(unused_imports)]use crate::averrors::OH_AVErrCode;")
                } else {
                    builder
                };
                match file_stem {
                    "avplayer" => builder.raw_line("use ohos_sys_opaque_types::OHNativeWindow;")
                        .raw_line("use crate::avplayer_base::{AVPlaybackSpeed, AVPlayerCallback, AVPlayerSeekMode, AVPlayerState, OH_AVPlayer};")
                        .raw_line("#[cfg(feature = \"api-12\")]use crate::avplayer_base::{OH_AVPlayerOnErrorCallback, OH_AVPlayerOnInfoCallback};")
                        // require bindings to OH audio.
                        .blocklist_function("OH_AVPlayer_SetAudioRendererInfo")
                        .blocklist_function("OH_AVPlayer_SetAudioInterruptMode")
                        .blocklist_function("OH_AVPlayer_SetAudioEffectMode")
                    ,
                    "avplayer_base" => builder.raw_line("#[cfg(feature = \"api-12\")]use crate::avformat::OH_AVFormat;"),
                    "avcapability" => builder
                        .raw_line("#[cfg(feature = \"api-12\")]use crate::avformat::OH_AVFormat;")
                        .raw_line("use crate::avcodec_base::OH_BitrateMode;"),

                    "avcodec_base" => builder
                        .raw_line("use crate::avbuffer_info::OH_AVCodecBufferAttr;")
                        .raw_line("use crate::avmemory::OH_AVMemory;")
                        .raw_line("use crate::avformat::OH_AVFormat;")
                        .raw_line("#[cfg(feature = \"api-11\")]use crate::avbuffer::OH_AVBuffer;")
                    ,
                    "avsource" => builder
                        .raw_line("#[cfg(feature = \"api-12\")]use crate::avcodec_base::OH_AVDataSource;")
                        .raw_line("use crate::avformat::OH_AVFormat;")
                    ,
                    "avbuffer" => builder.raw_line("use ohos_sys_opaque_types::OH_NativeBuffer;")
                        .raw_line("use crate::avbuffer_info::OH_AVCodecBufferAttr;")
                        .raw_line("use crate::avformat::OH_AVFormat;"),
                    "avbuffer_info" => builder
                        .bitfield_enum("OH_AVCodecBufferFlags")
                    ,
                    "avdemuxer" => builder
                        .raw_line("#[cfg(feature = \"api-11\")]use crate::avbuffer::OH_AVBuffer;")
                        .raw_line("use crate::avbuffer_info::OH_AVCodecBufferAttr;")
                        .raw_line("use crate::avcodec_base::OH_AVSeekMode;")
                        .raw_line("use crate::avsource::OH_AVSource;")
                        .raw_line("use crate::avmemory::OH_AVMemory;")
                    ,
                    _ => builder,
                }
            }),
        },
        DirBindingsConf {
            directory: "database/pasteboard".to_string(),
            output_dir: "components/pasteboard/src".to_string(),
            rename_output_file: Some(Box::new(|stem| strip_prefix(stem, "oh_"))),
            set_builder_opts: Box::new(|file_stem, header_path, builder| {
                let builder = builder
                    .allowlist_file(format!("{}", header_path.to_str().unwrap()))
                    .allowlist_recursively(false)
                    .default_enum_style(EnumVariation::NewType {
                        is_bitfield: false,
                        is_global: false,
                        is_result_type: false,
                    });
                match file_stem {
                    "pasteboard" => builder.raw_line("use ohos_sys_opaque_types::OH_UdmfData;"),
                    _ => builder,
                }
            }),
        },
        DirBindingsConf {
            directory: "database/udmf".to_string(),
            output_dir: "components/udmf/src".to_string(),
            rename_output_file: None,
            set_builder_opts: Box::new(|file_stem, header_path, builder| {
                let builder = builder
                    .allowlist_file(format!("{}", header_path.to_str().unwrap()))
                    .allowlist_recursively(false)
                    .default_enum_style(EnumVariation::NewType {
                        is_bitfield: false,
                        is_global: false,
                        is_result_type: false,
                    });

                match file_stem {
                    "udmf" => builder.raw_line("use ohos_sys_opaque_types::*;"),
                    "uds" => builder
                        .raw_line("pub use ohos_sys_opaque_types::{OH_UdsAppItem, OH_UdsHtml, OH_UdsHyperlink, OH_UdsPlainText};")
                        .raw_line("#[cfg(feature = \"api-13\")]use ohos_sys_opaque_types::OH_PixelmapNative;")
                        .raw_line("#[cfg(feature = \"api-13\")]pub use ohos_sys_opaque_types::{OH_UdsPixelMap, OH_UdsArrayBuffer, OH_UdsFileUri};")
                        .raw_line("#[cfg(feature = \"api-14\")]pub use ohos_sys_opaque_types::OH_UdsContentForm;")
                    ,
                    "utd" => builder.raw_line("pub use ohos_sys_opaque_types::OH_Utd;"),
                    _ => builder,
                }
            }),
        },
        DirBindingsConf {
            directory: "multimedia/image_framework/image".to_string(),
            output_dir: "components/multimedia/image_framework/src/native_image".to_string(),
            rename_output_file: Some(Box::new(|stem| strip_suffix(stem, "_native"))),
            set_builder_opts: Box::new(|file_stem, header_path, builder| {
                let builder = if file_stem != "image_common" {
                    builder.raw_line("use crate::native_image::common::*;")
                } else {
                    builder
                };
                let builder = builder.parse_callbacks(Box::new(ResultEnumParseCallbacks {
                    rename_item: Box::new(|original_item_name| match original_item_name {
                        "Image_ErrorCode" => Some("ImageResult".to_string()),
                        _ => None,
                    }),
                    ..Default::default()
                }));
                let builder = match file_stem {
                    "pixelmap" => {
                        builder
                            .raw_line("use ohos_sys_opaque_types::{napi_env, napi_value, \
                            OH_NativeBuffer, OH_PixelmapNative, OH_NativeColorSpaceManager};")

                    },
                    "picture" => {
                        builder
                            .raw_line("use ohos_sys_opaque_types::OH_PixelmapNative;")
                            .raw_line("use crate::native_image::pixelmap::PIXEL_FORMAT;")
                    }
                    "image_source" => {
                        builder
                            .raw_line("pub use ohos_sys_opaque_types::OH_ImageSourceNative;")
                            .raw_line("use ohos_sys_opaque_types::OH_PixelmapNative;")
                            .raw_line("use ohos_rawfile_sys::RawFileDescriptor;")
                            .raw_line("#[cfg(feature = \"api-13\")]")
                            .raw_line("use crate::native_image::picture::{OH_PictureNative, Image_AuxiliaryPictureType};")
                    }
                    "image_receiver" => {
                        builder.raw_line("use crate::native_image::image::OH_ImageNative;")
                    }
                    "image_packer" => {
                        builder
                            .raw_line("use ohos_sys_opaque_types::OH_PixelmapNative;")
                            .raw_line("#[cfg(feature = \"api-12\")]")
                            .raw_line("use ohos_sys_opaque_types::OH_ImageSourceNative;")
                            .raw_line("#[cfg(feature = \"api-13\")]")
                            .raw_line("use crate::native_image::picture::OH_PictureNative;")
                    }
                    "image" => {
                        builder
                            .raw_line("use ohos_sys_opaque_types::OH_NativeBuffer;")

                    }
                    _ => builder,
                };
                builder
                    .allowlist_file(format!("{}", header_path.to_str().unwrap()))
                    .allowlist_recursively(false)
                    .default_enum_style(EnumVariation::NewType {
                        is_bitfield: false,
                        is_global: false,
                        is_result_type: false,
                    })
                    .derive_copy(false)
                    .prepend_enum_name(false)
                    .clang_args(&["-x", "c++"])
            }),
        },
        DirBindingsConf {
            directory: "inputmethod".to_string(),
            output_dir: "components/inputmethod/src".to_string(),
            rename_output_file: Some(Box::new(|stem| {
                let stem = strip_suffix(stem, "_capi");
                let stem = strip_prefix(&stem, "inputmethod_");
                stem
            })),
            set_builder_opts: Box::new(|file_stem, header_path, builder| {
                let builder = if file_stem != "types" {
                    builder.raw_line("use crate::types::*;")
                } else {
                    builder.result_error_enum("InputMethod_ErrorCode")
                }
                .parse_callbacks(Box::new(ResultEnumParseCallbacks {
                    rename_item: Box::new(|enum_name| match enum_name {
                        "InputMethod_ErrorCode" => Some("InputMethodResult".to_string()),
                        _ => None,
                    }),
                    ..Default::default()
                }));
                let builder = match file_stem {
                    "text_editor_proxy" => builder
                        .raw_line("use crate::private_command::InputMethod_PrivateCommand;")
                        .raw_line("use crate::text_config::InputMethod_TextConfig;"),
                    "text_config" => builder
                        .raw_line("use crate::text_avoid_info::InputMethod_TextAvoidInfo;")
                        .raw_line("use crate::cursor_info::InputMethod_CursorInfo;"),
                    "inputmethod_proxy" => builder
                        .raw_line("use crate::private_command::InputMethod_PrivateCommand;")
                        .raw_line("use crate::cursor_info::InputMethod_CursorInfo;"),
                    "controller" => builder
                        .raw_line("use crate::inputmethod_proxy::InputMethod_InputMethodProxy;")
                        .raw_line("use crate::text_editor_proxy::InputMethod_TextEditorProxy;")
                        .raw_line("use crate::attach_options::InputMethod_AttachOptions;"),
                    _ => builder,
                };
                builder
                    .allowlist_file(format!("{}", header_path.to_str().unwrap()))
                    .allowlist_recursively(false)
                    .default_enum_style(EnumVariation::NewType {
                        is_bitfield: false,
                        is_global: false,
                        is_result_type: false,
                    })
                    .prepend_enum_name(false)
                    .clang_args(&["-x", "c++"])
            }),
        },
        DirBindingsConf {
            directory: "native_drawing".to_string(),
            output_dir: "components/drawing/src".to_string(),
            rename_output_file: Some(Box::new(|stem| {
                let stem = strip_prefix(&stem, "drawing_");
                stem
            })),
            set_builder_opts: Box::new(|file_stem, header_path, builder| {
                let builder = if file_stem != "types" {
                    let builder = builder.raw_line("use crate::types::*;");
                    if file_stem != "error_code" {
                        builder.parse_callbacks(Box::new(ResultEnumParseCallbacks {
                            rename_item: Box::new(|original_item_name| match original_item_name {
                                "OH_Drawing_ErrorCode" => {
                                    Some("crate::error_code::DrawingResult".to_string())
                                }
                                _ => None,
                            }),
                            rename_enum_variant: None,
                        }))
                    } else {
                        builder
                            .result_error_enum("OH_Drawing_ErrorCode")
                            .parse_callbacks(Box::new(ResultEnumParseCallbacks {
                                rename_item: Box::new(
                                    |original_item_name| match original_item_name {
                                        "OH_Drawing_ErrorCode" => Some("DrawingResult".to_string()),
                                        _ => None,
                                    },
                                ),
                                ..Default::default()
                            }))
                    }
                } else {
                    builder
                };
                let builder = match file_stem {
                    "font_collection" => builder.raw_line("use crate::text_declaration::*;"),
                    "text_typography" => builder
                        .raw_line("use crate::text_declaration::*;")
                        .raw_line("#[cfg(feature = \"api-12\")]")
                        .raw_line("use crate::font::OH_Drawing_Font_Metrics;"),
                    "text_font_descriptor" => {
                        builder.raw_line("use crate::text_typography::OH_Drawing_FontDescriptor;")
                    }
                    "register_font" => builder.raw_line("use crate::text_declaration::*;"),
                    "image_filter" => builder.raw_line("use crate::shader_effect::*;"),
                    "font_mgr" => builder.raw_line("use crate::text_typography::*;"),
                    "pixel_map" => builder.raw_line(
                        "use ohos_sys_opaque_types::{OH_PixelmapNative, NativePixelMap_};",
                    ),
                    "text_blob" => builder.no_copy("OH_Drawing_RunBuffer"),
                    _ => builder,
                };
                builder
                    .allowlist_file(format!("{}", header_path.to_str().unwrap()))
                    .allowlist_recursively(false)
                    .default_enum_style(EnumVariation::NewType {
                        is_bitfield: false,
                        is_global: false,
                        is_result_type: false,
                    })
                    .prepend_enum_name(false)
                    .clang_args(&["-x", "c++"])
            }),
        },
        DirBindingsConf {
            directory: "arkui".to_string(),
            output_dir: "components/arkui/src".to_string(),
            rename_output_file: None,
            set_builder_opts: Box::new(|file_stem, header_path, builder| {
                let builder = if file_stem != "native_type" {
                    builder.raw_line("use crate::native_type::*;")
                } else {
                    builder
                };
                let builder = builder
                    .allowlist_file(format!("{}", header_path.to_str().unwrap()))
                    .allowlist_recursively(false)
                    .default_enum_style(EnumVariation::NewType {
                        is_bitfield: false,
                        is_global: false,
                        is_result_type: false,
                    })
                    .prepend_enum_name(false)
                    .parse_callbacks(Box::new(ResultEnumParseCallbacks {
                        rename_item: Box::new(|original_item_name| match original_item_name {
                            "ArkUI_ErrorCode" => Some("ArkUiResult".to_string()),
                            _ => None,
                        }),
                        ..Default::default()
                    }))
                    .clang_args(&["-x", "c++"]);
                match file_stem {
                    "drag_and_drop" => {
                        builder
                            // Todo: Requires bindings to `database/udmf`
                            .blocklist_function("OH_ArkUI_DragEvent_SetData")
                            .blocklist_function("OH_ArkUI_DragEvent_GetUdmfData")
                            .blocklist_function("OH_ArkUI_DragAction_SetData")
                            // Pixelmap is from image-kit
                            .raw_line("pub use ohos_sys_opaque_types::OH_PixelmapNative;")
                    }
                    "drawable_descriptor" => {
                        builder.raw_line("pub use ohos_sys_opaque_types::OH_PixelmapNative;")
                    }
                    "native_animate" => builder
                        .no_debug("ArkUI_NativeAnimateAPI_.*")
                        .no_copy("ArkUI_NativeAnimateAPI_.*"),
                    "native_dialog" => builder
                        .no_debug("ArkUI_NativeDialogAPI_.*")
                        .no_copy("ArkUI_NativeDialogAPI_.*"),
                    "native_gesture" => builder
                        .raw_line("use crate::ui_input_event::ArkUI_UIInputEvent;")
                        .blocklist_function("^OH_ArkUI_GestureEvent_GetNode")
                        .blocklist_function("^OH_ArkUI_GestureEvent_SetNode")
                        .no_debug("ArkUI_NativeGestureAPI_1")
                        .no_copy("ArkUI_NativeGestureAPI_1"),
                    "native_interface_accessibility" => {
                        builder.raw_line("use ohos_sys_opaque_types::ArkUI_AccessibilityProvider;")
                    }
                    "native_key_event" => {
                        builder.raw_line("use crate::ui_input_event::ArkUI_UIInputEvent;")
                    }
                    "native_node" => builder
                        .blocklist_var("MAX_NODE_SCOPE_NUM")
                        .blocklist_var("MAX_COMPONENT_EVENT_ARG_NUM")
                        .raw_line("use crate::ui_input_event::ArkUI_UIInputEvent;"),
                    "native_node_napi" => builder
                        .raw_line("use ohos_sys_opaque_types::{napi_env, napi_value};")
                        .raw_line("use crate::drawable_descriptor::ArkUI_DrawableDescriptor;"),
                    "native_type" => {
                        builder
                            .raw_line("use crate::drawable_descriptor::ArkUI_DrawableDescriptor;")
                            // We want copy for the union type `ArkUI_NumberValue`
                            .derive_copy(true)
                            .no_copy("ArkUI_ContextCallback")
                            .no_copy("ARKUI_TextPickerRangeContent")
                            .no_copy("ARKUI_TextPickerCascadeRangeContent")
                            .no_copy("ArkUI_ColorStop")
                            .result_error_enum("ArkUI_ErrorCode")
                    }
                    "styled_string" => builder
                        .blocklist_function("OH_ArkUI_StyledString_Create")
                        .blocklist_function("OH_ArkUI_StyledString_PushTextStyle")
                        .blocklist_function("OH_ArkUI_StyledString_CreateTypography")
                        .blocklist_function("OH_ArkUI_StyledString_AddPlaceholder"),
                    "ui_input_event" => builder
                        .bitfield_enum("ArkUI_ModifierKeyName")
                        .blocklist_item("UI_TOUCH_EVENT_ACTION_.*")
                        .blocklist_item("UI_INPUT_EVENT_TOOL_TYPE_.*")
                        .blocklist_item("UI_INPUT_EVENT_SOURCE_TYPE_.*")
                        .blocklist_item("UI_MOUSE_EVENT_ACTION_.*")
                        .blocklist_item("UI_MOUSE_EVENT_BUTTON_.*"),
                    _ => builder,
                }
            }),
        },
        DirBindingsConf {
            directory: "rawfile".to_string(),
            output_dir: "components/rawfile/src".to_string(),
            rename_output_file: None,
            set_builder_opts: Box::new(|file_stem, header_path, builder| {
                let builder = builder
                    .allowlist_file(format!("{}", header_path.to_str().unwrap()))
                    .allowlist_recursively(false)
                    .default_enum_style(EnumVariation::NewType {
                        is_bitfield: false,
                        is_global: false,
                        is_result_type: false,
                    })
                    .prepend_enum_name(false)
                    .clang_args(&["-x", "c++"]);
                match file_stem {
                         "raw_file" => {
                             builder
                                 // Types are generated separately, since they might be shared.
                                 .blocklist_var(".*")
                                 .blocklist_type(".*")
                                 .raw_line("use crate::raw_file_types_ffi::*;")
                                 .raw_line("#[cfg(doc)]")
                                 .raw_line("use crate::raw_file_manager::{OH_ResourceManager_OpenRawFile, OH_ResourceManager_OpenRawDir};")
                                 .raw_line("#[cfg(doc)]")
                                 .raw_line("use crate::raw_file_manager::OH_ResourceManager_OpenRawFile64;")
                         },
                         "raw_dir" => {
                             builder
                                 .raw_line("#[cfg(doc)]")
                                 .raw_line("use crate::raw_file_manager::{OH_ResourceManager_OpenRawFile, OH_ResourceManager_OpenRawDir};")

                         }
                         "raw_file_manager" => {
                             builder
                                 .raw_line("use ohos_sys_opaque_types::{napi_env, napi_value};")
                                 .raw_line("use crate::raw_dir::RawDir;")
                                 .raw_line("use crate::RawFile;")
                                 .raw_line("#[cfg(doc)]")
                                 .raw_line("use crate::raw_dir::OH_ResourceManager_CloseRawDir;")
                                 .raw_line("#[cfg(doc)]")
                                 .raw_line("use crate::raw_file::{OH_ResourceManager_CloseRawFile, OH_ResourceManager_CloseRawFile64};")
                                 .raw_line("#[cfg(feature = \"api-11\")]")
                                 .raw_line("use crate::RawFile64;")

                         }
                         _ => builder,
                     }
            }),
        },
        DirBindingsConf {
            directory: "multimodalinput".to_string(),
            output_dir: "components/multimodal-input/src".to_string(),
            rename_output_file: Some(Box::new(|name| name.trim_start_matches("oh_").to_string())),
            set_builder_opts: Box::new(|file_stem, header_path, builder| {
                let builder = builder
                    .allowlist_file(format!("{}", header_path.to_str().unwrap()))
                    .allowlist_recursively(false)
                    .default_enum_style(EnumVariation::NewType {
                        is_bitfield: false,
                        is_global: false,
                        is_result_type: false,
                    })
                    .prepend_enum_name(false)
                    .clang_args(["-include", "stdbool.h"]);
                match file_stem {
                         "input_manager" => {
                             builder
                                 .result_error_enum("Input_Result")
                                 .parse_callbacks(Box::new(ResultEnumParseCallbacks::default()))
                                 .raw_line("use crate::axis_type::{InputEvent_AxisAction, InputEvent_AxisEventType, InputEvent_AxisType};")
                         },
                         "key_code" => {
                             builder
                                 // Input_KeyCode is not directly exposed via FFI, instead a
                                 // raw integer is used there. Hence, we can use a rust enum here
                                 // which is much nicer to use.
                                 .rustified_enum("Input_KeyCode")

                         }
                         _ => builder,
                     }
            }),
        },
    ]
}
