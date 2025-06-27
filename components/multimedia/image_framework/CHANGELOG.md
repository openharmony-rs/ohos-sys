# Changelog

## 0.3.2

- Update to API-16, 17 and 18

## 0.3.1

- Update to API-15

## 0.3.0 

- Replace `Image_ErrorCode` with `ImageResult` (an alias to `Result<(), NonZero<ImageErrorCode>>`)

## 0.2.2 (2025-01-18)

- Update bindings for API-14

## 0.2.1 (2025-01-11)

- Add `OH_ImageSourceNative_CreateFromRawFile()` and `OH_PixelmapNative_ConvertPixelmapNativeToNapi` bindings.

## 0.2.0 (2025-01-09)

### Breaking 

- Use `CStr` instead of raw byte strings

### Add

- Updated the bindings for API-13

## 0.1.2 (2025-01-04)

### Changes

- Regenerated the bindings with new generator
