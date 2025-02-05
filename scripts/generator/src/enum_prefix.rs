use std::collections::HashMap;
use std::sync::LazyLock;

/// Defines the prefixes that each enum has (which we probably want to remove)
pub(crate) static ENUM_PREFIX_MAP: LazyLock<HashMap<&'static str, &'static str>> = LazyLock::new(|| {
    HashMap::from([
        ("Image_ErrorCode", "IMAGE_"),
        ("OH_Drawing_ErrorCode", "OH_DRAWING_ERROR_"),
        ("ArkUI_DragResult", "ARKUI_DRAG_RESULT_"),
        ("ArkUI_GestureInterruptResult", "GESTURE_INTERRUPT_RESULT_"),
        ("ArkUI_ErrorCode", "ARKUI_ERROR_CODE_"),
        ("InputMethod_ErrorCode", "IME_ERR_"),
        ("Input_Result", "INPUT_"),
        ("OH_Drawing_FontConfigInfoErrorCode", "ERROR_FONT_CONFIG_INFO_")
    ])
});