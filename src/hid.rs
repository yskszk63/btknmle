#![warn(clippy::all)]

#[derive(thiserror::Error, Debug)]
#[error("no mapping found {0:?}")]
pub struct NoMappingFound(btknmle_input::KeyCodes);

impl From<btknmle_input::KeyCodes> for NoMappingFound {
    fn from(v: btknmle_input::KeyCodes) -> Self {
        Self(v)
    }
}

macro_rules! kbd_usage_id {
    (
        $(#[$attrs:meta])*
        $vis:vis enum $name:ident: $fromty:ty {
            type Error = $errty:ty;
            $(
                $(#[$fattr:meta])*
                $fname:ident : $fval:literal $( => $from:ident)?,
            )*
        }
    ) => {
        $(#[$attrs])*
        pub enum $name {
            $(
                $(#[$fattr])*
                $fname,
            )*
        }

        impl std::convert::TryFrom<$fromty> for $name {
            type Error = $errty;

            fn try_from(v: $fromty) -> Result<Self, Self::Error> {
                match v {
                    $(
                        $(<$fromty>::$from => Ok(Self::$fname),)?
                    )*
                    x => Err(Self::Error::from(x)),
                }
            }
        }

        impl From<$name> for u8 {
            fn from(v: $name) -> Self {
                match v {
                    $(
                        $name::$fname => $fval,
                    )*
                }
            }
        }
    }
}

kbd_usage_id! {
    #[allow(non_camel_case_types)]
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub enum KeyboardUsageId: btknmle_input::KeyCodes {
        type Error = NoMappingFound;

        KEY_ERROR_ROLLOVER: 0x01,
        KEY_POST_FAIL: 0x02,
        KEY_ERROR_UNDEFINED: 0x03,
        KEY_A: 0x04 => KEY_A,
        KEY_B: 0x05 => KEY_B,
        KEY_C: 0x06 => KEY_C,
        KEY_D: 0x07 => KEY_D,
        KEY_E: 0x08 => KEY_E,
        KEY_F: 0x09 => KEY_F,
        KEY_G: 0x0A => KEY_G,
        KEY_H: 0x0B => KEY_H,
        KEY_I: 0x0C => KEY_I,
        KEY_J: 0x0D => KEY_J,
        KEY_K: 0x0E => KEY_K,
        KEY_L: 0x0F => KEY_L,
        KEY_M: 0x10 => KEY_M,
        KEY_N: 0x11 => KEY_N,
        KEY_O: 0x12 => KEY_O,
        KEY_P: 0x13 => KEY_P,
        KEY_Q: 0x14 => KEY_Q,
        KEY_R: 0x15 => KEY_R,
        KEY_S: 0x16 => KEY_S,
        KEY_T: 0x17 => KEY_T,
        KEY_U: 0x18 => KEY_U,
        KEY_V: 0x19 => KEY_V,
        KEY_W: 0x1A => KEY_W,
        KEY_X: 0x1B => KEY_X,
        KEY_Y: 0x1C => KEY_Y,
        KEY_Z: 0x1D => KEY_Z,
        KEY_1: 0x1E => KEY_1,
        KEY_2: 0x1F => KEY_2,
        KEY_3: 0x20 => KEY_3,
        KEY_4: 0x21 => KEY_4,
        KEY_5: 0x22 => KEY_5,
        KEY_6: 0x23 => KEY_6,
        KEY_7: 0x24 => KEY_7,
        KEY_8: 0x25 => KEY_8,
        KEY_9: 0x26 => KEY_9,
        KEY_0: 0x27 => KEY_0,
        KEY_ENTER: 0x28 => KEY_ENTER,
        KEY_ESC: 0x29 => KEY_ESC,
        KEY_DELETE: 0x2A => KEY_BACKSPACE,
        KEY_TAB: 0x2B => KEY_TAB,
        KEY_SPACE: 0x2C => KEY_SPACE,
        KEY_MINUS: 0x2D => KEY_MINUS,
        KEY_EQUAL: 0x2E => KEY_EQUAL,
        KEY_LEFTBRACE: 0x2F => KEY_LEFTBRACE,
        KEY_RIGHTBRACE: 0x30 => KEY_RIGHTBRACE,
        KEY_BACKSLASH: 0x31 => KEY_BACKSLASH,
        KEY_NON_US_SHARP: 0x32,
        KEY_SEMICOLON: 0x33 => KEY_SEMICOLON,
        KEY_APOSTROPHE: 0x34 => KEY_APOSTROPHE,
        KEY_GRAVE: 0x35 => KEY_GRAVE,
        KEY_COMMA: 0x36 => KEY_COMMA,
        KEY_DOT: 0x37 => KEY_DOT,
        KEY_SLASH: 0x38 => KEY_SLASH,
        KEY_CAPSLOCK: 0x39 => KEY_CAPSLOCK,
        KEY_F1: 0x3A => KEY_F1,
        KEY_F2: 0x3B => KEY_F2,
        KEY_F3: 0x3C => KEY_F3,
        KEY_F4: 0x3D => KEY_F4,
        KEY_F5: 0x3E => KEY_F5,
        KEY_F6: 0x3F => KEY_F6,
        KEY_F7: 0x40 => KEY_F7,
        KEY_F8: 0x41 => KEY_F8,
        KEY_F9: 0x42 => KEY_F9,
        KEY_F10: 0x43 => KEY_F10,
        KEY_F11: 0x44 => KEY_F11,
        KEY_F12: 0x45 => KEY_F12,
        KEY_SYSRQ: 0x46 => KEY_SYSRQ,
        KEY_SCROLLLOCK: 0x47 => KEY_SCROLLLOCK,
        KEY_PAUSE: 0x48 => KEY_PAUSE,
        KEY_INSERT: 0x49 => KEY_INSERT,
        KEY_HOME: 0x4A => KEY_HOME,
        KEY_PAGEUP: 0x4B => KEY_PAGEUP,
        KEY_DELETE_FORWARD: 0x4C => KEY_DELETE,
        KEY_END: 0x4D => KEY_END,
        KEY_PAGEDOWN: 0x4E => KEY_PAGEDOWN,
        KEY_RIGHT: 0x4F => KEY_RIGHT,
        KEY_LEFT: 0x50 => KEY_LEFT,
        KEY_DOWN: 0x51 => KEY_DOWN,
        KEY_UP: 0x52 => KEY_UP,
        KEY_NUMLOCK: 0x53 => KEY_NUMLOCK,
        KEY_KPSLASH: 0x54 => KEY_KPSLASH,
        KEY_KPASTERISK: 0x55 => KEY_KPASTERISK,
        KEY_KPMINUS: 0x56 => KEY_KPMINUS,
        KEY_KPPLUS: 0x57 => KEY_KPPLUS,
        KEY_KPENTER: 0x58 => KEY_KPENTER,
        KEY_KP1: 0x59 => KEY_KP1,
        KEY_KP2: 0x5A => KEY_KP2,
        KEY_KP3: 0x5B => KEY_KP3,
        KEY_KP4: 0x5C => KEY_KP4,
        KEY_KP5: 0x5D => KEY_KP5,
        KEY_KP6: 0x5E => KEY_KP6,
        KEY_KP7: 0x5F => KEY_KP7,
        KEY_KP8: 0x60 => KEY_KP8,
        KEY_KP9: 0x61 => KEY_KP9,
        KEY_KP0: 0x62 => KEY_KP0,
        KEY_KPDOT: 0x63 => KEY_KPDOT,
        KEY_102ND: 0x64 => KEY_102ND,
        KEY_COMPOSE: 0x65 => KEY_COMPOSE,
        KEY_POWER: 0x66 => KEY_POWER,
        KEY_KPEQUAL: 0x67 => KEY_KPEQUAL,
        KEY_F13: 0x68 => KEY_F13,
        KEY_F14: 0x69 => KEY_F14,
        KEY_F15: 0x6A => KEY_F15,
        KEY_F16: 0x6B => KEY_F16,
        KEY_F17: 0x6C => KEY_F17,
        KEY_F18: 0x6D => KEY_F18,
        KEY_F19: 0x6E => KEY_F19,
        KEY_F20: 0x6F => KEY_F20,
        KEY_F21: 0x70 => KEY_F21,
        KEY_F22: 0x71 => KEY_F22,
        KEY_F23: 0x72 => KEY_F23,
        KEY_F24: 0x73 => KEY_F24,
        KEY_OPEN: 0x74 => KEY_OPEN,
        KEY_HELP: 0x75 => KEY_HELP,
        KEY_PROPS: 0x76 => KEY_PROPS,
        KEY_FRONT: 0x77 => KEY_FRONT,
        KEY_STOP: 0x78 => KEY_STOP,
        KEY_AGAIN: 0x79 => KEY_AGAIN,
        KEY_UNDO: 0x7A => KEY_UNDO,
        KEY_CUT: 0x7B => KEY_CUT,
        KEY_COPY: 0x7C => KEY_COPY,
        KEY_PASTE: 0x7D => KEY_PASTE,
        KEY_FIND: 0x7E => KEY_FIND,
        KEY_MUTE: 0x7F => KEY_MUTE,
        KEY_VOLUMEUP: 0x80 => KEY_VOLUMEUP,
        KEY_VOLUMEDOWN: 0x81 => KEY_VOLUMEDOWN,
        KEY_LOCKING_CAPSLOCK: 0x82,
        KEY_LOCKING_NUMLOCK: 0x83,
        KEY_LOCKING_SCROLLOCK: 0x84,
        KEY_KPCOMMA: 0x85 => KEY_KPCOMMA,
        KEY_KPEQUALSIGN: 0x86,
        KEY_RO: 0x87 => KEY_RO,
        KEY_KATAKANAHIRAGANA: 0x88 => KEY_KATAKANAHIRAGANA,
        KEY_YEN: 0x89 => KEY_YEN,
        KEY_HENKAN: 0x8A => KEY_HENKAN,
        KEY_MUHENKAN: 0x8B => KEY_MUHENKAN,
        KEY_KPJPCOMMA: 0x8C => KEY_KPJPCOMMA,
        KEY_INTERNATIONAL7: 0x8D,
        KEY_INTERNATIONAL8: 0x8E,
        KEY_INTERNATIONAL9: 0x8F,
        KEY_HANGUEL: 0x90,
        KEY_HANJA: 0x91 => KEY_HANJA,
        KEY_KATAKANA: 0x92 => KEY_KATAKANA,
        KEY_HIRAGANA: 0x93 => KEY_HIRAGANA,
        KEY_ZENKAKUHANKAKU: 0x94 => KEY_ZENKAKUHANKAKU,
        KEY_LANG6: 0x95,
        KEY_LANG7: 0x96,
        KEY_LANG8: 0x97,
        KEY_LANG9: 0x98,
        KEY_ALTERNATE_ERASE: 0x99,
        KEY_ATTENTION: 0x9A,
        KEY_CANCEL: 0x9B => KEY_CANCEL,
        // KEY_DELETE: 0x9C => KEY_DELETE,
        KEY_PRIOR: 0x9D,
        KEY_RETURN: 0x9E,
        KEY_SEPARATOR: 0x9F,
        KEY_OUT: 0xA0,
        // KEY_OPEN: 0xA1 => KEY_OPEN,
        KEY_CLEAR_AGAIN: 0xA2,
        KEY_SRSEL_PROPS: 0xA3,
        KEY_EXSEL: 0xA4,
        KEY_KP00: 0xB0,
        KEY_KP000: 0xB1,
        KEY_THOUSANDS_SEPARATOR: 0xB2,
        KEY_DECIMAL_SEPARATOR: 0xB3,
        KEY_CURRENCY_UNIT: 0xB4,
        KEY_CURRENCY_SUBUNIT: 0xB5,
        KEY_KPLEFTPAREN: 0xB6 => KEY_KPLEFTPAREN,
        KEY_KPRIGHTPAREN: 0xB7 => KEY_KPRIGHTPAREN,
        KEY_KPLEFTBRACE: 0xB8,
        KEY_KPRIGHTBRACE: 0xB9,
        KEY_KPTAB: 0xBA,
        KEY_KPBACKSPACE: 0xBB,
        KEY_KPA: 0xBC,
        KEY_KPB: 0xBD,
        KEY_KPC: 0xBE,
        KEY_KPD: 0xBF,
        KEY_KPE: 0xC0,
        KEY_KPF: 0xC1,
        KEY_KPXOR: 0xC2,
        KEY_KPCIRCUMEFLEX_ACCENT: 0xC3,
        KEY_KPPERCENT: 0xC4,
        KEY_KPLESS_THAN: 0xC5,
        KEY_KPGREATER_THAN: 0xC6,
        KEY_KPAMP: 0xC7,
        KEY_KPDOUBLE_AMP: 0xC8,
        KEY_KPOR: 0xC9,
        KEY_KPDOUBLE_OR: 0xCA,
        KEY_KPCOLON: 0xCB,
        KEY_KPSHARP: 0xCC,
        KEY_KPSPACE: 0xCD,
        KEY_KPATMARK: 0xCE,
        KEY_KPEXCLAMATION: 0xCF,
        KEY_KPMEMORY_STORE: 0xD0,
        KEY_KPMEMORY_RECALL: 0xD1,
        KEY_KPMEMORY_CLEAR: 0xD2,
        KEY_KPMEMORY_ADD: 0xD3,
        KEY_KPMEMORY_SUBTRACT: 0xD4,
        KEY_KPMEMORY_MULTIPLE: 0xD5,
        KEY_KPMEMORY_DIVIDE: 0xD6,
        KEY_KPMEMORY_PLUSMINUS: 0xD7,
        KEY_KPCLEAR: 0xD8,
        KEY_KPCLEARENTRY: 0xD9,
        KEY_KPBINARY: 0xDA,
        KEY_KPOCTAL: 0xDB,
        KEY_KPDECIMAL: 0xDC,
        KEY_KPHEX: 0xDD,
        KEY_LEFT_CTRL: 0xE0 => KEY_LEFTCTRL,
        KEY_LEFT_SHIFT: 0xE1 => KEY_LEFTSHIFT,
        KEY_LEFT_ALT: 0xE2 => KEY_LEFTALT,
        KEY_LEFT_GUI: 0xE3 => KEY_LEFTMETA,
        KEY_RIGHT_CTRL: 0xE4 => KEY_RIGHTCTRL,
        KEY_RIGHT_SHIFT: 0xE5 => KEY_RIGHTSHIFT,
        KEY_RIGHT_ALT: 0xE6 => KEY_RIGHTALT,
        KEY_RIGHT_GUI: 0xE7 => KEY_RIGHTMETA,
        KEY_PLAYPAUSE: 0xE8 => KEY_PLAYPAUSE,
        KEY_STOPCD: 0xE9 => KEY_STOPCD,
        KEY_PREVIOUSSONG: 0xEA => KEY_PREVIOUSSONG,
        KEY_NEXTSONG: 0xEB => KEY_NEXTSONG,
        KEY_EJECTCD: 0xEC => KEY_EJECTCD,
        // KEY_VOLUMEUP: 0xED => KEY_VOLUMEUP,
        // KEY_VOLUMEDOWN: 0xEE => KEY_VOLUMEDOWN,
        // KEY_MUTE: 0xEF => KEY_MUTE,
        KEY_WWW: 0xF0 => KEY_WWW,
        KEY_BACK: 0xF1 => KEY_BACK,
        KEY_FORWARD: 0xF2 => KEY_FORWARD,
        // KEY_STOP: 0xF3 => KEY_STOP,
        // KEY_FIND: 0xF4 => KEY_FIND,
        KEY_SCROLLUP: 0xF5 => KEY_SCROLLUP,
        KEY_SCROLLDOWN: 0xF6 => KEY_SCROLLDOWN,
        KEY_EDIT: 0xF7 => KEY_EDIT,
        KEY_SLEEP: 0xF8 => KEY_SLEEP,
        KEY_COFFEE: 0xF9 => KEY_COFFEE,
        KEY_REFRESH: 0xFA => KEY_REFRESH,
        KEY_CALC: 0xFB => KEY_CALC,
    }
}
