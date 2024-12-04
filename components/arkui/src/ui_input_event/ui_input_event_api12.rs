/* automatically generated by rust-bindgen 0.70.1 */

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use crate::native_type::*;

#[repr(C)]
pub struct ArkUI_UIInputEvent {
    _unused: [u8; 0],
}
impl ArkUI_UIInputEvent_Type {
    pub const ARKUI_UIINPUTEVENT_TYPE_UNKNOWN: ArkUI_UIInputEvent_Type = ArkUI_UIInputEvent_Type(0);
}
impl ArkUI_UIInputEvent_Type {
    pub const ARKUI_UIINPUTEVENT_TYPE_TOUCH: ArkUI_UIInputEvent_Type = ArkUI_UIInputEvent_Type(1);
}
impl ArkUI_UIInputEvent_Type {
    pub const ARKUI_UIINPUTEVENT_TYPE_AXIS: ArkUI_UIInputEvent_Type = ArkUI_UIInputEvent_Type(2);
}
impl ArkUI_UIInputEvent_Type {
    /// Mouse event.
    pub const ARKUI_UIINPUTEVENT_TYPE_MOUSE: ArkUI_UIInputEvent_Type = ArkUI_UIInputEvent_Type(3);
}
#[repr(transparent)]
/** @brief Enumerates the UI input event types.

@since 12*/
#[derive(Clone, Hash, PartialEq, Eq)]
pub struct ArkUI_UIInputEvent_Type(pub ::core::ffi::c_uint);
/// Cancellation of touch.
pub const UI_TOUCH_EVENT_ACTION_CANCEL: _bindgen_ty_1 = _bindgen_ty_1(0);
/// Pressing of a touch point.
pub const UI_TOUCH_EVENT_ACTION_DOWN: _bindgen_ty_1 = _bindgen_ty_1(1);
/// Moving of a touch point.
pub const UI_TOUCH_EVENT_ACTION_MOVE: _bindgen_ty_1 = _bindgen_ty_1(2);
/// Lifting of a touch point.
pub const UI_TOUCH_EVENT_ACTION_UP: _bindgen_ty_1 = _bindgen_ty_1(3);
#[repr(transparent)]
/** @brief Defines the action code of the input event.

@since 12*/
#[derive(Clone, Hash, PartialEq, Eq)]
pub struct _bindgen_ty_1(pub ::core::ffi::c_uint);
/// Unknown tool type.
pub const UI_INPUT_EVENT_TOOL_TYPE_UNKNOWN: _bindgen_ty_2 = _bindgen_ty_2(0);
/// Finger.
pub const UI_INPUT_EVENT_TOOL_TYPE_FINGER: _bindgen_ty_2 = _bindgen_ty_2(1);
/// Pen.
pub const UI_INPUT_EVENT_TOOL_TYPE_PEN: _bindgen_ty_2 = _bindgen_ty_2(2);
/// Mouse.
pub const UI_INPUT_EVENT_TOOL_TYPE_MOUSE: _bindgen_ty_2 = _bindgen_ty_2(3);
/// TouchPad.
pub const UI_INPUT_EVENT_TOOL_TYPE_TOUCHPAD: _bindgen_ty_2 = _bindgen_ty_2(4);
/// JoyStick.
pub const UI_INPUT_EVENT_TOOL_TYPE_JOYSTICK: _bindgen_ty_2 = _bindgen_ty_2(5);
#[repr(transparent)]
/** @brief Defines the tool type of the touch event.

@since 12*/
#[derive(Clone, Hash, PartialEq, Eq)]
pub struct _bindgen_ty_2(pub ::core::ffi::c_uint);
/// Unknown source type.
pub const UI_INPUT_EVENT_SOURCE_TYPE_UNKNOWN: _bindgen_ty_3 = _bindgen_ty_3(0);
/// Mouse.
pub const UI_INPUT_EVENT_SOURCE_TYPE_MOUSE: _bindgen_ty_3 = _bindgen_ty_3(1);
/// Touchscreen.
pub const UI_INPUT_EVENT_SOURCE_TYPE_TOUCH_SCREEN: _bindgen_ty_3 = _bindgen_ty_3(2);
#[repr(transparent)]
/** @brief Defines the source type of the touch event.

@since 12*/
#[derive(Clone, Hash, PartialEq, Eq)]
pub struct _bindgen_ty_3(pub ::core::ffi::c_uint);
impl HitTestMode {
    /** Both the node and its child node respond to the hit test of a touch event, but its sibling node is blocked from
    the hit test.*/
    pub const HTM_DEFAULT: HitTestMode = HitTestMode(0);
}
impl HitTestMode {
    /** The node responds to the hit test of a touch event, but its child node and sibling node are blocked from the hit
    test.*/
    pub const HTM_BLOCK: HitTestMode = HitTestMode(1);
}
impl HitTestMode {
    /** Both the node and its child node respond to the hit test of a touch event, and its sibling node is also
    considered during the hit test.*/
    pub const HTM_TRANSPARENT: HitTestMode = HitTestMode(2);
}
impl HitTestMode {
    /** The node does not respond to the hit test of a touch event, but its child node and sibling node are considered
    during the hit test.*/
    pub const HTM_NONE: HitTestMode = HitTestMode(3);
}
#[repr(transparent)]
/** @brief Enumerates the hit test modes.

@since 12*/
#[derive(Clone, Hash, PartialEq, Eq)]
pub struct HitTestMode(pub ::core::ffi::c_uint);
/// Invalid.
pub const UI_MOUSE_EVENT_ACTION_UNKNOWN: _bindgen_ty_4 = _bindgen_ty_4(0);
/// Press.
pub const UI_MOUSE_EVENT_ACTION_PRESS: _bindgen_ty_4 = _bindgen_ty_4(1);
/// Release.
pub const UI_MOUSE_EVENT_ACTION_RELEASE: _bindgen_ty_4 = _bindgen_ty_4(2);
/// Move.
pub const UI_MOUSE_EVENT_ACTION_MOVE: _bindgen_ty_4 = _bindgen_ty_4(3);
#[repr(transparent)]
/** @brief Define the Action Code for mouse events.

@since 12*/
#[derive(Clone, Hash, PartialEq, Eq)]
pub struct _bindgen_ty_4(pub ::core::ffi::c_uint);
/// None.
pub const UI_MOUSE_EVENT_BUTTON_NONE: _bindgen_ty_5 = _bindgen_ty_5(0);
/// Left.
pub const UI_MOUSE_EVENT_BUTTON_LEFT: _bindgen_ty_5 = _bindgen_ty_5(1);
/// Right.
pub const UI_MOUSE_EVENT_BUTTON_RIGHT: _bindgen_ty_5 = _bindgen_ty_5(2);
/// Middle.
pub const UI_MOUSE_EVENT_BUTTON_MIDDLE: _bindgen_ty_5 = _bindgen_ty_5(3);
/// Back.
pub const UI_MOUSE_EVENT_BUTTON_BACK: _bindgen_ty_5 = _bindgen_ty_5(4);
/// Forward.
pub const UI_MOUSE_EVENT_BUTTON_FORWARD: _bindgen_ty_5 = _bindgen_ty_5(5);
#[repr(transparent)]
/** @brief Define the button type for mouse events.

@since 12*/
#[derive(Clone, Hash, PartialEq, Eq)]
pub struct _bindgen_ty_5(pub ::core::ffi::c_uint);
impl ArkUI_ModifierKeyName {
    /// Ctrl.
    pub const ARKUI_MODIFIER_KEY_CTRL: ArkUI_ModifierKeyName = ArkUI_ModifierKeyName(1);
}
impl ArkUI_ModifierKeyName {
    /// Shift.
    pub const ARKUI_MODIFIER_KEY_SHIFT: ArkUI_ModifierKeyName = ArkUI_ModifierKeyName(2);
}
impl ArkUI_ModifierKeyName {
    /// Alt.
    pub const ARKUI_MODIFIER_KEY_ALT: ArkUI_ModifierKeyName = ArkUI_ModifierKeyName(4);
}
impl ArkUI_ModifierKeyName {
    /// Fn.
    pub const ARKUI_MODIFIER_KEY_FN: ArkUI_ModifierKeyName = ArkUI_ModifierKeyName(8);
}
#[repr(transparent)]
/** @brief Defines an enum for modifier keys.

@since 12*/
#[derive(Clone, Hash, PartialEq, Eq)]
pub struct ArkUI_ModifierKeyName(pub ::core::ffi::c_uint);
extern "C" {
    /** @brief Obtains the type of this UI input event.

    @param event Indicates the pointer to the current UI input event.
    @return Returns the type of the current UI input event; returns <b>0</b> if any parameter error occurs.
    @since 12*/
    pub fn OH_ArkUI_UIInputEvent_GetType(event: *const ArkUI_UIInputEvent) -> i32;
    /** @brief Obtains the action type of this UI input event.

    @param event Indicates the pointer to the current UI input event.
    @return Returns the action type of the current UI input event; returns <b>0</b> if any parameter error occurs.
    @since 12*/
    pub fn OH_ArkUI_UIInputEvent_GetAction(event: *const ArkUI_UIInputEvent) -> i32;
    /** @brief Obtains the source type of this UI input event.

    @param event Indicates the pointer to the current UI input event.
    @return Returns the source type of the current UI input event.
    @since 12*/
    pub fn OH_ArkUI_UIInputEvent_GetSourceType(event: *const ArkUI_UIInputEvent) -> i32;
    /** @brief Obtains the tool type of this UI input event.

    @param event Indicates the pointer to the current UI input event.
    @return Returns the tool type of the current UI input event.
    @since 12*/
    pub fn OH_ArkUI_UIInputEvent_GetToolType(event: *const ArkUI_UIInputEvent) -> i32;
    /** @brief Obtains the time when this UI input event occurs.

    @param event Indicates the pointer to the current UI input event.
    @return Returns the time when the UI input event occurs; returns <b>0</b> if any parameter error occurs.
    @since 12*/
    pub fn OH_ArkUI_UIInputEvent_GetEventTime(event: *const ArkUI_UIInputEvent) -> i64;
    /** @brief Obtains the number of touch points from a directional input event (such as a touch event, mouse event,
    or axis event).

    @param event Indicates the pointer to the current UI input event.
    @return Returns the number of touch points for the directional input event.
    @since 12*/
    pub fn OH_ArkUI_PointerEvent_GetPointerCount(event: *const ArkUI_UIInputEvent) -> u32;
    /** @brief Obtains the ID of a touch point from a directional input event (such as a touch event, mouse event,
    or axis event).

    @param event Indicates the pointer to the current UI input event.
    @param pointerIndex Indicates the index of the target touch point in the multi-touch data list.
    @return Returns the ID of the corresponding touch point.
    @since 12*/
    pub fn OH_ArkUI_PointerEvent_GetPointerId(
        event: *const ArkUI_UIInputEvent,
        pointerIndex: u32,
    ) -> i32;
    /** @brief Obtains the X coordinate relative to the upper left corner of the current component from a directional
    input event (such as a touch event, mouse event, or axis event).

    @param event Indicates the pointer to the directional input event.
    @return Returns the X coordinate relative to the upper left corner of the current component;
    returns <b>0</b> if any parameter error occurs.
    @since 12*/
    pub fn OH_ArkUI_PointerEvent_GetX(event: *const ArkUI_UIInputEvent) -> f32;
    /** @brief Obtains the X coordinate of a specific touch point relative to the upper left corner of the current component
    from a directional input event (such as a touch event, mouse event, or axis event).

    @param event Indicates the pointer to the current UI input event.
    @param pointerIndex Indicates the index of the target touch point in the multi-touch data list.
    @return Returns the X coordinate relative to the upper left corner of the current component;
    returns <b>0.0f</b> if any parameter error occurs.
    @since 12*/
    pub fn OH_ArkUI_PointerEvent_GetXByIndex(
        event: *const ArkUI_UIInputEvent,
        pointerIndex: u32,
    ) -> f32;
    /** @brief Obtains the Y coordinate relative to the upper left corner of the current component from a directional
    input event (such as a touch event, mouse event, or axis event).

    @param event Indicates the pointer to the UI input event.
    @return Returns the Y coordinate relative to the upper left corner of the current component;
    returns <b>0</b> if any parameter error occurs.
    @since 12*/
    pub fn OH_ArkUI_PointerEvent_GetY(event: *const ArkUI_UIInputEvent) -> f32;
    /** @brief Obtains the Y coordinate of a specific touch point relative to the upper left corner of the current component
    from a directional input event (such as a touch event, mouse event, or axis event).

    @param event Indicates the pointer to the current UI input event.
    @param pointerIndex Indicates the index of the target touch point in the multi-touch data list.
    @return Returns the Y coordinate relative to the upper left corner of the current component;
    returns <b>0.0f</b> if any parameter error occurs.
    @since 12*/
    pub fn OH_ArkUI_PointerEvent_GetYByIndex(
        event: *const ArkUI_UIInputEvent,
        pointerIndex: u32,
    ) -> f32;
    /** @brief Obtains the X coordinate relative to the upper left corner of the current application window from a
    directional input event (such as a touch event, mouse event, or axis event).

    @param event Indicates the pointer to the UI input event.
    @return Returns the X coordinate relative to the upper left corner of the current application window;
    returns <b>0</b> if any parameter error occurs.
    @since 12*/
    pub fn OH_ArkUI_PointerEvent_GetWindowX(event: *const ArkUI_UIInputEvent) -> f32;
    /** @brief Obtains the X coordinate of a specific touch point relative to the upper left corner of the current
    application window from a directional input event (such as a touch event, mouse event, or axis event).

    @param event Indicates the pointer to the current UI input event.
    @param pointerIndex Indicates the index of the target touch point in the multi-touch data list.
    @return Returns the X coordinate relative to the upper left corner of the current application window;
    returns <b>0.0f</b> if any parameter error occurs.
    @since 12*/
    pub fn OH_ArkUI_PointerEvent_GetWindowXByIndex(
        event: *const ArkUI_UIInputEvent,
        pointerIndex: u32,
    ) -> f32;
    /** @brief Obtains the Y coordinate relative to the upper left corner of the current application window from a
    directional input event (such as a touch event, mouse event, or axis event).

    @param event Indicates the pointer to the UI input event.
    @return Returns the Y coordinate relative to the upper left corner of the current application window;
    returns <b>0</b> if any parameter error occurs.
    @since 12*/
    pub fn OH_ArkUI_PointerEvent_GetWindowY(event: *const ArkUI_UIInputEvent) -> f32;
    /** @brief Obtains the Y coordinate of a specific touch point relative to the upper left corner of the current
    application window from a directional input event (such as a touch event, mouse event, or axis event).

    @param event Indicates the pointer to the current UI input event.
    @param pointerIndex Indicates the index of the target touch point in the multi-touch data list.
    @return Returns the Y coordinate relative to the upper left corner of the current application window;
    returns <b>0.0f</b> if any parameter error occurs.
    @since 12*/
    pub fn OH_ArkUI_PointerEvent_GetWindowYByIndex(
        event: *const ArkUI_UIInputEvent,
        pointerIndex: u32,
    ) -> f32;
    /** @brief Obtains the X coordinate relative to the upper left corner of the current screen from a directional input
    event (such as a touch event, mouse event, or axis event).

    @param event Indicates the pointer to the UI input event.
    @return Returns the X coordinate relative to the upper left corner of the current screen;
    returns <b>0</b> if any parameter error occurs.
    @since 12*/
    pub fn OH_ArkUI_PointerEvent_GetDisplayX(event: *const ArkUI_UIInputEvent) -> f32;
    /** @brief Obtains the X coordinate of a specific touch point relative to the upper left corner of the current screen
    from a directional input event (such as a touch event, mouse event, or axis event).

    @param event Indicates the pointer to the current UI input event.
    @param pointerIndex Indicates the index of the target touch point in the multi-touch data list.
    @return Returns the X coordinate relative to the upper left corner of the current screen;
    returns <b>0.0f</b> if any parameter error occurs.
    @since 12*/
    pub fn OH_ArkUI_PointerEvent_GetDisplayXByIndex(
        event: *const ArkUI_UIInputEvent,
        pointerIndex: u32,
    ) -> f32;
    /** @brief Obtains the Y coordinate relative to the upper left corner of the current screen from a directional input
    event (such as a touch event, mouse event, or axis event).

    @param event Indicates the pointer to the UI input event.
    @return Returns the Y coordinate relative to the upper left corner of the current screen;
    returns <b>0</b> if any parameter error occurs.
    @since 12*/
    pub fn OH_ArkUI_PointerEvent_GetDisplayY(event: *const ArkUI_UIInputEvent) -> f32;
    /** @brief Obtains the Y coordinate of a specific touch point relative to the upper left corner of the current screen
    from a directional input event (such as a touch event, mouse event, or axis event).

    @param event Indicates the pointer to the current UI input event.
    @param pointerIndex Indicates the index of the target touch point in the multi-touch data list.
    @return Returns the Y coordinate relative to the upper left corner of the current screen;
    returns <b>0.0f</b> if any parameter error occurs.
    @since 12*/
    pub fn OH_ArkUI_PointerEvent_GetDisplayYByIndex(
        event: *const ArkUI_UIInputEvent,
        pointerIndex: u32,
    ) -> f32;
    /** @brief Obtains the pressure applied to the touchscreen from a directional input event (for example, a touch event).

    @param event Indicates the pointer to the current UI input event.
    @param pointerIndex Indicates the index of the target touch point in the multi-touch data list.
    @return Returns the pressure applied to the touchscreen; returns <b>0.0f</b> if any parameter error occurs.
    @since 12*/
    pub fn OH_ArkUI_PointerEvent_GetPressure(
        event: *const ArkUI_UIInputEvent,
        pointerIndex: u32,
    ) -> f32;
    /** @brief Obtains the angle relative to the YZ plane from a directional input event (for example, a touch event).
    The value range is [-90, 90]. A positive value indicates a rightward tilt.

    @param event Indicates the pointer to the current UI input event.
    @param pointerIndex Indicates the index of the target touch point in the multi-touch data list.
    @return Returns the angle relative to the YZ plane.
    @since 12*/
    pub fn OH_ArkUI_PointerEvent_GetTiltX(
        event: *const ArkUI_UIInputEvent,
        pointerIndex: u32,
    ) -> f32;
    /** @brief Obtains the angle relative to the XZ plane from a directional input event (for example, a touch event).
    The value range is [-90, 90]. A positive value indicates a downward tilt.

    @param event Indicates the pointer to the current UI input event.
    @param pointerIndex Indicates the index of the target touch point in the multi-touch data list.
    @return Returns the angle relative to the XZ plane.
    @since 12*/
    pub fn OH_ArkUI_PointerEvent_GetTiltY(
        event: *const ArkUI_UIInputEvent,
        pointerIndex: u32,
    ) -> f32;
    /** @brief Obtains the width of the touch area from a directional input event (for example, a touch event).

    @param event Indicates the pointer to the current UI input event.
    @param pointerIndex Indicates the index of the target touch point in the multi-touch data list.
    @return Returns the width of the touch area.
    @since 12*/
    pub fn OH_ArkUI_PointerEvent_GetTouchAreaWidth(
        event: *const ArkUI_UIInputEvent,
        pointerIndex: u32,
    ) -> f32;
    /** @brief Obtains the height of the touch area from a directional input event (for example, a touch event).

    @param event Indicates the pointer to the current UI input event.
    @param pointerIndex Indicates the index of the target touch point in the multi-touch data list.
    @return Returns the height of the touch area.
    @since 12*/
    pub fn OH_ArkUI_PointerEvent_GetTouchAreaHeight(
        event: *const ArkUI_UIInputEvent,
        pointerIndex: u32,
    ) -> f32;
    /** @brief Obtains the number of historical events from a directional input event (such as a touch event, mouse event,
    or axis event).

    @param event Indicates the pointer to the current UI input event.
    @return Returns the number of historical events.
    @since 12*/
    pub fn OH_ArkUI_PointerEvent_GetHistorySize(event: *const ArkUI_UIInputEvent) -> u32;
    /** @brief Obtains the occurrence time of a historical event from a directional input event (such as a touch event,
    mouse event, or axis event).

    @param event Indicates the pointer to the current UI input event.
    @param historyIndex Indicates the index of the target historical event.
    @return Returns the time when the UI input event occurs; returns <b>0</b> if any parameter error occurs.
    @since 12*/
    pub fn OH_ArkUI_PointerEvent_GetHistoryEventTime(
        event: *const ArkUI_UIInputEvent,
        historyIndex: u32,
    ) -> i64;
    /** @brief Obtains the number of touch points in a specific historical event from a directional input event (such as
    a touch event, mouse event, or axis event).

    @param event Indicates the pointer to the current UI input event.
    @param historyIndex Indicates the index of the target historical event.
    @return Returns the number of touch points in the specified historical event
    @since 12*/
    pub fn OH_ArkUI_PointerEvent_GetHistoryPointerCount(
        event: *const ArkUI_UIInputEvent,
        historyIndex: u32,
    ) -> u32;
    /** @brief Obtains the ID of a touch point in a specific historical event from a directional input event (such as
    a touch event, mouse event, or axis event).

    @param event Indicates the pointer to the current UI input event.
    @param pointerIndex Indicates the index of the target touch point in the multi-touch data list.
    @param historyIndex Indicates the index of the target historical event.
    @return Returns the ID of the corresponding touch point in the specified historical event.
    @since 12*/
    pub fn OH_ArkUI_PointerEvent_GetHistoryPointerId(
        event: *const ArkUI_UIInputEvent,
        pointerIndex: u32,
        historyIndex: u32,
    ) -> i32;
    /** @brief Obtains the X coordinate of a specific touch point in a historical event relative to the upper left corner
    of the current component from a directional input event (such as a touch event, mouse event, or axis event).

    @param event Indicates the pointer to the current UI input event.
    @param pointerIndex Indicates the index of the target touch point in the multi-touch data list.
    @param historyIndex Indicates the index of the target historical event.
    @return Returns the X coordinate relative to the upper left corner of the current component;
    returns <b>0.0f</b> if any parameter error occurs.
    @since 12*/
    pub fn OH_ArkUI_PointerEvent_GetHistoryX(
        event: *const ArkUI_UIInputEvent,
        pointerIndex: u32,
        historyIndex: u32,
    ) -> f32;
    /** @brief Obtains the Y coordinate of a specific touch point in a historical event relative to the upper left corner
    of the current component from a directional input event (such as a touch event, mouse event, or axis event).

    @param event Indicates the pointer to the current UI input event.
    @param pointerIndex Indicates the index of the target touch point in the multi-touch data list.
    @param historyIndex Indicates the index of the target historical event.
    @return Returns the Y coordinate relative to the upper left corner of the current component;
    returns <b>0.0f</b> if any parameter error occurs.
    @since 12*/
    pub fn OH_ArkUI_PointerEvent_GetHistoryY(
        event: *const ArkUI_UIInputEvent,
        pointerIndex: u32,
        historyIndex: u32,
    ) -> f32;
    /** @brief Obtains the X coordinate of a specific touch point in a historical event relative to the upper left corner
    of the current application window from a directional input event (such as a touch event, mouse event, or axis event).

    @param event Indicates the pointer to the current UI input event.
    @param pointerIndex Indicates the index of the target touch point in the multi-touch data list.
    @param historyIndex Indicates the index of the target historical event.
    @return Returns the X coordinate relative to the upper left corner of the current application window;
    returns <b>0.0f</b> if any parameter error occurs.
    @since 12*/
    pub fn OH_ArkUI_PointerEvent_GetHistoryWindowX(
        event: *const ArkUI_UIInputEvent,
        pointerIndex: u32,
        historyIndex: u32,
    ) -> f32;
    /** @brief Obtains the Y coordinate of a specific touch point in a historical event relative to the upper left corner
    of the current application window from a directional input event (such as a touch event, mouse event, or axis event).

    @param event Indicates the pointer to the current UI input event.
    @param pointerIndex Indicates the index of the target touch point in the multi-touch data list.
    @param historyIndex Indicates the index of the target historical event.
    @return Returns the Y coordinate relative to the upper left corner of the current application window;
    returns <b>0.0f</b> if any parameter error occurs.
    @since 12*/
    pub fn OH_ArkUI_PointerEvent_GetHistoryWindowY(
        event: *const ArkUI_UIInputEvent,
        pointerIndex: u32,
        historyIndex: u32,
    ) -> f32;
    /** @brief Obtains the X coordinate of a specific touch point in a historical event relative to the upper left corner
    of the current screen from a directional input event (such as a touch event, mouse event, or axis event).

    @param event Indicates the pointer to the current UI input event.
    @param pointerIndex Indicates the index of the target touch point in the multi-touch data list.
    @param historyIndex Indicates the index of the target historical event.
    @return Returns the X coordinate relative to the upper left corner of the current screen;
    returns <b>0.0f</b> if any parameter error occurs.
    @since 12*/
    pub fn OH_ArkUI_PointerEvent_GetHistoryDisplayX(
        event: *const ArkUI_UIInputEvent,
        pointerIndex: u32,
        historyIndex: u32,
    ) -> f32;
    /** @brief Obtains the Y coordinate of a specific touch point in a historical event relative to the upper left corner
    of the current screen from a directional input event (such as a touch event, mouse event, or axis event).

    @param event Indicates the pointer to the current UI input event.
    @param pointerIndex Indicates the index of the target touch point in the multi-touch data list.
    @param historyIndex Indicates the index of the target historical event.
    @return Returns the Y coordinate relative to the upper left corner of the current screen;
    returns <b>0.0f</b> if any parameter error occurs.
    @since 12*/
    pub fn OH_ArkUI_PointerEvent_GetHistoryDisplayY(
        event: *const ArkUI_UIInputEvent,
        pointerIndex: u32,
        historyIndex: u32,
    ) -> f32;
    /** @brief Obtains the pressure applied to the touchscreen in a specific historical event from a directional input event
    (for example, a touch event)..

    @param event Indicates the pointer to the current UI input event.
    @param pointerIndex Indicates the index of the target touch point in the multi-touch data list.
    @param historyIndex Indicates the index of the target historical event.
    @return Returns the pressure applied to the touchscreen; returns <b>0.0f</b> if any parameter error occurs.
    @since 12*/
    pub fn OH_ArkUI_PointerEvent_GetHistoryPressure(
        event: *const ArkUI_UIInputEvent,
        pointerIndex: u32,
        historyIndex: u32,
    ) -> f32;
    /** @brief Obtains the angle relative to the YZ plane in a specific historical event from a directional input event
    (for example, a touch event). The value range is [-90, 90]. A positive value indicates a rightward tilt.

    @param event Indicates the pointer to the current UI input event.
    @param pointerIndex Indicates the index of the target touch point in the multi-touch data list.
    @param historyIndex Indicates the index of the target historical event.
    @return Returns the angle relative to the YZ plane.
    @since 12*/
    pub fn OH_ArkUI_PointerEvent_GetHistoryTiltX(
        event: *const ArkUI_UIInputEvent,
        pointerIndex: u32,
        historyIndex: u32,
    ) -> f32;
    /** @brief Obtains the angle relative to the XZ plane in a specific historical event from a directional input event
    (for example, a touch event). The value range is [-90, 90]. A positive value indicates a downward tilt.

    @param event Indicates the pointer to the current UI input event.
    @param pointerIndex Indicates the index of the target touch point in the multi-touch data list.
    @param historyIndex Indicates the index of the target historical event.
    @return Returns the angle relative to the XZ plane.
    @since 12*/
    pub fn OH_ArkUI_PointerEvent_GetHistoryTiltY(
        event: *const ArkUI_UIInputEvent,
        pointerIndex: u32,
        historyIndex: u32,
    ) -> f32;
    /** @brief Obtains the width of the touch area in a specific historical event from a directional input event
    (for example, a touch event).

    @param event Indicates the pointer to the current UI input event.
    @param pointerIndex Indicates the index of the target touch point in the multi-touch data list.
    @param historyIndex Indicates the index of the target historical event.
    @return Returns the width of the touch area.
    @since 12*/
    pub fn OH_ArkUI_PointerEvent_GetHistoryTouchAreaWidth(
        event: *const ArkUI_UIInputEvent,
        pointerIndex: u32,
        historyIndex: u32,
    ) -> f32;
    /** @brief Obtains the height of the touch area in a specific historical event from a directional input event
    (for example, a touch event).

    @param event Indicates the pointer to the current UI input event.
    @param pointerIndex Indicates the index of the target touch point in the multi-touch data list.
    @param historyIndex Indicates the index of the target historical event.
    @return Returns the height of the touch area.
    @since 12*/
    pub fn OH_ArkUI_PointerEvent_GetHistoryTouchAreaHeight(
        event: *const ArkUI_UIInputEvent,
        pointerIndex: u32,
        historyIndex: u32,
    ) -> f32;
    /** @brief Obtains the value of the vertical scroll axis for this axis event.

    @param event Indicates the pointer to the UI input event.
    @return Returns the value of the vertical scroll axis of the current axis event;
    returns <b>0</b> if any parameter error occurs.
    @since 12*/
    pub fn OH_ArkUI_AxisEvent_GetVerticalAxisValue(event: *const ArkUI_UIInputEvent) -> f64;
    /** @brief Obtains the value of the horizontal scroll axis for this axis event.

    @param event Indicates the pointer to the UI input event.
    @return Returns the value of the horizontal scroll axis of the current axis event;
    returns <b>0</b> if any parameter error occurs.
    @since 12*/
    pub fn OH_ArkUI_AxisEvent_GetHorizontalAxisValue(event: *const ArkUI_UIInputEvent) -> f64;
    /** @brief Obtains the scale value of the pinch axis for this axis event.

    @param event Indicates the pointer to the UI input event.
    @return Returns the scale value of the pinch axis of the current axis event;
    returns <b>0</b> if any parameter error occurs.
    @since 12*/
    pub fn OH_ArkUI_AxisEvent_GetPinchAxisScaleValue(event: *const ArkUI_UIInputEvent) -> f64;
    /** @brief Sets how the component behaves during hit testing.

    @param event Indicates the pointer to the current UI input event.
    @param mode Indicates how the component behaves during hit testing. The parameter type is {@link HitTestMode}.
    @return Returns the status code of the execution.
    @since 12*/
    pub fn OH_ArkUI_PointerEvent_SetInterceptHitTestMode(
        event: *const ArkUI_UIInputEvent,
        mode: HitTestMode,
    ) -> i32;
    /** @brief Get the value of the button type for mouse events.

    @param event Represents a pointer to the current UI input event.
    @return Return to the mouse button type, where <b>1</b> is the left button, <b>2</b> is the right button,
    <b>3</b> is the middle button, <b>4</b> is the back button, and <b>5</b> is the forward button.
    @since 12*/
    pub fn OH_ArkUI_MouseEvent_GetMouseButton(event: *const ArkUI_UIInputEvent) -> i32;
    /** @brief Get the value of the mouse action type for mouse events.

    @param event Represents a pointer to the current UI input event.
    @return Returns the type of mouse action, where <b>1</b> represents button pressed,
    <b>2</b> represents button released, and <b>3</b> represents mouse movement.
    @since 12*/
    pub fn OH_ArkUI_MouseEvent_GetMouseAction(event: *const ArkUI_UIInputEvent) -> i32;
    /** @brief Sets whether to prevent event bubbling.

    @param event Indicates the pointer to the current UI input event.
    @param stopPropagation Indicates whether the event is prevented from bubbling.
    @return Returns the status code of the execution. If 0 is returned, the setting is successful.
            If 401 is returned, the execution fails.
            The possible cause of the failure is that the event parameter is abnormal, such as a null pointer.
    @since 12*/
    pub fn OH_ArkUI_PointerEvent_SetStopPropagation(
        event: *const ArkUI_UIInputEvent,
        stopPropagation: bool,
    ) -> i32;
}
