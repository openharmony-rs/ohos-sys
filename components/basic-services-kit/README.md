# OpenHarmony BasicServicesKit bindings

Raw FFI bindings to the OpenHarmony BasicServicesKit NDK. BasicServicesKit
groups several unrelated system-service NDKs under one upstream kit; this
crate exposes each behind its own cargo feature:

| Sub-API     | Feature        | Library                  | Min API |
| ----------- | -------------- | ------------------------ | ------- |
| CommonEvent | `commonevent`  | `libohcommonevent.so`    | 12      |
| BatteryInfo | `battery-info` | `libohbattery_info.so`   | 13      |
| Print       | `print`        | `libohprint.so`          | 12      |
| Scan        | `scan`         | `libohscan.so`           | 12      |
| OS Account  | `os-account`   | `libos_account_ndk.so`   | 12      |
| TimeService | `time-service` | `libtime_service_ndk.so` | 12      |

A sub-API feature is a no-op unless the matching `api-XX` feature is also
enabled — i.e. enabling `commonevent` without `api-12` (or higher) leaves
the module hidden and the `.so` unlinked. The `all-apis` meta-feature
enables every sub-API at once.

C API reference: https://gitcode.com/openharmony/docs/blob/master/en/application-dev/reference/apis-basic-services-kit/

## License

Licensed under the Apache-2.0 license, matching the license of OpenHarmony.
