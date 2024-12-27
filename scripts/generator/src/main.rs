use std::fs;
use std::num::ParseIntError;
use anyhow::{anyhow, bail, Context};
use bindgen::{CodeGenAttributes, EnumVariation, Formatter};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use thiserror::Error;

/// Parse the api version
///
/// Manually parse the API version to avoid pulling in any heavy dependencies.
fn parse_api_version(sdk_native_dir: &Path) -> anyhow::Result<u32> {
    let metadata = std::fs::read_to_string(&sdk_native_dir.join("oh-uni-package.json"))
        .context("Failed to read oh-uni-package.json")?;
    let api_version = metadata
        .lines()
        .find(|line| line.contains("apiVersion"))
        .context("No apiVersion in oh-uni-package.json")?
        // Expected: `  "apiVersion": "12",`
        .split_once(":")
        .context("Invalid json?")?
        .1
        .trim()
        .trim_end_matches(",")
        .trim_matches('"')
        .parse::<u32>()
        .context("apiVersion is not an integer")?;
    Ok(api_version)
}

#[derive(Copy, Clone, PartialEq, Ord, PartialOrd, Eq)]
enum OpenHarmonyApiLevel {
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Eleven = 11,
    Twelve = 12,
    Thirteen = 13,
    Fourteen = 14,
}

#[derive(Error, Debug)]
enum ApiLevelParseError {
    #[error("Could not parse API level from interger: {0:?}")]
    ParseIntError(#[from] ParseIntError),
    #[error("Unknown API level {0}! Perhaps we need an update")]
    UnknownApiVersion(u32),
}

impl TryFrom<&str> for OpenHarmonyApiLevel {
    type Error = ApiLevelParseError;

    fn try_from(api_level: &str) -> Result<Self, ApiLevelParseError> {
        let num: u32 = api_level.parse()?;
        let level = match num {
            8 => OpenHarmonyApiLevel::Eight,
            9 => OpenHarmonyApiLevel::Nine,
            10 => OpenHarmonyApiLevel::Ten,
            11 => OpenHarmonyApiLevel::Eleven,
            12 => OpenHarmonyApiLevel::Twelve,
            13 => OpenHarmonyApiLevel::Thirteen,
            14 => OpenHarmonyApiLevel::Fourteen,
            other => { return Err(ApiLevelParseError::UnknownApiVersion(other)); }
        };
        Ok(level)
    }
}

#[derive(Error, Debug)]
enum ParseDeprecatedError {
    #[error("Could not parse API level: {0:?}")]
    ApiLevelParseError(#[from] ApiLevelParseError),
    #[error("Failed to find @deprecated in line: {0}")]
    InvalidLine(String),
}

#[derive(Debug)]
struct DoxygenCommentCb;

fn parse_deprecated_since(line: &str) -> Result<Option<OpenHarmonyApiLevel>, ParseDeprecatedError> {
    if line.trim() == "@deprecated" {
        return Ok(None);
    }
    let (_, rhs) = line.split_once("@deprecated").ok_or_else(|| ParseDeprecatedError::InvalidLine(line.to_string()))?;
    // Variant 1: `@deprecated(since = "XX")`
    // Note: Regex parsing might be more readable, but we want to avoid pulling in more dependencies.
    if let Some(api_level_str) = rhs.split_once("(since = \"")
        .or_else(|| rhs.split_once("(since=\""))
        .map(|(_, rhs)| { rhs.split_once("\"").expect("String end delimiter not found").0 }) {
        return Ok(Some(OpenHarmonyApiLevel::try_from(api_level_str)?));
    }

    if let Some((_, api_level_str)) = rhs.split_once("since ") {
        Ok(Some(OpenHarmonyApiLevel::try_from(api_level_str.trim())?))
    } else {
        Err(ParseDeprecatedError::InvalidLine(line.to_string()))
    }
}

impl bindgen::callbacks::ParseCallbacks for DoxygenCommentCb {
    fn parse_comments_for_attributes(&self, comment: &str) -> Vec<CodeGenAttributes> {
        let mut attributes: Vec<CodeGenAttributes> = vec![];
        let api_version = comment.lines()
            .find_map(|line| line.split_once("@since"))
            .map(|(_, since)| {
                let api_level_str = since
                    .trim();
                let api_level: Result<OpenHarmonyApiLevel, _> =
                    api_level_str.try_into()
                    .inspect_err(|err| eprintln!("Failed to parse OH API version: {:?}", err));
                api_level.expect("Failed to parse OH API version")
            }
            );
        if let Some(api_version) = api_version {
            let cfg = format!("feature = \"api-{}\"", api_version as u32);
            // Our Minimum api-level is 10, so we don't feature guard things <= API level 10.
            if api_version > OpenHarmonyApiLevel::Ten {
                attributes.push(CodeGenAttributes::Cfg(cfg));
                attributes.push(CodeGenAttributes::CfgAttr(format!("docsrs, doc(cfg(feature = \"api-{}\"))", api_version as u32)));
            }
        }

        if let Some(deprecated_line) = comment.lines().find(|line| line.contains("@deprecated")) {
            let deprecated_since = parse_deprecated_since(deprecated_line).expect("Parse failed");
            let deprecated_opt =  deprecated_since.map(|since | {
                    format!("since = \"{}\"", since as u32)
            });
            // if let Some(since ) = deprecated_since {
            //     if since <= OpenHarmonyApiLevel::Ten {
            //         // We can't tell bindgen to not generate something directly, but we can add a
            //         // a `cfg` which will never be enabled.
            //         // Todo: This should be revisited once we support checking type, e.g. for
            //         // enum variants, we should not cfg them out.
            //         attributes.push(CodeGenAttributes::Cfg("ohos_sys_deprecated_removed".to_string()));
            //     }
            // }
            attributes.push(CodeGenAttributes::Deprecated(deprecated_opt));
        }

        attributes
    }
}

fn base_bindgen_builder(sysroot_dir: &Path) -> anyhow::Result<bindgen::Builder> {
    let builder = bindgen::builder()
        .use_core()
        .layout_tests(false)
        .formatter(Formatter::Prettyplease)
        .merge_extern_blocks(true)
        .rust_target(bindgen::RustTarget::from_str("1.78").expect("invalid rust target"))
        .blocklist_file(r".*stdint\.h")
        .blocklist_file(r".*stddef\.h")
        .blocklist_file(r".*stdarg\.h")
        .blocklist_file(r".*stdbool\.h")
        .blocklist_file(r".*stdbool\.h")
        .blocklist_file(r".*/std[a-z]{3,4}\.h")
        .blocklist_file(r".*/__std[a-z_]+\.h")
        .blocklist_item("__(BYTE_ORDER|LONG_MAX|LITTLE_ENDIAN|BIG_ENDIAN|USE_TIME_BITS64)")
        .blocklist_item("u?intmax_t")
        .raw_line("#![allow(non_upper_case_globals)]")
        .raw_line("#![allow(non_camel_case_types)]")
        .raw_line("#![allow(non_snake_case)]")
        .parse_callbacks(Box::new(DoxygenCommentCb))
        .clang_arg(format!("--sysroot={}",
                           sysroot_dir.to_str().context("The OpenHarmony SDK directory must be encodable as utf-8")?)
        )
        // TODO: How to detect / deal with target specific bindings - Could this be a problem?
        .clang_arg("--target=aarch64-linux-ohos")
        // .generate_cstr(true)
        // dynamic_library_name()
        // .prepend_enum_name()
        ;

    Ok(builder)
}

struct DirBindingsConf {
    /// Directory the bindings should be generated for relative to the sysroot
    directory: String,
    /// Directory the bindings should be written to.
    output_dir: String,
    /// API version that this module was added in.
    min_api_version: u32,
    /// Optionally transform the output file name stem
    rename_output_file: Option<Box<dyn Fn(&str) -> String>>,
    /// Options which apply to all or most files
    set_builder_opts: Box<dyn Fn(&str, &Path, bindgen::Builder) -> bindgen::Builder>,
}

struct BindingConf {
    include_filename: String,
    /// Will be interpolated as "${ROOT_DIR}/${output_prefix}_api${API_VERSION}"
    output_prefix: String,
    set_builder_opts: Box<dyn FnOnce(bindgen::Builder) -> bindgen::Builder>,
}

fn get_bindings_config(api_version: u32) -> Vec<BindingConf> {
    vec![
        BindingConf {
            include_filename: "deviceinfo.h".to_string(),
            output_prefix: "components/deviceinfo/src/deviceinfo".to_string(),
            set_builder_opts: Box::new(|builder| {
                builder.default_enum_style(EnumVariation::NewType {
                    is_bitfield: false,
                    is_global: false,
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
                    })
                    .blocklist_var("LOG_DOMAIN")
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
                    })
                    .allowlist_file(r".*/xcomponent/native_.*xcomponent.*\.h")
                    .allowlist_recursively(false)
                    .no_copy("^OH_NativeXComponent$")
                    .no_copy("^OH_NativeXComponent_KeyEvent$")
                    .no_debug("^OH_NativeXComponent$")
                    .no_debug("^OH_NativeXComponent_KeyEvent$")
                    .blocklist_item("ArkUI_.*")
                    // FIXME: this doesn't work - needs to be fixed in bindgen (anonymous enum variant)
                    .blocklist_item("OH_NATIVEXCOMPONENT_RESULT.*")
                    // Note: this needs to be updated semi-regularly ....
                    .blocklist_type("_bindgen_ty_11")
                    .blocklist_function("OH_NativeXComponent_.*NativeRootNode")
                    .blocklist_function("OH_NativeXComponent_RegisterUIInputEventCallback")
                    .blocklist_function("OH_NativeXComponent_RegisterOnTouchInterceptCallback")
                    .blocklist_function("OH_NativeXComponent_GetNativeXComponent")
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
            include_filename: "native_buffer/native_buffer.h".to_string(),
            output_prefix: "src/native_buffer/native_buffer".to_string(),
            set_builder_opts: Box::new(|builder| {
                builder
                    .default_enum_style(EnumVariation::NewType {
                        is_bitfield: false,
                        is_global: false,
                    })
                    .allowlist_file(r".*/native_buffer/.*\.h")
                    .bitfield_enum("OH_NativeBuffer_Usage")
                    .blocklist_item("^(OH)?NativeWindow(Buffer)?")
                    .blocklist_function("OH_NativeBuffer_.*NativeWindow.*")
                    .no_copy("^OH_NativeBuffer$")
                    .no_copy("^OHIPCParcel$")
                    .no_debug("^OH_NativeBuffer$")
            }),
        },
        BindingConf {
            include_filename: "native_buffer/native_buffer.h".to_string(),
            output_prefix: "src/native_buffer/native_buffer_window".to_string(),
            set_builder_opts: Box::new(|builder| {
                builder
                    .default_enum_style(EnumVariation::NewType {
                        is_bitfield: false,
                        is_global: false,
                    })
                    .raw_line("use super::native_buffer_ffi::*;")
                    .raw_line("use crate::native_window::OHNativeWindowBuffer;")
                    .blocklist_type(".*")
                    .allowlist_function("OH_NativeBuffer_.*NativeWindow.*")
            }),
        },
        BindingConf {
            include_filename: "native_image/native_image.h".to_string(),
            output_prefix: "src/native_image/native_image".to_string(),
            set_builder_opts: Box::new(|builder| {
                builder
                    .default_enum_style(EnumVariation::NewType {
                        is_bitfield: false,
                        is_global: false,
                    })
                    .allowlist_file(r".*/native_image/.*\.h")
                    .blocklist_item("^(OH)?NativeWindow(Buffer)?")
                    // Blocklist everything with native window, so we can feature guard it.
                    .blocklist_function("OH_NativeImage_.*NativeWindow.*")
                    .no_copy("^OH_NativeImage$")
                    .no_copy("^OH_OnFrameAvailableListener")
                    .no_debug("^OH_NativeImage$")
            }),
        },
        BindingConf {
            include_filename: "native_image/native_image.h".to_string(),
            output_prefix: "src/native_image/native_image_window".to_string(),
            set_builder_opts: Box::new(|builder| {
                builder
                    .default_enum_style(EnumVariation::NewType {
                        is_bitfield: false,
                        is_global: false,
                    })
                    .raw_line("use super::native_image_ffi::*;")
                    .raw_line("use crate::native_window::OHNativeWindow;")
                    .raw_line("use crate::native_window::OHNativeWindowBuffer;")
                    .allowlist_recursively(false)
                    .allowlist_function(".*NativeWindow.*")
            }),
        },
        BindingConf {
            include_filename: "native_window/external_window.h".to_string(),
            output_prefix: "src/native_window/native_window".to_string(),
            set_builder_opts: Box::new(|builder| {
                builder
                    .default_enum_style(EnumVariation::NewType {
                        is_bitfield: false,
                        is_global: false,
                    })
                    .constified_enum_module("^NativeWindowOperation$")
                    .derive_copy(false)
            }),
        },
        BindingConf {
            include_filename: "native_vsync/native_vsync.h".to_string(),
            output_prefix: "src/vsync/vsync".to_string(),
            set_builder_opts: Box::new(move |builder| {
                let builder = builder
                    .default_enum_style(EnumVariation::NewType {
                        is_bitfield: false,
                        is_global: false,
                    })
                    .derive_copy(false)
                    .derive_debug(false);
                if api_version == 10 {
                    builder.blocklist_item("^NativeWindowOperation$")
                } else {
                    builder
                }
            }),
        },
        BindingConf {
            include_filename: "arkui/ui_input_event.h".to_string(),
            output_prefix: "components/arkui/src/ui_input_event/ui_input_event_anon_enums".to_string(),
            set_builder_opts: Box::new(move |builder| {
               builder
                    .default_enum_style(EnumVariation::NewType {
                        is_bitfield: false,
                        is_global: false,
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
    ]
}

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

fn apply_drawing_nocopy(builder: bindgen::Builder) -> bindgen::Builder {
    let blocklist = [
        "OH_Drawing_Canvas",
        "OH_Drawing_Pen",
        "OH_Drawing_Brush",
        "OH_Drawing_Path",
        "OH_Drawing_Bitmap",
        "OH_Drawing_FontCollection",
        "OH_Drawing_Typography",
        "OH_Drawing_TextStyle",
        "OH_Drawing_TypographyStyle",
        "OH_Drawing_TypographyCreate",
        //--- API-11 -------------------------------------
        "OH_Drawing_Point",
        "OH_Drawing_Rect",
        "OH_Drawing_RoundRect",
        "OH_Drawing_ShaderEffect",
        "OH_Drawing_Filter",
        "OH_Drawing_MaskFilter",
        "OH_Drawing_ColorFilter",
        "OH_Drawing_Font",
        "OH_Drawing_Typeface",
        "OH_Drawing_TextBlob",
        "OH_Drawing_TextBlobBuilder",
        "OH_Drawing_TextBox",
        "OH_Drawing_PositionAndAffinity",
        "OH_Drawing_Range",
        "OH_Drawing_Matrix",
        "OH_Drawing_RunBuffer",
        // ---- API-12 ---------------------------------------
        "OH_Drawing_Region",
        "OH_Drawing_PixelMap",
        "OH_Drawing_ColorSpace",
        "OH_Drawing_Point2D",
        "OH_Drawing_Point3D",
        "OH_Drawing_PathEffect",
        "OH_Drawing_ShadowLayer",
        "OH_Drawing_MemoryStream",
        "OH_Drawing_Image",
        "OH_Drawing_ImageFilter",
        "OH_Drawing_SamplingOptions",
        "OH_Drawing_GpuContext",
        "OH_Drawing_Surface",
        "OH_Drawing_FontMgr",
        "OH_Drawing_FontStyleSet",
        "OH_Drawing_BitmapFormat",
        "OH_Drawing_FontParser",
        "OH_Drawing_TextShadow",
        // Maybe: OH_Drawing_StrutStyle
    ];
    let mut builder = builder;
    for obj in blocklist {
        builder = builder.no_copy(obj);
        builder = builder.no_debug(obj);
    }
    builder
}

// todo: unify via trait
fn get_module_bindings_config(api_version: u32) -> Vec<DirBindingsConf> {
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
                        // FIXME: Add necessary feature guards for `napi` and remove the blocklist
                        builder.blocklist_function("^OH_PixelmapNative_ConvertPixelmapNative(To|From)Napi")
                    },
                    "image_source" => {
                        builder
                            // FIXME: Add necessary feature guards for `rawfile` and remove the blocklist
                            .blocklist_function("^OH_ImageSourceNative_CreateFromRawFile")
                            // Todo: This function is hand-picked
                            .blocklist_function("^OH_ImageSourceNative_CreatePixelmap(List)?")
                    }
                    // Todo: these bindings are hand-picked and feature guarded right now - autogenerate...
                    "image_receiver" => {
                        builder.raw_line("use crate::native_image::image::OH_ImageNative;")
                    }
                    "image_packer" => {
                        builder
                            .blocklist_function("^OH_ImagePackerNative_PackTo(Data|File)FromImageSource")
                            .blocklist_function("^OH_ImagePackerNative_PackTo(Data|File)FromPixelmap")
                    }
                    "image" => {
                        builder.blocklist_function("^OH_ImageNative_GetByteBuffer")
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
                         }
                         _ => builder,
                     };
                     let builder = builder
                         .allowlist_file(format!("{}", header_path.to_str().unwrap()))
                         .allowlist_recursively(false)
                         .default_enum_style(EnumVariation::NewType {
                             is_bitfield: false,
                             is_global: false,
                         })
                         .prepend_enum_name(false)
                         .clang_args(&["-x", "c++"]);

                     apply_drawing_nocopy(builder)
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
                         "drawable_descriptor" => {
                             builder.blocklist_item("^OH_PixelmapNative$")
                         },
                         "native_type" => {
                             builder //.raw_line("use crate::drawable_descriptor::ArkUI_DrawableDescriptor;")
                              .blocklist_function("^OH_ArkUI_ImageAnimatorFrameInfo_CreateFromDrawableDescriptor$")
                             // We want copy for the union type `ArkUI_NumberValue`
                             .derive_copy(true)
                                 .no_copy("ArkUI_ContextCallback")
                                 .no_copy("ARKUI_TextPickerRangeContent")
                                 .no_copy("ARKUI_TextPickerCascadeRangeContent")
                                 .no_copy("ArkUI_ColorStop")

                         },
                         "native_gesture" => {
                             builder
                                 .raw_line("use crate::ui_input_event::ArkUI_UIInputEvent;")
                                 .blocklist_function("^OH_ArkUI_GestureEvent_GetNode")
                         },
                         "ui_input_event" => {
                             builder
                                 .blocklist_var("^UI_TOUCH_EVENT_ACTION_*")
                                 .bitfield_enum("ArkUI_ModifierKeyName")

                         },
                         _ => builder,
                     }
                 }
             ),
         },
    ]
}

fn generate_bindings(sdk_native_dir: &Path, api_version: u32) -> anyhow::Result<()> {
    let base_builder = base_bindgen_builder(&sdk_native_dir.join("sysroot"))?;
    let sysroot_include_dir = sdk_native_dir.join("sysroot/usr/include");
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    let root_dir = manifest_dir
        .parent()
        .and_then(|d| d.parent())
        .ok_or(anyhow!("Could not determine root directory"))?
        .canonicalize()
        .context("Could not canonicalize root directory")?;

    for binding in get_bindings_config(api_version) {
        println!("Generating binding: {}", binding.include_filename);
        let header_filename = sysroot_include_dir.join(binding.include_filename);
        let header_filename_str = header_filename.to_str().context("Unicode")?;
        let builder = base_builder
            .clone()
            .header(header_filename_str);
        let builder = (binding.set_builder_opts)(builder);
        let bindings = builder.generate().context("Bindgen failed")?;

        bindings
            .write_to_file(root_dir.join(format!("{}_ffi.rs", binding.output_prefix)))
            .context("Failed to write bindings to file")?;
    }

    for binding in &get_module_bindings_config(api_version) {
        if binding.min_api_version > api_version {
            continue
        }
        println!("Generating binding: {}", binding.directory);
        let module_dir = sysroot_include_dir.join(&binding.directory);
        if !module_dir.exists() {
            bail!("Could not find directory {} at {}", binding.directory, module_dir.display());
        }
        let paths = fs::read_dir(module_dir)?;
        for file in paths {
            let file = file.context("Failed to enumerate dir")?;
            if file.file_type()?.is_dir() {
                bail!("Subdirectories are not supported yet by this script");
            }
            let file_path = file.path().canonicalize().context("Failed to canonicalize path")?;
            let header_filename_str = file_path.to_str().context("Unicode")?;
            let file_stem = file_path.file_stem().ok_or(anyhow!("Failed to get filestem"))?.to_str().context("Unicode")?;
            let file_stem = binding.rename_output_file.as_ref().map(|rename| rename(file_stem)).unwrap_or(file_stem.to_string());
            let builder = base_builder
                .clone()
                .header(header_filename_str)
                .allowlist_file(header_filename_str);
            let builder = (binding.set_builder_opts)(&file_stem, file_path.as_path(), builder);


            let bindings = builder.generate().context("Bindgen failed")?;
            let base_path = root_dir.join(&binding.output_dir).join(&file_stem);
            if !base_path.exists() {
                eprintln!("Creating target directory for bindings: {}", base_path.display());
                fs::create_dir_all(&base_path).context("Failed to create target directory for bindings")?;
            }

            bindings
                .write_to_file(base_path.join(format!("{file_stem}_ffi.rs")))
                .context("Failed to write bindings to file")?;
        }
    }

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let sdk_native_dir = std::env::var_os("OHOS_SDK_NATIVE").context(
        "Please set environment variable OHOS_SDK_NATIVE to the `native` \
        directory of the OpenHarmony SDK",
    )?;
    let sdk_native_dir = PathBuf::from(sdk_native_dir);
    if !sdk_native_dir.is_dir() {
        return Err(anyhow!(
            "OHOS_SDK_NATIVE: {:?} is not a directory",
            sdk_native_dir
        ));
    }

    let libclang_path = sdk_native_dir.join("llvm/lib");
    let clang_path = sdk_native_dir.join("llvm/bin/clang");
    let api_version = parse_api_version(&sdk_native_dir)?;
    println!(
        "Libclang: {}, clang: {}",
        libclang_path.display(),
        clang_path.display()
    );
    std::env::set_var("LIBCLANG_PATH", libclang_path);
    std::env::set_var("CLANG_PATH", clang_path);
    generate_bindings(&sdk_native_dir, api_version)?;

    Ok(())
}
