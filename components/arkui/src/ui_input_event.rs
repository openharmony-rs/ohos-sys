#[allow(unused_imports)]
mod ui_input_event_ffi;
pub use ui_input_event_ffi::*;

mod ui_input_event_anon_enums_ffi;

pub use anon_enums::*;
mod anon_enums {
    use super::ui_input_event_anon_enums_ffi as bindgen_enums;
    use std::ffi::c_uint;

    #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
    #[repr(transparent)]
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub struct TouchEventAction(c_uint);

    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    impl TouchEventAction {
        pub const CANCEL: Self = Self(bindgen_enums::UI_TOUCH_EVENT_ACTION_CANCEL.0);
        pub const DOWN: Self = Self(bindgen_enums::UI_TOUCH_EVENT_ACTION_DOWN.0);
        pub const MOVE: Self = Self(bindgen_enums::UI_TOUCH_EVENT_ACTION_MOVE.0);
        pub const UP: Self = Self(bindgen_enums::UI_TOUCH_EVENT_ACTION_UP.0);
    }

    #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
    #[repr(transparent)]
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub struct TouchEventToolType(c_uint);

    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    impl TouchEventToolType {
        pub const UNKNOWN: Self = Self(bindgen_enums::UI_INPUT_EVENT_TOOL_TYPE_UNKNOWN.0);
        pub const FINGER: Self = Self(bindgen_enums::UI_INPUT_EVENT_TOOL_TYPE_FINGER.0);
        pub const PEN: Self = Self(bindgen_enums::UI_INPUT_EVENT_TOOL_TYPE_PEN.0);
        pub const MOUSE: Self = Self(bindgen_enums::UI_INPUT_EVENT_TOOL_TYPE_MOUSE.0);
        pub const TOUCHPAD: Self = Self(bindgen_enums::UI_INPUT_EVENT_TOOL_TYPE_TOUCHPAD.0);
        pub const JOYSTICK: Self = Self(bindgen_enums::UI_INPUT_EVENT_TOOL_TYPE_JOYSTICK.0);
    }

    #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
    #[repr(transparent)]
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub struct InputEventSourceType(c_uint);

    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    impl InputEventSourceType {
        pub const UNKNOWN: Self = Self(bindgen_enums::UI_INPUT_EVENT_SOURCE_TYPE_UNKNOWN.0);
        pub const MOUSE: Self = Self(bindgen_enums::UI_INPUT_EVENT_SOURCE_TYPE_MOUSE.0);
        pub const TOUCH_SCREEN: Self =
            Self(bindgen_enums::UI_INPUT_EVENT_SOURCE_TYPE_TOUCH_SCREEN.0);
    }

    #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
    #[repr(transparent)]
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub struct MouseEventAction(c_uint);

    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    impl MouseEventAction {
        pub const UNKNOWN: Self = Self(bindgen_enums::UI_MOUSE_EVENT_ACTION_UNKNOWN.0);
        pub const PRESS: Self = Self(bindgen_enums::UI_MOUSE_EVENT_ACTION_PRESS.0);
        pub const RELEASE: Self = Self(bindgen_enums::UI_MOUSE_EVENT_ACTION_RELEASE.0);
        pub const MOVE: Self = Self(bindgen_enums::UI_MOUSE_EVENT_ACTION_MOVE.0);
        /// Cancel.
        ///
        /// Available since API-level: 18
        #[cfg(feature = "api-18")]
        #[cfg_attr(docsrs, doc(cfg(feature = "api-18")))]
        pub const CANCEL: Self = Self(bindgen_enums::UI_MOUSE_EVENT_ACTION_CANCEL.0);
    }
}
