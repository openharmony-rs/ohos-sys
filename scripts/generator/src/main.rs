use std::fs;
use anyhow::{anyhow, bail, Context};
use bindgen::{EnumVariation, Formatter};
use std::path::{Path, PathBuf};

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

fn base_bindgen_builder(sysroot_dir: &Path) -> anyhow::Result<bindgen::Builder> {
    let builder = bindgen::builder()
        .use_core()
        .layout_tests(false)
        .formatter(Formatter::Prettyplease)
        .merge_extern_blocks(true)
        .rust_target(bindgen::RustTarget::Stable_1_77)
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
                    .no_copy("^OH_NativeXComponent$")
                    .no_copy("^OH_NativeXComponent_KeyEvent$")
                    .no_debug("^OH_NativeXComponent$")
                    .no_debug("^OH_NativeXComponent_KeyEvent$")
                    .blocklist_item("^ArkUI_")
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
                    .no_copy("^OH_NativeBuffer$")
                    .no_copy("^OHIPCParcel$")
                    .no_debug("^OH_NativeBuffer$")
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
                    .raw_line("use crate::native_window::OHNativeWindow;")
                    .allowlist_file(r".*/native_image/.*\.h")
                    .blocklist_item("^(OH)?NativeWindow(Buffer)?")
                    .no_copy("^OH_NativeImage$")
                    .no_copy("^OH_OnFrameAvailableListener")
                    .no_debug("^OH_NativeImage$")
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
                    // todo: if API == 10, --blocklist-item=^NativeWindowOperation$
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
                         builder.raw_line("use crate::types::*;")
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
                     let builder = match file_stem {
                         "drawable_descriptor" => {
                             builder.blocklist_item("^OH_PixelmapNative$")
                         },
                         "native_type" => {
                             builder //.raw_line("use crate::drawable_descriptor::ArkUI_DrawableDescriptor;")
                              .blocklist_function("^OH_ArkUI_ImageAnimatorFrameInfo_CreateFromDrawableDescriptor$")

                         },
                         "native_gesture" => {
                             builder
                                 .raw_line("use crate::ui_input_event::ArkUI_UIInputEvent;")
                                 .blocklist_function("^OH_ArkUI_GestureEvent_GetNode")
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
            .header(header_filename_str)
            .allowlist_file(header_filename_str);
        let builder = (binding.set_builder_opts)(builder);
        let bindings = builder.generate().context("Bindgen failed")?;

        bindings
            .write_to_file(root_dir.join(format!("{}_api{api_version}.rs", binding.output_prefix)))
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

            // We want to commit all generated files to version control, so we can easily see if something changed,
            // when updating bindgen or the SDK patch release.
            // However, we any split changes into incremental modules, and don't use any of the newer versions of the API
            // besides the first one. If a binding was not introduced in the current api version, then we add a nopublish
            // suffix, so we can exclude the file from cargo publish and save some download bandwidth.
            let previous_version_stem = format!("{file_stem}_api{}", api_version - 1);
            let no_publish_suffix = if
                        base_path.join(format!("{previous_version_stem}.rs")).is_file()
                    ||  base_path.join(format!("{previous_version_stem}_nopublish.rs")).is_file() {
                println!("Found older API file for {}", base_path.display());
                "_nopublish"
            } else {
                ""
            };
            bindings
                .write_to_file(base_path.join(format!("{file_stem}_api{api_version}{no_publish_suffix}.rs")))
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
