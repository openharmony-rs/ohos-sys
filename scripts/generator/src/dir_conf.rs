//! Configuration file for OpenHarmony modules with multiple header files in a directory.
//!
//! Add new bindings to `get_module_bindings_config()` by appending a new `DirBindingsConf`.

use bindgen::EnumVariation;
use crate::DirBindingsConf;

/// Convenience method for stripping am optional suffix and returning an owned String
fn strip_suffix(input: &str, suffix: &str) -> String {
    match input.strip_suffix(suffix) {
        None => input.to_string(),
        Some(stripped) => stripped.to_string()
    }
}

/// Convenience method for stripping am optional suffix and returning an owned String
fn strip_prefix(input: &str, prefix: &str) -> String {
    match input.strip_prefix(prefix) {
        None => input.to_string(),
        Some(stripped) => stripped.to_string()
    }
}

pub(crate) fn get_module_bindings_config(api_version: u32) -> Vec<DirBindingsConf> {
    vec![DirBindingsConf {
        directory: "multimedia/image_framework/image".to_string(),
        output_dir: "components/multimedia/image_framework/src/native_image".to_string(),
        min_api_version: 12,
        rename_output_file: Some(Box::new(|stem| strip_suffix(stem, "_native"))),
        set_builder_opts: Box::new(
            |file_stem, header_path, builder| {
                let builder = if file_stem != "image_common" {
                    builder.raw_line("use crate::native_image::common::*;")
                } else {
                    builder
                };
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
                    })
                    .derive_copy(false)
                    .derive_debug(false)
                    .prepend_enum_name(false)
                    .clang_args(&["-x", "c++"])
            }
        ),
    }, DirBindingsConf {
        directory: "inputmethod".to_string(),
        output_dir: "components/inputmethod/src".to_string(),
        min_api_version: 12,
        rename_output_file: Some(Box::new(|stem| {
            let stem = strip_suffix(stem, "_capi");
            let stem = strip_prefix(&stem, "inputmethod_");
            stem
        })),
        set_builder_opts: Box::new(
            |file_stem, header_path, builder| {
                let builder = if file_stem != "types" {
                    builder.raw_line("use crate::types::*;")
                } else {
                    builder
                };
                let builder = match file_stem {
                    "text_editor_proxy" => {
                        builder
                            .raw_line("use crate::private_command::InputMethod_PrivateCommand;")
                            .raw_line("use crate::text_config::InputMethod_TextConfig;")
                    },
                    "text_config" => {
                        builder
                            .raw_line("use crate::text_avoid_info::InputMethod_TextAvoidInfo;")
                            .raw_line("use crate::cursor_info::InputMethod_CursorInfo;")
                    }
                    "inputmethod_proxy" => {
                        builder
                            .raw_line("use crate::private_command::InputMethod_PrivateCommand;")
                            .raw_line("use crate::cursor_info::InputMethod_CursorInfo;")
                    }
                    "controller" => {
                        builder
                            .raw_line("use crate::inputmethod_proxy::InputMethod_InputMethodProxy;")
                            .raw_line("use crate::text_editor_proxy::InputMethod_TextEditorProxy;")
                            .raw_line("use crate::attach_options::InputMethod_AttachOptions;")
                    }
                    _ => builder,
                };
                builder
                    .allowlist_file(format!("{}", header_path.to_str().unwrap()))
                    .allowlist_recursively(false)
                    .default_enum_style(EnumVariation::NewType {
                        is_bitfield: false,
                        is_global: false,
                    })
                    .derive_copy(false)
                    .derive_debug(false)
                    .prepend_enum_name(false)
                    .clang_args(&["-x", "c++"])
            }
        ),
    },
         DirBindingsConf {
             directory: "native_drawing".to_string(),
             output_dir: "components/drawing/src".to_string(),
             min_api_version: 12,
             rename_output_file: Some(Box::new(|stem| {
                 let stem = strip_prefix(&stem, "drawing_");
                 stem
             })),
             set_builder_opts: Box::new(
                 |file_stem, header_path, builder| {
                     let builder = if file_stem != "types" {
                         let builder = builder.raw_line("use crate::types::*;");
                         if file_stem != "error_code" {
                             builder
                                 .raw_line("")
                                 .raw_line("#[allow(unused_imports)]")
                                 .raw_line("#[cfg(feature = \"api-12\")]")
                                 .raw_line("use crate::error_code::OH_Drawing_ErrorCode;")
                         } else {
                             builder
                         }
                     } else {
                         builder
                     };
                     let builder = match file_stem {
                         "font_collection" => {
                             builder
                                 .raw_line("use crate::text_declaration::*;")
                         },
                         "text_typography" => {
                             builder
                                 .raw_line("use crate::text_declaration::*;")
                                 .raw_line("#[cfg(feature = \"api-12\")]")
                                 .raw_line("use crate::font::OH_Drawing_Font_Metrics;")
                                 // FIXME: This needs to be guarded behind API-level-12 (fixed in SDK-13)
                                 // We blocklist for now and remove when updating to SDK-13
                                 .blocklist_function("OH_Drawing_TypographyGetLineFontMetrics")
                         }
                         "register_font" => {
                             builder
                                 .raw_line("use crate::text_declaration::*;")
                         }
                         "image_filter" => {
                             builder
                                 .raw_line("use crate::shader_effect::*;")
                         },
                         "font_mgr" => {
                             builder.raw_line("use crate::text_typography::*;")
                         },
                         "pixel_map" => {
                             builder.raw_line("use ohos_sys_opaque_types::{OH_PixelmapNative, NativePixelMap_};")
                         }
                         "text_blob" => {
                             builder
                                 .no_copy("OH_Drawing_RunBuffer")
                         }
                         _ => builder,
                     };
                     builder
                         .allowlist_file(format!("{}", header_path.to_str().unwrap()))
                         .allowlist_recursively(false)
                         .default_enum_style(EnumVariation::NewType {
                             is_bitfield: false,
                             is_global: false,
                         })
                         .prepend_enum_name(false)
                         .clang_args(&["-x", "c++"])

                 }
             ),
         },
         DirBindingsConf {
             directory: "arkui".to_string(),
             output_dir: "components/arkui/src".to_string(),
             min_api_version: 12,
             rename_output_file: None,
             set_builder_opts: Box::new(
                 |file_stem, header_path, builder| {
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
                         })
                         .derive_copy(false)
                         .derive_debug(false)
                         .prepend_enum_name(false)
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
                             builder
                                 .raw_line("pub use ohos_sys_opaque_types::OH_PixelmapNative;")

                         },
                         "native_gesture" => {
                             builder
                                 .raw_line("use crate::ui_input_event::ArkUI_UIInputEvent;")
                                 .blocklist_function("^OH_ArkUI_GestureEvent_GetNode")
                         },
                         "native_interface_accessibility" => {
                             builder
                                 .raw_line("use ohos_sys_opaque_types::ArkUI_AccessibilityProvider;")
                         },
                         "native_node" => {
                             builder
                                 .blocklist_var("MAX_NODE_SCOPE_NUM")
                                 .blocklist_var("MAX_COMPONENT_EVENT_ARG_NUM")
                                 .raw_line("use crate::ui_input_event::ArkUI_UIInputEvent;")
                         },
                         "native_node_napi" => {
                             builder
                                 .raw_line("use ohos_sys_opaque_types::{napi_env, napi_value};")
                                 .raw_line("use crate::drawable_descriptor::ArkUI_DrawableDescriptor;")

                         },
                         "native_type" => {
                             builder
                                 .raw_line("use crate::drawable_descriptor::ArkUI_DrawableDescriptor;")
                                 // We want copy for the union type `ArkUI_NumberValue`
                                 .derive_copy(true)
                                 .no_copy("ArkUI_ContextCallback")
                                 .no_copy("ARKUI_TextPickerRangeContent")
                                 .no_copy("ARKUI_TextPickerCascadeRangeContent")
                                 .no_copy("ArkUI_ColorStop")

                         },
                         "styled_string" => {
                             builder
                                 .blocklist_function("OH_ArkUI_StyledString_Create")
                                 .blocklist_function("OH_ArkUI_StyledString_PushTextStyle")
                                 .blocklist_function("OH_ArkUI_StyledString_CreateTypography")
                                 .blocklist_function("OH_ArkUI_StyledString_AddPlaceholder")
                         }
                         "ui_input_event" => {
                             builder
                                 .bitfield_enum("ArkUI_ModifierKeyName")
                                 .blocklist_item("UI_TOUCH_EVENT_ACTION_.*")
                                 .blocklist_item("UI_INPUT_EVENT_TOOL_TYPE_.*")
                                 .blocklist_item("UI_INPUT_EVENT_SOURCE_TYPE_.*")
                                 .blocklist_item("UI_MOUSE_EVENT_ACTION_.*")
                                 .blocklist_item("UI_MOUSE_EVENT_BUTTON_.*")

                         },
                         _ => builder,
                     }
                 }
             ),
         },

         DirBindingsConf {
             directory: "rawfile".to_string(),
             output_dir: "components/rawfile/src".to_string(),
             min_api_version: 10,
             rename_output_file: None,
             set_builder_opts: Box::new(
                 |file_stem, header_path, builder| {
                     let builder = builder
                         .allowlist_file(format!("{}", header_path.to_str().unwrap()))
                         .allowlist_recursively(false)
                         .default_enum_style(EnumVariation::NewType {
                             is_bitfield: false,
                             is_global: false,
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
                 }
             ),
         },
         DirBindingsConf {
             directory: "multimodalinput".to_string(),
             output_dir: "components/multimodal-input/src".to_string(),
             min_api_version: 12,
             rename_output_file: Some(Box::new(|name| name.trim_start_matches("oh_").to_string())),
             set_builder_opts: Box::new(
                 |file_stem, header_path, builder| {
                     let builder = builder
                         .allowlist_file(format!("{}", header_path.to_str().unwrap()))
                         .allowlist_recursively(false)
                         .default_enum_style(EnumVariation::NewType {
                             is_bitfield: false,
                             is_global: false,
                         })
                         .prepend_enum_name(false);
                     //.clang_args(&["-x", "c++"]);
                     match file_stem {
                         "input_manager" => {
                             builder
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
                 }
             ),
         },
    ]
}
