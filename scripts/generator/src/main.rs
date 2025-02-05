mod dir_conf;
mod enum_prefix;
mod header_conf;
mod opaque_types;

use crate::dir_conf::get_module_bindings_config;
use crate::header_conf::get_bindings_config;
use anyhow::{anyhow, bail, Context};
use bindgen::{CodeGenAttributes, EnumVariation, Formatter};
use log::{debug, error, info, warn};
use std::fs;
use std::num::ParseIntError;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use bindgen::callbacks::EnumVariantValue;
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
            other => {
                return Err(ApiLevelParseError::UnknownApiVersion(other));
            }
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
    let (_, rhs) = line
        .split_once("@deprecated")
        .ok_or_else(|| ParseDeprecatedError::InvalidLine(line.to_string()))?;
    // Variant 1: `@deprecated(since = "XX")`
    // Note: Regex parsing might be more readable, but we want to avoid pulling in more dependencies.
    if let Some(api_level_str) = rhs
        .split_once("(since = \"")
        .or_else(|| rhs.split_once("(since=\""))
        .map(|(_, rhs)| {
            rhs.split_once("\"")
                .expect("String end delimiter not found")
                .0
        })
    {
        return Ok(Some(OpenHarmonyApiLevel::try_from(api_level_str)?));
    }

    if let Some((_, api_level_str)) = rhs.split_once("since ") {
        Ok(Some(OpenHarmonyApiLevel::try_from(api_level_str.trim())?))
    } else {
        Err(ParseDeprecatedError::InvalidLine(line.to_string()))
    }
}

impl bindgen::callbacks::ParseCallbacks for DoxygenCommentCb {
    fn result_error_enum_name(&self, original_enum_name: &str) -> Option<String> {
        original_enum_name
            .strip_suffix("Result")
            .map(|base| format!("{}ErrorCode", base.trim_end_matches("_")))
    }

    fn enum_variant_name(
        &self,
        enum_name: Option<&str>,
        original_variant_name: &str,
        _variant_value: EnumVariantValue,
    ) -> Option<String> {
        let enum_name = enum_name?.trim_start_matches("enum ");
            enum_prefix::ENUM_PREFIX_MAP
                .get(enum_name)
                .and_then(|prefix| original_variant_name.strip_prefix(prefix))
                .map(|stripped| stripped.to_string())

    }

    fn process_comment(&self, comment: &str) -> Option<String> {
        if comment.starts_with(" < ") {
            // The leading space breaks the ///< detection of clang-sys.
            warn!(
                "Invalid doxygen comment. Should apply to item on left, but malformed: `{comment}`"
            );
            return None;
        }
        // Replace manual linebreaks in doxygen with double linebreaks for markdown.
        let comment = comment.replace("\\n", "\n");
        Some(doxygen_rs::transform(&comment))
        // None
    }

    fn parse_comments_for_attributes(&self, comment: &str) -> Vec<CodeGenAttributes> {
        let mut attributes: Vec<CodeGenAttributes> = vec![];
        let api_version = comment
            .lines()
            // TODO: Investigate why some comments appear to have already been processed!
            .find_map(|line| {
                line.split_once("@since")
                    .or_else(|| line.split_once("Available since API-level: "))
            })
            .map(|(_, since)| {
                let api_level_str = since.trim();
                let api_level: Result<OpenHarmonyApiLevel, _> = api_level_str
                    .try_into()
                    .inspect_err(|err| error!("Failed to parse OH API version: {:?}", err));
                api_level.expect("Failed to parse OH API version")
            });
        if let Some(api_version) = api_version {
            let cfg = format!("feature = \"api-{}\"", api_version as u32);
            // Our Minimum api-level is 10, so we don't feature guard things <= API level 10.
            if api_version > OpenHarmonyApiLevel::Ten {
                attributes.push(CodeGenAttributes::Cfg(cfg));
                attributes.push(CodeGenAttributes::CfgAttr(format!(
                    "docsrs, doc(cfg(feature = \"api-{}\"))",
                    api_version as u32
                )));
            }
        }

        if let Some(deprecated_line) = comment.lines().find(|line| line.contains("@deprecated")) {
            let deprecated_since = parse_deprecated_since(deprecated_line).expect("Parse failed");
            let deprecated_opt =
                deprecated_since.map(|since| format!("since = \"{}\"", since as u32));
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
        .formatter(Formatter::None)
        .merge_extern_blocks(true)
        .rust_target(bindgen::RustTarget::from_str("1.78").expect("invalid rust target"))
        .generate_cstr(true)
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
    /// Optionally transform the output file name stem
    rename_output_file: Option<Box<dyn Fn(&str) -> String>>,
    /// Options which apply to all or most files
    set_builder_opts: Box<dyn Fn(&str, &Path, bindgen::Builder) -> bindgen::Builder>,
}

struct BindingConf {
    include_filename: String,
    /// Will be interpolated as "${ROOT_DIR}/${output_prefix}_ffi"
    output_prefix: String,
    set_builder_opts: Box<dyn FnOnce(bindgen::Builder) -> bindgen::Builder>,
}

/// Generate bindings for the helper crate with the opaque type definitions.
fn generate_opaque_types_bindings(
    root_dir: &Path,
    builder: bindgen::Builder,
    sysroot_include_dir: &Path,
) -> anyhow::Result<()> {
    let mut builder = builder
        .header(
            root_dir
                .join("components/opaque-types/wrapper.h")
                .to_str()
                .unwrap(),
        )
        .ignore_functions()
        .ignore_methods()
        .derive_debug(false)
        .derive_copy(false)
        .generate_comments(true)
        .clang_args(&["-x", "c++"]);
    for ty_name in opaque_types::OPAQUE_TYPES {
        builder = builder.allowlist_type(ty_name)
    }
    let binding = builder.generate().context("Bindgen failed")?;
    binding
        .write_to_file(root_dir.join("components/opaque-types/src/opaque_types.rs"))
        .context("Failed to write bindings to file")?;
    Ok(())
}

fn generate_bindings(sdk_native_dir: &Path, api_version: u32) -> anyhow::Result<()> {
    let base_builder = base_bindgen_builder(&sdk_native_dir.join("sysroot"))?;
    let base_builder = base_builder
        .clang_arg("-D")
        .clang_arg(format!("BINDGEN_OHOS_API_LEVEL={api_version}"));
    let sysroot_include_dir = sdk_native_dir.join("sysroot/usr/include");
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    let root_dir = manifest_dir
        .parent()
        .and_then(|d| d.parent())
        .ok_or(anyhow!("Could not determine root directory"))?
        .canonicalize()
        .context("Could not canonicalize root directory")?;

    generate_opaque_types_bindings(&root_dir, base_builder.clone(), &sysroot_include_dir)?;

    let mut base_builder = base_builder;
    for ty_name in opaque_types::OPAQUE_TYPES {
        base_builder = base_builder.blocklist_type(ty_name)
    }

    for binding in get_bindings_config(api_version) {
        debug!("Generating binding: {}", binding.include_filename);
        let header_filename = sysroot_include_dir.join(binding.include_filename);
        let header_filename_str = header_filename.to_str().context("Unicode")?;
        let builder = base_builder.clone().header(header_filename_str);
        let builder = (binding.set_builder_opts)(builder);
        let bindings = builder.generate().context("Bindgen failed")?;

        bindings
            .write_to_file(root_dir.join(format!("{}_ffi.rs", binding.output_prefix)))
            .context("Failed to write bindings to file")?;
    }

    for binding in &get_module_bindings_config() {
        debug!("Generating binding: {}", binding.directory);
        let module_dir = sysroot_include_dir.join(&binding.directory);
        if !module_dir.exists() {
            bail!(
                "Could not find directory {} at {}",
                binding.directory,
                module_dir.display()
            );
        }
        let paths = fs::read_dir(module_dir)?;
        for file in paths {
            let file = file.context("Failed to enumerate dir")?;
            if file.file_type()?.is_dir() {
                bail!("Subdirectories are not supported yet by this script");
            }
            let file_path = file
                .path()
                .canonicalize()
                .context("Failed to canonicalize path")?;
            let header_filename_str = file_path.to_str().context("Unicode")?;
            let file_stem = file_path
                .file_stem()
                .ok_or(anyhow!("Failed to get filestem"))?
                .to_str()
                .context("Unicode")?;
            let file_stem = binding
                .rename_output_file
                .as_ref()
                .map(|rename| rename(file_stem))
                .unwrap_or(file_stem.to_string());
            let builder = base_builder
                .clone()
                .header(header_filename_str)
                .allowlist_file(header_filename_str);
            let builder = (binding.set_builder_opts)(&file_stem, file_path.as_path(), builder);

            let bindings = builder.generate().context("Bindgen failed")?;
            let base_path = root_dir.join(&binding.output_dir).join(&file_stem);
            if !base_path.exists() {
                info!(
                    "Creating target directory for bindings: {}",
                    base_path.display()
                );
                fs::create_dir_all(&base_path)
                    .context("Failed to create target directory for bindings")?;
            }

            bindings
                .write_to_file(base_path.join(format!("{file_stem}_ffi.rs")))
                .context("Failed to write bindings to file")?;
        }
    }

    Ok(())
}

fn main() -> anyhow::Result<()> {
    env_logger::init();
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
    debug!(
        "Libclang: {}, clang: {}",
        libclang_path.display(),
        clang_path.display()
    );
    std::env::set_var("LIBCLANG_PATH", libclang_path);
    std::env::set_var("CLANG_PATH", clang_path);
    generate_bindings(&sdk_native_dir, api_version)?;

    Ok(())
}
