# Changelog

## v0.3.2

- Add API-16, 17 and 18 bindings

## v0.3.1

- Add API-15 bindings

## v0.3.0 

- Replace `OH_Drawing_ErrorCode` with `DrawingResult` (an alias to `Result<(), NonZero<DrawingErrorCode>>`)

## v0.2.2 (2025-01-29)

- Update bindings to API-14
- `OH_Drawing_Point2D` and `OH_Drawing_Point3D` now derive Copy, Clone and Debug

## v0.2.1 (2025-01-08)

- Update bindings for API-13


## v0.2.0 (2025-01-04)

- Internal changes to binding generation
- Some types no longer derive Debug, Copy and Clone.

## v0.1.2 (2024-11-11)

- Update API-12 bindings from beta-1 to release (5.0.0.65)
