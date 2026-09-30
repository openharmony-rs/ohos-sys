# Changelog

## 0.1.8

- Add API-24, API-25 and API-26 bindings.
- `NATIVEBUFFER_PIXEL_FMT_RGB_565` and `NATIVEBUFFER_PIXEL_FMT_BUTT` no longer require the
  `api-12` feature, and the docs of the `OH_NativeBuffer_Format` variants are no longer shifted.

## 0.1.7

- Update bindings to API-22 and API-23. Adds `OH_NativeBuffer_IsSupported` and related
  config helpers in `native_buffer`, and additional `native_image` / `native_window`
  callbacks at API-22.

## 0.1.6

- Fix missing link of native library for native-fence.

## 0.1.5

- Add native-fence bindings

## 0.1.4

- Add API-21 bindings

## 0.1.3

- Add API-16, 17 and 18 bindings

## 0.1.2

- Add API-15 bindings

## 0.1.1

- Fix build with API-level 11

## 0.1.0

- Initial Release