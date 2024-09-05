# v0.3.1 (2024-09-05)

## Add

- Re-export hitrace-sys binding (behind hitrace feature)

# v0.3.0 (2024-08-29)

## Breaking

- Change enum types in `native_buffer` and `native_window` to newtype pattern.

## Added

- deviceinfo bindings 
- native_buffer API-12 bindings
- native_image bindings
- syscap bindings

# v0.2.2 (2024-08-18)

## Added

- Added bindings for `native_vsync` (behind the `vsync` feature flag)

# v0.2.1

## Fixed

- `ohos-drawing-sys` is now an optional dependency. Usage was already guarded behind the `drawing`
  feature.

# v0.2.0 (2024-07-18)

## Breaking

- Renamed and moved the xcomponent module to the top-level  (from ace/xcomponent/native_interface_xcomponent)
- Guard each component behind a feature flag

## Added

- Added `native_drawing` API bindings (Also available separately as `ohos-drawing-sys` )
- Added bindings for API level 11 behind a feature flag

## Fixed

- `native_window` now links against the correct dynamic library.
- Remove Copy / Clone impls on opaque structs
- 