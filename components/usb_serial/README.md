## USB Serial DDK native bindings for OpenHarmony

Raw FFI bindings to the OpenHarmony USB Serial DDK (`libusb_serial_ndk.z.so`), available since API level 18. Calling these APIs requires `ohos.permission.ACCESS_DDK_USB_SERIAL`.

The C functions return `int32_t`, and `USB_SERIAL_DDK_SUCCESS` is `31600000` rather than zero.

C API reference: https://gitcode.com/openharmony/docs/blob/master/zh-cn/application-dev/device/driver/usb-serial-ddk-guidelines.md

## Development

This crate is developed as part of the [ohos-sys](https://github.com/openharmony-rs/ohos-sys) bindings.

## License

This crate is licensed under the Apache-2.0 license, matching the OpenHarmony OS SDK.
