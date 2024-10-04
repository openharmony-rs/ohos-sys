use crate::{OH_NativeXComponent_KeyAction, OH_NativeXComponent_KeyCode};
use keyboard_types::{Code, KeyState};

impl From<OH_NativeXComponent_KeyCode> for Code {
    fn from(code: OH_NativeXComponent_KeyCode) -> Self {
        use OH_NativeXComponent_KeyCode as OH_KeyCode;
        match code {
            // OH_KeyCode::KEY_BACK / code::Backquote ?
            OH_KeyCode::KEY_BACKSLASH => Code::Backslash,
            OH_KeyCode::KEY_LEFT_BRACKET => Code::BracketLeft,
            OH_KeyCode::KEY_RIGHT_BRACKET => Code::BracketRight,
            OH_KeyCode::KEY_COMMA => Code::Comma,
            OH_KeyCode::KEY_0 => Code::Digit0,
            OH_KeyCode::KEY_1 => Code::Digit1,
            OH_KeyCode::KEY_2 => Code::Digit2,
            OH_KeyCode::KEY_3 => Code::Digit3,
            OH_KeyCode::KEY_4 => Code::Digit4,
            OH_KeyCode::KEY_5 => Code::Digit5,
            OH_KeyCode::KEY_6 => Code::Digit6,
            OH_KeyCode::KEY_7 => Code::Digit7,
            OH_KeyCode::KEY_8 => Code::Digit8,
            OH_KeyCode::KEY_9 => Code::Digit9,

            OH_KeyCode::KEY_EQUALS => Code::Equal,
            OH_KeyCode::KEY_RO => Code::IntlRo,
            OH_KeyCode::KEY_YEN => Code::IntlYen,

            OH_KeyCode::KEY_A => Code::KeyA,
            OH_KeyCode::KEY_B => Code::KeyB,
            OH_KeyCode::KEY_C => Code::KeyC,
            OH_KeyCode::KEY_D => Code::KeyD,
            OH_KeyCode::KEY_E => Code::KeyE,
            OH_KeyCode::KEY_F => Code::KeyF,
            OH_KeyCode::KEY_G => Code::KeyG,
            OH_KeyCode::KEY_H => Code::KeyH,
            OH_KeyCode::KEY_I => Code::KeyI,
            OH_KeyCode::KEY_J => Code::KeyJ,
            OH_KeyCode::KEY_K => Code::KeyK,
            OH_KeyCode::KEY_L => Code::KeyL,
            OH_KeyCode::KEY_M => Code::KeyM,
            OH_KeyCode::KEY_N => Code::KeyN,
            OH_KeyCode::KEY_O => Code::KeyO,
            OH_KeyCode::KEY_P => Code::KeyP,
            OH_KeyCode::KEY_Q => Code::KeyQ,
            OH_KeyCode::KEY_R => Code::KeyR,
            OH_KeyCode::KEY_S => Code::KeyS,
            OH_KeyCode::KEY_T => Code::KeyT,
            OH_KeyCode::KEY_U => Code::KeyU,
            OH_KeyCode::KEY_V => Code::KeyV,
            OH_KeyCode::KEY_W => Code::KeyW,
            OH_KeyCode::KEY_X => Code::KeyX,
            OH_KeyCode::KEY_Y => Code::KeyY,
            OH_KeyCode::KEY_Z => Code::KeyZ,

            OH_KeyCode::KEY_MINUS => Code::Minus,
            OH_KeyCode::KEY_PERIOD => Code::Period,
            // Quote?
            OH_KeyCode::KEY_SEMICOLON => Code::Semicolon,
            OH_KeyCode::KEY_SLASH => Code::Slash,
            OH_KeyCode::KEY_ALT_LEFT => Code::AltLeft,
            OH_KeyCode::KEY_ALT_RIGHT => Code::AltRight,
            OH_KeyCode::KEY_DEL => Code::Backspace,
            OH_KeyCode::KEY_CAPS_LOCK => Code::CapsLock,
            // todo: double check
            OH_KeyCode::KEY_MENU => Code::ContextMenu,
            OH_KeyCode::KEY_CTRL_LEFT => Code::ControlLeft,
            OH_KeyCode::KEY_CTRL_RIGHT => Code::ControlRight,
            OH_KeyCode::KEY_ENTER => Code::Enter,
            OH_KeyCode::KEY_META_LEFT => Code::MetaLeft,
            OH_KeyCode::KEY_META_RIGHT => Code::MetaRight,
            OH_KeyCode::KEY_SHIFT_LEFT => Code::ShiftLeft,
            OH_KeyCode::KEY_SHIFT_RIGHT => Code::ShiftRight,
            OH_KeyCode::KEY_SPACE => Code::Space,
            OH_KeyCode::KEY_TAB => Code::Tab,
            // Code::Convert
            // Lang1-5
            // NonConvert
            OH_KeyCode::KEY_FORWARD_DEL => Code::Delete,
            // Code::End
            OH_KeyCode::KEY_HELP => Code::Help,
            OH_KeyCode::KEY_HOME => Code::Home,
            OH_KeyCode::KEY_INSERT => Code::Insert,
            OH_KeyCode::KEY_PAGE_DOWN => Code::PageDown,
            OH_KeyCode::KEY_PAGE_UP => Code::PageUp,
            // todo: not sure about dpad -> arrow
            OH_KeyCode::KEY_DPAD_DOWN => Code::ArrowDown,
            OH_KeyCode::KEY_DPAD_UP => Code::ArrowUp,
            OH_KeyCode::KEY_DPAD_LEFT => Code::ArrowLeft,
            OH_KeyCode::KEY_DPAD_RIGHT => Code::ArrowRight,

            OH_KeyCode::KEY_NUM_LOCK => Code::NumLock,
            OH_KeyCode::KEY_NUMPAD_0 => Code::Numpad0,
            OH_KeyCode::KEY_NUMPAD_1 => Code::Numpad1,
            OH_KeyCode::KEY_NUMPAD_2 => Code::Numpad2,
            OH_KeyCode::KEY_NUMPAD_3 => Code::Numpad3,
            OH_KeyCode::KEY_NUMPAD_4 => Code::Numpad4,
            OH_KeyCode::KEY_NUMPAD_5 => Code::Numpad5,
            OH_KeyCode::KEY_NUMPAD_6 => Code::Numpad6,
            OH_KeyCode::KEY_NUMPAD_7 => Code::Numpad7,
            OH_KeyCode::KEY_NUMPAD_8 => Code::Numpad8,
            OH_KeyCode::KEY_NUMPAD_9 => Code::Numpad9,
            OH_KeyCode::KEY_NUMPAD_ADD => Code::NumpadAdd,
            OH_KeyCode::KEY_NUMPAD_COMMA => Code::NumpadComma,
            OH_KeyCode::KEY_NUMPAD_DIVIDE => Code::NumpadDivide,
            OH_KeyCode::KEY_NUMPAD_DOT => Code::NumpadDecimal,
            OH_KeyCode::KEY_NUMPAD_ENTER => Code::NumpadEnter,
            OH_KeyCode::KEY_NUMPAD_EQUALS => Code::NumpadEqual,
            OH_KeyCode::KEY_NUMPAD_LEFT_PAREN => Code::NumpadParenLeft,
            OH_KeyCode::KEY_NUMPAD_RIGHT_PAREN => Code::NumpadParenRight,
            OH_KeyCode::KEY_NUMPAD_MULTIPLY => Code::NumpadMultiply,
            //OH_KeyCode::KEY_NUMPAD_PLUSMINUS => Code::Numpad
            OH_KeyCode::KEY_NUMPAD_SUBTRACT => Code::NumpadSubtract,

            OH_KeyCode::KEY_ESCAPE => Code::Escape,
            OH_KeyCode::KEY_FN => Code::Fn,
            // apparently no fn lock
            OH_KeyCode::KEY_PRINT => Code::PrintScreen,
            OH_KeyCode::KEY_SCROLL_LOCK => Code::ScrollLock,
            OH_KeyCode::KEY_PLAYPAUSE => Code::Pause,
            // Code Browser*
            OH_KeyCode::KEY_MEDIA_EJECT => Code::Eject,
            // Code Launch*
            OH_KeyCode::KEY_MEDIA_PLAY_PAUSE => Code::MediaPlayPause,
            OH_KeyCode::KEY_MEDIA_PAUSE => Code::MediaPause,
            OH_KeyCode::KEY_MEDIA_PLAY => Code::MediaPlay,
            OH_KeyCode::KEY_MEDIA_NEXT => Code::MediaTrackNext,
            OH_KeyCode::KEY_MEDIA_PREVIOUS => Code::MediaTrackPrevious,
            OH_KeyCode::KEY_MEDIA_STOP => Code::MediaStop,

            OH_KeyCode::KEY_POWER => Code::Power,
            OH_KeyCode::KEY_SLEEP => Code::Sleep,

            OH_KeyCode::KEY_VOLUME_DOWN => Code::AudioVolumeDown,
            OH_KeyCode::KEY_VOLUME_UP => Code::AudioVolumeUp,
            OH_KeyCode::KEY_VOLUME_MUTE => Code::AudioVolumeMute,

            OH_KeyCode::KEY_WAKEUP => Code::WakeUp,
            // Hyper, Super, Turbo, Abort, Resume
            OH_KeyCode::KEY_SUSPEND => Code::Suspend,
            OH_KeyCode::KEY_AGAIN => Code::Again,
            OH_KeyCode::KEY_COPY => Code::Copy,
            OH_KeyCode::KEY_CUT => Code::Cut,
            OH_KeyCode::KEY_FIND => Code::Find,
            OH_KeyCode::KEY_OPEN => Code::Open,
            OH_KeyCode::KEY_PASTE => Code::Paste,
            OH_KeyCode::KEY_PROPS => Code::Props,
            // Select
            OH_KeyCode::KEY_UNDO => Code::Undo,
            OH_KeyCode::KEY_HIRAGANA => Code::Hiragana,
            OH_KeyCode::KEY_KATAKANA => Code::Katakana,
            OH_KeyCode::KEY_F1 => Code::F1,
            OH_KeyCode::KEY_F2 => Code::F2,
            OH_KeyCode::KEY_F3 => Code::F3,
            OH_KeyCode::KEY_F4 => Code::F4,
            OH_KeyCode::KEY_F5 => Code::F5,
            OH_KeyCode::KEY_F6 => Code::F6,
            OH_KeyCode::KEY_F7 => Code::F7,
            OH_KeyCode::KEY_F8 => Code::F8,
            OH_KeyCode::KEY_F9 => Code::F9,
            OH_KeyCode::KEY_F10 => Code::F10,
            OH_KeyCode::KEY_F11 => Code::F11,
            OH_KeyCode::KEY_F12 => Code::F12,
            OH_KeyCode::KEY_F13 => Code::F13,
            OH_KeyCode::KEY_F14 => Code::F14,
            OH_KeyCode::KEY_F15 => Code::F15,
            OH_KeyCode::KEY_F16 => Code::F16,
            OH_KeyCode::KEY_F17 => Code::F17,
            OH_KeyCode::KEY_F18 => Code::F18,
            OH_KeyCode::KEY_F19 => Code::F19,
            OH_KeyCode::KEY_F20 => Code::F20,
            OH_KeyCode::KEY_F21 => Code::F21,
            OH_KeyCode::KEY_F22 => Code::F22,
            OH_KeyCode::KEY_F23 => Code::F23,
            OH_KeyCode::KEY_F24 => Code::F24,
            OH_KeyCode::KEY_BRIGHTNESS_DOWN => Code::BrightnessDown,
            OH_KeyCode::KEY_BRIGHTNESS_UP => Code::BrightnessUp,
            // DisplayToggleIntExt?
            // OH_KeyCode::KEY_KBD_LAYOUT_NEXT => Code::KeyboardLayoutSelect
            OH_KeyCode::KEY_MUTE => Code::MicrophoneMuteToggle,
            OH_KeyCode::KEY_APPSELECT => Code::SelectTask,
            // todo: verify
            OH_KeyCode::KEY_CYCLEWINDOWS => Code::ShowAllWindows,
            // OH_KeyCode::KEY_ZOOMIN => Code::
            OH_KeyCode::KEY_UNKNOWN => Code::Unidentified,
            _ => Code::Unidentified,
        }
    }
}

pub struct UnknownKeyState {}

impl TryFrom<OH_NativeXComponent_KeyAction> for KeyState {
    type Error = UnknownKeyState;

    fn try_from(value: OH_NativeXComponent_KeyAction) -> Result<Self, Self::Error> {
        use OH_NativeXComponent_KeyAction as KeyAction;
        match value {
            KeyAction::OH_NATIVEXCOMPONENT_KEY_ACTION_UP => Ok(KeyState::Up),
            KeyAction::OH_NATIVEXCOMPONENT_KEY_ACTION_DOWN => Ok(KeyState::Down),
            _ => Err(UnknownKeyState {}),
        }
    }
}
