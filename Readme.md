# ohos-sys

FFI-bindings for the native API of [OpenHarmony OS]. See the [documentation] for a list of supported components.
This crate is under active development, and not officially affiliated with OpenHarmony OS.

## Development

The current bindings are generated with `bindgen` using `scripts/generate_bindings.sh`.
A separate file is generated for each API version, and (assuming no breaking changes) the new additions are
manually copied to an `apiXX_additions.rs` file, which is included as a module if `feature = api-XX` is selected.
The generated file `drawing_apiXX.rs` should be committed to version control, so we can easily rerun the script on a 
patch-release for a given API-level and see what changed.
The file itself is however not needed, and will be excluded from `crates.io` releases.

# Contributing

There are still quite a few OpenHarmony APIs missing. Feel free to contribute missing APIs, but be sure to adapt
the script, so your bindings are reproducible! 
The OpenHarmony SDK can be downloaded from the release notes of the respective release, e.g. the
[5.0.0 release notes](https://docs.openharmony.cn/pages/v5.0/en/release-notes/OpenHarmony-v5.0.0-release.md).
Navigate to the `Acquiring Source Code from Mirrors` section, select the `Public SDK package for the standard system`
for your host Operating System (Windows / Linux / Mac) click download and optionally verify the SHA-256 checksum 
of the downloaded archive. 
Extract the archive to a suitable location. Please note that the <os_name> subfolder contains more archives.
For the purpose of generating the bindings extracting the `native` archive is sufficient.

Once you have setup your local SDK, you should set the environment variable `OHOS_SDK_NATIVE` to 
`/path/to/ohos-sdk/<your_host_os>/native`. Afterwards you can run the script to generate the bindings
and adapt it to incorporate new modules.

Please also check the following:

- Ensure that opaque struct definitions do not derive `Copy`, `Clone` and `Debug`.
- Blocklist all unnecessary type definitions, e.g. from the C standard library.
- Preferably generate the bindings with libclang in `C` mode. However, if a header file is not C-compliant
  due to an issue of the OpenHarmony SDK, then setting `libclang` to C++ mode is fine.
- Be sure to guard the new component behind a cargo feature and document the feature in Cargo.toml.
- If you did not generate the bindings with API-level 10, specify which API-level you generated the bindings with
  and guard the generated module behind the corresponding api-level feature flag.
- Installing `bindgen`: We require at least bindgen 0.70.0, with the `prettyplease` feature enabled. 
  You can install it by running `cargo install bindgen-cli --features prettyplease`

## License

This crate is licensed under the Apache-2.0 license, matching the OpenHarmony OS SDK.

[OpenHarmony OS]: https://docs.openharmony.cn/pages/v5.0/en/OpenHarmony-Overview.md
[documentation]: https://docs.rs/ohos-sys/latest/ohos_sys/
