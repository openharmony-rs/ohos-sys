# ohos-sys

FFI-bindings for the native API of [OpenHarmony OS]. See the [documentation] for a list of supported components.
This crate is under active development, and not officially affiliated with OpenHarmony OS.

## Status of the bindings

Here is an overview of the available C header directories in the OpenHarmony sysroot, and for which of them
this crate already provides bindings. The API-level column denotes up to which api level the bindings have
already been generated.

| API name                                | status | API-level |
|-----------------------------------------|--------|-----------|
| AbilityKit                              | ✅      | 21        |
| BasicServicesKit                        |        |           |
| Background Process Manager              |        |           |
| ConnectivityKit                         |        |           |
| CryptoArchitectureKit                   |        |           |
| DataProtectionKit                       |        |           |
| GameControllerKit                       |        |           |
| IPCKit                                  |        |           |
| LocationKit                             |        |           |
| NotificationKit                         |        |           |
| TEEKit                                  |        |           |
| accesstoken                             |        |           |
| ace/xcomponent                          | ✅      | 21        |
| ark_runtime                             |        |           |
| arkui                                   | ✅      | 21        |
| asset                                   |        |           |
| bundle                                  |        |           |
| database                                |        |           |
| ddk                                     |        |           |
| distributedhardware                     |        |           |
| ffrt                                    |        |           |
| filemanagement                          |        |           |
| hiappevent                              |        |           |
| hicollie                                |        |           |
| hid                                     |        |           |
| hidebug                                 |        |           |
| hilog                                   | ✅      | 21        |
| hitrace                                 | ✅      | 21        |
| huks                                    |        |           |
| info                                    |        |           |
| inputmethod                             | ✅      | 21        |
| mindspore                               |        |           |
| multimedia                              |        |           |
| multimodalinput                         | ✅      | 21        |
| napi                                    | ✅      | 21        |
| native_buffer                           | ✅      | 21        |
| native_color_space_manager              |        |           |
| native_display_soloist                  |        |           |
| native_drawing                          | ✅      | 21        |
| native_effect                           |        |           |
| native_fence                            |        |           |
| native_image                            | ✅      | 21        |
| native_vsync                            | ✅      | 21        |
| native_window                           | ✅      | 21        |
| network                                 |        |           |
| neural_network_runtime                  |        |           |
| ohaudio                                 |        |           |
| ohcamera                                |        |           |
| Pasteboard                              | ✅      | 21        |
| purgeable_memory                        |        |           |
| qos                                     |        |           |
| rawfile                                 | ✅      | 21        |
| resourcemanager                         |        |           |
| sensors                                 |        |           |
| SCSI Peripherals                        |        |           |
| telephony                               |        |           |
| transient_task                          |        |           |
| Unified Data Management Framework(UDMF) | ✅      | 21        |
| usb                                     |        |           |
| usb serial                              |        |           |
| web                                     |        |           |
| window_manager                          | ✅      | 21        |

## Development

The current bindings are generated with `bindgen` using `scripts/generate_bindings.sh`.
Bindings are currently generated with the OpenHarmony SDK API level 21, but with items
added after API level 10 feature guarded behind `api-XX` features. this is handled
automatically by the tool based on the documentation comments in the C header files.

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

- Preferably generate the bindings with libclang in `C` mode. However, if a header file is not C-compliant
  due to an issue of the OpenHarmony SDK, then setting `libclang` to C++ mode is fine.
- Be sure to guard the new component behind a cargo feature and document the feature in Cargo.toml.


## License

This crate is licensed under the Apache-2.0 license, matching the OpenHarmony OS SDK.

[OpenHarmony OS]: https://docs.openharmony.cn/pages/v5.0/en/OpenHarmony-Overview.md
[documentation]: https://docs.rs/ohos-sys/latest/ohos_sys/
