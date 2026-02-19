## Initial setup

1. Make sure to download the OpenHarmony SDK. Use the same version as documented in the README.
2. Set the environment variable `OHOS_SDK_NATIVE` to the `native` directory in the OpenHarmony SDK.
   It can be useful to use a tool like [direnv](https://direnv.net/) to set this variable automatically.
3. If you run `scripts/generate_bindings.sh` now, with the same OpenHarmony SDK version as documented in the README,
   there should be no errors and no diff.

## Adding a new `-sys` crate for an OpenHarmony module / API

1. Determine the folder in the OpenHarmony sysroot include directory.
2. Go to `components` and run `cargo new --lib <name>` with a suitable name for the sys crate.
   Add a short Readme.md file describing the purpose, the license (Apache-2.0) and linking to the official OpenHarmony
   documentation for that module. Edit Cargo.toml and add a description, the license, link to this repository and add
   relevant keywords.
3. Add `document-features` as an optional dependency, with a feature of the same name to enable it, and add the following
   snippet at the bottom of Cargo.toml:
   ```toml
    [package.metadata.docs.rs]
    features = ["document-features"]
    targets = ["aarch64-unknown-linux-ohos", "armv7-unknown-linux-ohos", "x86_64-unknown-linux-ohos"]
    all-features = true
    ```
4. Add a modul level doc comment to the crate root with a link to the OpenHarmony documentation and
   end the documentation with:
   ```rust
    //! ## Feature flags
    #![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
    )]
    #![cfg_attr(docsrs, feature(doc_cfg))]
    ```
   This will enable the feature flags to be documented.
5. Edit scripts/generator/src/dir_conf.rs and add a `DirBindingsConf` entry for the new crate.
   "directory" should be the path to the directory in the sysroot include directory. `output_dir` should be the path
   to the `src` directory of the crate you just created. If the header files have a common prefix, you can set
   `header_prefix` so that the generator can strip the prefix from the header filenames.
   The rest of the fields can be initialized via `..Default::default()`
6. Go to the scripts directory, make sure that `OHOS_SDK_NATIVE` is set to the `native` directory inthe OpenHarmony SDK,
   and then run `ONLY_MODULE=<c_module_name> ./generate_bindings.sh`. `ONLY_MODULE` can be set to a specific module
   name to only generate bindings for that module, which is useful for debugging and quick iteration.
7. Go to the `src` directory of the new crate and notice that new directories have been created for each header file.
   These directories contain the generated bindings for that header file. We now need to add modules for each of these
   directories to the `src/lib.rs` file. E.g. if the directory is `raw_dir`, then we add `pub mod raw_dir;` to the
   `lib.rs` file and create a raw_dir.rs file in the same directory. The raw_dir.rs file should contain the following:
   ```rust
   mod raw_dir_ffi;
   pub use raw_dir_ffi::*;
   ```
   I.e., it should re-export the bindings from `_ffi.rs` file. If necessary the `mod` statement can later be annottaed
   to suppress warnings.
8. Check the documentation of the header file that binding was generated from and search for a file-level doxygen 
   comment with `@library` that tells the native library name we need to link against. Strip the trailing `.so` and the
    `lib` prefix and add the following link instruction to the `lib.rs` file:
    ```rust
    #[link(name = "libname")]
    unsafe extern "C" {}
    ```
    Make sure to replace `libname` with the actual library name.
9. You now have created the skeleton. Continue in the next section to fix any errors and fine-tune the bindings.

### Fixing and Fine-tuning the bindings

1. Run `ONLY_MODULE=<c_module_name> ./generate_bindings.sh` again. The bindings should now be formatted.
2. You will probably see many errors about missing types. You can use `set_builder_opts` and add custom
   builder options per header file to fix these. Commonly you will use some opaque types, or types from other modules,
   and need to add `use` statements via `builder.rawline()`. If items were added in certain API levels, also add the 
   feature guard to the `use` statement. Generally you can assume a minimum API level of 10, so itemes added in API 
   level 10 can be used without feature guards.
3. Avoid editing any `_ffi.rs` files directly. Always modify the `DirBindingsConf` entry and run `generate_bindings.sh` again.
   It can sometimes be quicker to first test by editing the _ffi.rs file directly, to see if the changes are correct,
   but don't forget to add the change to `DirBindingsConf` afterwards, otherwise the change will be lost the next time
   you run `generate_bindings.sh`.
4. If you encounter functions which require opaque types from a different crate, ensure that 
   `components/opaque-types/wrapper.h` includes the header file containing the opaque type definition and then edit
   `generator/src/opaque_types.rs` to add the opaque type definition to the `opaque_types` module.
   Rerun `generate_bindings.sh` and you should be good to go (but don't set `ONLY_MODULE` this time).
5. If you encounter functions or types which require a non-opaque type from a different crate, where we don't have
   bindings for yet, then you can consider blocklisting that type and function using the type, while adding a todo
   comment explaining why it is blocked.
6. Check the feature documentation comments in Cargo.toml and document the minimum required API level for the crate,
   and add a note if the API level did not add any new APIs.
7. Check the modules added in `src/lib.rs`. If the header file the module was generated from was added in a later API
   level than the minimum required API level, add a feature guard to the module and annottate it for docs.rs with:
   ```rust
   #[cfg(feature = "api-<level>")]
   #[cfg_attr(docsrs, doc(cfg(feature = "api-<level>")))]
   mod my_mod;
   ```
8. Run cargo check for each feature-level, to make sure there are no errors.

## Link Smoke Tests

To ensure linking works with the SDK sysroot, we maintain smoke integration tests that references at least
one symbol from each module. This forces the linker to pull in every required library.

1. Create or update `<crate>/tests/link_smoke.rs` when adding modules:
   - If the module exposes functions, call one function.
   - If the module is type/constant-only, it can be skipped.
2. Run the CI link check script:
   ```bash
   OHOS_SDK_NATIVE=/path/to/ohos-sdk/<your_host_os>/native scripts/ci_link_check.sh
   ```
   This builds the test binary for `aarch64-unknown-linux-ohos` using the SDK sysroot.



## Fixing common issues when generating bindings

### error: unknown type name 'bool'

Some OpenHarmony headers use `bool` without including stdbool.h, which is valid C++, but not valid C.
You can fix this by adding `.clang_args(["-include", "stdbool.h"])` to the builder of `DirBindingsConf`.
