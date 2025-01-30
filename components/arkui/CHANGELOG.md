# Changelog

## 0.3.0 (unreleased)

### Breaking

- Removed `ARKUI_DRAG_RESULT_` prefix from `ArkUI_DragResult` variants.
- Removed `GESTURE_INTERRUPT_RESULT_` prefix from `ArkUI_GestureInterruptResult` variants.
- Replace `ArkUI_ErrorCode` with `ArkUiResult` (an alias to `Result<(), NonZero<ArkUiErrorCode>>`)

## 0.2.3 (2025-01-09)

### Add

- Add remaining bindings: native dialog, native interface, native node and native interface accessibility bindings.

## 0.2.2 (2025-01-08)

- Update bindings to api-13

## 0.2.1 (2025-01-05)

### Add

- drag and drop
- drawable descriptor
- native animate
- styled string

## 0.2.0 (2025-01-04)

### Breaking

- `ArkUI_NumberValue` is now a native Rust union instead of a bindgen union type.

