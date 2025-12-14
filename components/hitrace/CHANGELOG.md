# Changelog

## 0.1.6

- Update bindings to API-21. This adds the new `OH_Hitrace_<variant>Ex` functions,
  which allow specifying log levels and custom key-value arguments.

## 0.1.5

- Add API-16, 17 and 18 (no changes)

## 0.1.5

- Add API-14 and API-15 (no changes)

## 0.1.4 (2025-01-09)

- Added API-13 feature (no changes)

# 0.1.3 (2025-01-04)

## Changes

- Regenerated the bindings with new generator
- `HiTrace_Tracepoint_Type` is now marked as non-exhaustive. This change is not considered breaking,
  since the enum is only passed towards ffi and not received from ffi functions.

# 0.1.2 (2024-09-05)

## Added

- hitrace API level 12 bindings (beta)
