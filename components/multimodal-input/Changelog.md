# Changelog

## 0.3.0 (2025-06-06)

- Move some opaque types to ohos-sys-opaque-types. This is not a breaking change, but 
  to make cargo semver checks happy we bump the major.

## 0.2.0 (2025-01-30)

### Breaking

- `Input_Result` is now an alias to `Result<(), NonZero<InputError>>`

## v0.1.1

### Add

- Update bindings for API-14

## v0.1.0

- Initial release
