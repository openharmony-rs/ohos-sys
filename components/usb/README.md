## USB DDK native bindings for OpenHarmony

Raw FFI bindings to the OpenHarmony USB DDK (`libusb_ndk.z.so`), available since API level 10. Calling these APIs requires `ohos.permission.ACCESS_DDK_USB`.

C API reference: https://gitcode.com/openharmony/docs/blob/master/zh-cn/application-dev/reference/apis-driverdevelopment-kit/capi-usb-ddk-api-h.md

`UsbDdkErrCode` selects the historical error values for API 10–15 and the revised values from API 16. The API 12 shared-memory request also exposes the `DDK_Ashmem` layout required by its C signature; this crate does not bind the separate Base DDK functions.

## Development

This crate is developed as part of the [ohos-sys](https://github.com/openharmony-rs/ohos-sys) bindings.

## License

This crate is licensed under the Apache-2.0 license, matching the OpenHarmony OS SDK.
