use crate::BindingConf;
use bindgen::EnumVariation;

pub(crate) fn get_bindings_config(api_version: u32) -> Vec<BindingConf> {
    vec![
        BindingConf {
            include_filename: "deviceinfo.h".to_string(),
            output_prefix: "components/deviceinfo/src/deviceinfo".to_string(),
            set_builder_opts: Box::new(|builder| {
                builder.default_enum_style(EnumVariation::NewType {
                    is_bitfield: false,
                    is_global: false,
                    is_result_type: false,
                })
            }),
        },
        BindingConf {
            include_filename: "syscap_ndk.h".to_string(),
            output_prefix: "src/syscap/syscap".to_string(),
            set_builder_opts: Box::new(|builder| {
                builder.default_enum_style(EnumVariation::NewType {
                    is_bitfield: false,
                    is_global: false,
                    is_result_type: false,
                })
            }),
        },
        BindingConf {
            include_filename: "hilog/log.h".to_string(),
            output_prefix: "components/hilog/src/hilog".to_string(),
            set_builder_opts: Box::new(|builder| {
                builder
                    .default_enum_style(EnumVariation::NewType {
                        is_bitfield: false,
                        is_global: false,
                        is_result_type: false,
                    })
                    .blocklist_var("LOG_DOMAIN")
            }),
        },
        BindingConf {
            include_filename: "napi/native_api.h".to_string(),
            output_prefix: "src/napi/napi".to_string(),
            set_builder_opts: Box::new(|builder| {
                builder
                    .default_enum_style(EnumVariation::NewType {
                        is_bitfield: false,
                        is_global: false,
                        is_result_type: false,
                    })
                    .prepend_enum_name(false)
                    .no_copy("napi_property_descriptor")
                    .no_copy("napi_extended_error_info")
                    .no_copy("napi_node_version")
                    .no_copy("napi_module")
                    .raw_line("pub use ohos_sys_opaque_types::{napi_env, napi_value};")
            }),
        },
        BindingConf {
            include_filename: "hitrace/trace.h".to_string(),
            output_prefix: "components/hitrace/src/hitrace".to_string(),
            set_builder_opts: Box::new(|builder| {
                builder
                    .default_enum_style(EnumVariation::NewType {
                        is_bitfield: false,
                        is_global: false,
                        is_result_type: false,
                    })
                    .bitfield_enum("^HiTrace_Flag$")
                    .rustified_non_exhaustive_enum("HiTrace_Tracepoint_Type")
                    .blocklist_var("LOG_DOMAIN")
                    .clang_arg("-include")
                    .clang_arg("stdbool.h")
            }),
        },
        BindingConf {
            include_filename: "ace/xcomponent/native_interface_xcomponent.h".to_string(),
            output_prefix: "components/xcomponent/src/xcomponent".to_string(),
            set_builder_opts: Box::new(|builder| {
                builder
                    .default_enum_style(EnumVariation::NewType {
                        is_bitfield: false,
                        is_global: false,
                        is_result_type: false,
                    })
                    .allowlist_file(r".*/xcomponent/native_.*xcomponent.*\.h")
                    .allowlist_recursively(false)
                    .no_copy("^OH_NativeXComponent$")
                    .no_copy("^OH_NativeXComponent_KeyEvent$")
                    .no_debug("^OH_NativeXComponent$")
                    .no_debug("^OH_NativeXComponent_KeyEvent$")
                    .blocklist_item("ArkUI_.*")
                    // Requires a patch to bindgen
                    .blocklist_item("OH_NATIVEXCOMPONENT_RESULT.*")
                    .blocklist_function("OH_NativeXComponent_.*NativeRootNode")
                    .blocklist_function("OH_NativeXComponent_RegisterUIInputEventCallback")
                    .blocklist_function("OH_NativeXComponent_RegisterOnTouchInterceptCallback")
                    .blocklist_function("OH_NativeXComponent_GetNativeXComponent")
                    .raw_line("#[cfg(feature = \"api-13\")]")
                    .raw_line("use ohos_sys_opaque_types::ArkUI_AccessibilityProvider;")
                    .clang_args(&["-x", "c++"])
            }),
        },
        BindingConf {
            include_filename: "ace/xcomponent/native_interface_xcomponent.h".to_string(),
            output_prefix: "components/xcomponent/src/xcomponent_result".to_string(),
            set_builder_opts: Box::new(|builder| {
                builder
                    .raw_line("#![allow(unused)]")
                    .allowlist_var("OH_NATIVEXCOMPONENT_RESULT_.*")
                    .clang_args(&["-x", "c++"])
            }),
        },
        BindingConf {
            include_filename: "ace/xcomponent/native_interface_xcomponent.h".to_string(),
            output_prefix: "components/xcomponent/src/xcomponent_arkui".to_string(),
            set_builder_opts: Box::new(|builder| {
                builder
                    .raw_line("use super::xcomponent_ffi::*;")
                    .raw_line("use arkui_sys::native_type::ArkUI_NodeHandle;")
                    .raw_line("use arkui_sys::ui_input_event::{ArkUI_UIInputEvent, ArkUI_UIInputEvent_Type, HitTestMode};")
                    .allowlist_function("OH_NativeXComponent_.*NativeRootNode")
                    .allowlist_function("OH_NativeXComponent_RegisterUIInputEventCallback")
                    .allowlist_function("OH_NativeXComponent_RegisterOnTouchInterceptCallback")
                    .blocklist_type(".*")
                    .clang_args(&["-x", "c++"])
            }),
        },
        BindingConf {
            include_filename: "native_vsync/native_vsync.h".to_string(),
            output_prefix: "components/vsync/src/vsync".to_string(),
            set_builder_opts: Box::new(move |builder| {
                builder
                    .default_enum_style(EnumVariation::NewType {
                        is_bitfield: false,
                        is_global: false,
                        is_result_type: false,
                    })
                    .clang_args(["-include", "stdbool.h", "-include", "stdint.h"])
            }),
        },
        BindingConf {
            include_filename: "arkui/ui_input_event.h".to_string(),
            output_prefix: "components/arkui/src/ui_input_event/ui_input_event_anon_enums"
                .to_string(),
            set_builder_opts: Box::new(move |builder| {
                builder
                    .default_enum_style(EnumVariation::NewType {
                        is_bitfield: false,
                        is_global: false,
                        is_result_type: false,
                    })
                    .allowlist_var("UI_TOUCH_EVENT_ACTION_.*")
                    .allowlist_var("UI_INPUT_EVENT_TOOL_TYPE_.*")
                    .allowlist_var("UI_INPUT_EVENT_SOURCE_TYPE_.*")
                    .allowlist_var("UI_MOUSE_EVENT_ACTION_.*")
                    .allowlist_var("UI_MOUSE_EVENT_BUTTON_*")
                    .allowlist_recursively(true)
                    .clang_args(["-include", "stdbool.h"])
            }),
        },
        BindingConf {
            include_filename: "arkui/styled_string.h".to_string(),
            output_prefix: "components/arkui/src/styled_string/drawing".to_string(),
            set_builder_opts: Box::new(|builder| {
                builder
                    .allowlist_function("OH_ArkUI_StyledString_Create")
                    .allowlist_function("OH_ArkUI_StyledString_PushTextStyle")
                    .allowlist_function("OH_ArkUI_StyledString_CreateTypography")
                    .allowlist_function("OH_ArkUI_StyledString_AddPlaceholder")
                    .clang_args(["-x", "c++"])
                    .allowlist_recursively(false)
                    .raw_line("pub use super::ArkUI_StyledString;")
                    .raw_line("pub use ohos_drawing_sys::text_typography::OH_Drawing_PlaceholderSpan;")
                    .raw_line("pub use ohos_drawing_sys::text_declaration::{OH_Drawing_FontCollection, OH_Drawing_TextStyle, OH_Drawing_Typography, OH_Drawing_TypographyStyle};")
            }),
        },
        BindingConf {
            include_filename: "rawfile/raw_file.h".to_string(),
            output_prefix: "components/rawfile/src/raw_file_types".to_string(),
            set_builder_opts: Box::new(|builder| {
                builder
                    .ignore_functions()
                    .clang_args(["-x", "c++"])
                    .blocklist_var("_LIBCPP.*")
                    .raw_line("#[cfg(doc)]")
                    .raw_line("use crate::{raw_file::{OH_ResourceManager_GetRawFileDescriptor,OH_ResourceManager_GetRawFileDescriptor64},\
                        raw_file_manager::OH_ResourceManager_OpenRawFile};")
            }),
        },
    ]
}
