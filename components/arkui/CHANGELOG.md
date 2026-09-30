# Changelog

## 0.3.5

- Add API-24, API-25 and API-26 bindings.
- API-26 split `native_type.h` into `common_type.h`, `error_code.h`, `native_type_visual.h` and
  the headers in `node_attributes/`. These are now the `common_type`, `error_code`,
  `native_type_visual` and `node_attributes::*` modules. `native_type` re-exports all of them,
  so existing paths keep working. The items of `node_attributes/list_item.h` are only available
  from `native_type`, since newer SDKs merge that header back into `native_type.h`.
- `ArkUI_AttributeItem`, `ArkUI_NodeEvent` and `ArkUI_NodeCustomEventType` moved out of
  `native_node`, and `ArkUI_NodeEvent` out of `drag_and_drop`. They are re-exported at their
  old paths. `drag_and_drop::ArkUI_NodeEvent` and `native_node::ArkUI_NodeEvent` are now the
  same type.
- Add the `native_material` module (API-26).
- The new `styled_string` functions using `ohos-drawing-sys` types are available with the
  `drawing` feature.
- With the `drawing` feature, the `api-*` features now also enable the same API level in
  `ohos-drawing-sys`.
- Add `MouseEventButton`, and `InputEventSourceType::KEY` and `InputEventSourceType::JOYSTICK`
  (API-22), which were missing.
- The API-22 `OH_ArkUI_TextLayoutManager_*` functions using `ohos-drawing-sys` types, which
  were previously not bound, are now available with the `drawing` feature.
- `ArkUI_DrawableDescriptor` is now re-exported from `ohos-sys-opaque-types`.
  The public path `arkui_sys::drawable_descriptor::ArkUI_DrawableDescriptor` is
  unchanged.

## 0.3.4

- Add API-22 and API-23 bindings. Significant additions across `native_node`, `native_type`,
  `native_gesture`, `ui_input_event`, and related modules.

## 0.3.3

- Add API-21 bindings

## 0.3.2

- Add API-16, 17 and 18 bindings

## 0.3.1

- Add API-15 bindings

## 0.3.0

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

