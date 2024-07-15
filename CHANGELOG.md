# v0.2.0 (unreleased)

## Breaking

- Renamed and moved the xcomponent module to the top-level  (from ace/xcomponent/native_interface_xcomponent)
- Guard each component behind a feature flag

## Added

- Added `native_drawing` API bindings
- Added bindings for API level 11 behind a feature flag

## Fixed

- `native_window` now links against the correct dynamic library.
- Remove Copy / Clone impls on opaque structs
- 