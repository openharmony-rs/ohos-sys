# ohos-sys

FFI-bindings for the native API of [OpenHarmony OS]. See the [documentation] for a list of supported components.
This crate is under active development, and not officially affiliated with OpenHarmony OS.

## Status of the bindings

Here is an overview of the available C header directories in the OpenHarmony sysroot, and for which of them
this crate already provides bindings. The API-level column denotes up to which api level the bindings have
already been generated.

| API name                                | status | API-level | crate                     |
|-----------------------------------------|--------|-----------|---------------------------|
| AbilityKit                              | ✅      | 21        | [ohos-abilitykit-sys]     |
| BasicServicesKit                        |        |           |                           |
| Background Process Manager              |        |           |                           |
| ConnectivityKit                         |        |           |                           |
| CryptoArchitectureKit                   |        |           |                           |
| DataProtectionKit                       |        |           |                           |
| GameControllerKit                       |        |           |                           |
| IPCKit                                  | ✅      | 21        | [ohos-ipckit-sys]         |
| LocationKit                             |        |           |                           |
| NotificationKit                         |        |           |                           |
| TEEKit                                  |        |           |                           |
| accesstoken                             |        |           |                           |
| ace/xcomponent                          | ✅      | 21        | [xcomponent-sys]          |
| ark_runtime                             |        |           |                           |
| arkui                                   | ✅      | 21        | [arkui-sys]               |
| asset                                   |        |           |                           |
| bundle                                  |        |           |                           |
| database                                | ✅      | 21        | [ohos-rdb-sys]            |
| ddk                                     |        |           |                           |
| distributedhardware                     |        |           |                           |
| ffrt                                    |        |           |                           |
| filemanagement                          |        |           |                           |
| hiappevent                              |        |           |                           |
| hicollie                                |        |           |                           |
| hid                                     |        |           |                           |
| hidebug                                 |        |           |                           |
| hilog                                   | ✅      | 21        | [hilog-sys]               |
| hitrace                                 | ✅      | 21        | [hitrace-sys]             |
| huks                                    |        |           |                           |
| info                                    |        |           |                           |
| inputmethod                             | ✅      | 21        | [ohos-ime-sys]            |
| mindspore                               |        |           |                           |
| multimedia                              |        |           |                           |
| multimodalinput                         | ✅      | 21        | [ohos-input-sys]          |
| napi                                    | ✅      | 21        |                           |
| native_buffer                           | ✅      | 21        | [ohos-window-sys]         |
| native_color_space_manager              |        |           |                           |
| native_display_soloist                  |        |           |                           |
| native_drawing                          | ✅      | 21        | [ohos-drawing-sys]        |
| native_effect                           |        |           |                           |
| native_fence                            | ✅      | 21        | [ohos-window-sys]         |
| native_image                            | ✅      | 21        | [ohos-window-sys]         |
| native_vsync                            | ✅      | 21        | [ohos-vsync-sys]          |
| native_window                           | ✅      | 21        | [ohos-window-sys]         |
| network                                 |        |           |                           |
| neural_network_runtime                  |        |           |                           |
| ohaudio                                 | ✅      | 21        | [ohaudio-sys]             |
| ohcamera                                |        |           |                           |
| Pasteboard                              | ✅      | 21        | [ohos-pasteboard-sys]     |
| purgeable_memory                        |        |           |                           |
| qos                                     |        |           |                           |
| rawfile                                 | ✅      | 21        | [ohos-rawfile-sys]        |
| resourcemanager                         |        |           |                           |
| sensors                                 | ✅      | 21        | [ohos-sensors-sys]        |
| SCSI Peripherals                        |        |           |                           |
| telephony                               |        |           |                           |
| transient_task                          |        |           |                           |
| Unified Data Management Framework(UDMF) | ✅      | 21        | [udmf-sys]                |
| usb                                     |        |           |                           |
| usb serial                              |        |           |                           |
| web                                     | ✅      | 21        | [arkweb-sys]            |
| window_manager                          | ✅      | 21        | [ohos-window-manager-sys] |

[arkui-sys]: https://docs.rs/arkui-sys/latest/arkui_sys/
[hilog-sys]: https://docs.rs/hilog-sys/latest/hilog_sys/
[hitrace-sys]: https://docs.rs/hitrace-sys/latest/hitrace_sys/
[ohos-drawing-sys]: https://docs.rs/ohos-drawing-sys/latest/ohos_drawing_sys/
[ohos-ime-sys]: https://docs.rs/ohos-ime-sys/latest/ohos_ime_sys/
[ohos-input-sys]: https://docs.rs/ohos-input-sys/latest/ohos_input_sys/
[ohos-ipckit-sys]: https://docs.rs/ohos-ipckit-sys/latest/ohos_ipckit_sys/
[ohaudio-sys]: https://docs.rs/ohaudio-sys/latest/ohaudio_sys/
[ohos-pasteboard-sys]: https://docs.rs/ohos-pasteboard-sys/latest/ohos_pasteboard_sys/
[ohos-rawfile-sys]: https://docs.rs/ohos-rawfile-sys/latest/ohos_rawfile_sys/
[rdb-sys]: https://docs.rs/ohos-rdb-sys/latest/ohos-rdb_sys/
[ohos-window-manager-sys]: https://docs.rs/ohos-window-manager-sys/latest/ohos_window_manager_sys/
[ohos-window-sys]: https://docs.rs/ohos-window-sys/latest/ohos_window_sys/
[arkweb-sys]: https://docs.rs/arkweb-sys/latest/arkweb_sys/
[ohos-abilitykit-sys]: https://docs.rs/ohos-abilitykit-sys/latest/ohos_abilitykit_sys/
[ohos-vsync-sys]: https://docs.rs/ohos-vsync-sys/latest/ohos_vsync_sys/
[udmf-sys]: https://docs.rs/udmf-sys/latest/udmf_sys/
[ohos-sensors-sys]: https://docs.rs/ohos-sensors-sys/latest/ohos_sensors_sys/
[xcomponent-sys]: https://docs.rs/xcomponent-sys/latest/xcomponent_sys/


## Development

The current bindings are generated with `bindgen` using `scripts/generate_bindings.sh`.
Bindings are currently generated with the OpenHarmony SDK API level 21, but with items
added after API level 10 feature guarded behind `api-XX` features. This is handled
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
