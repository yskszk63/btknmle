#![allow(non_camel_case_types)]
use btknmle_input::KeyCodes;

const KEY_ERROR_ROLLOVER: u8 = 0x01;
const KEY_POST_FAIL: u8 = 0x02;
const KEY_ERROR_UNDEFINED: u8 = 0x03;
const KEY_A: u8 = 0x04;
const KEY_B: u8 = 0x05;
const KEY_C: u8 = 0x06;
const KEY_D: u8 = 0x07;
const KEY_E: u8 = 0x08;
const KEY_F: u8 = 0x09;
const KEY_G: u8 = 0x0A;
const KEY_H: u8 = 0x0B;
const KEY_I: u8 = 0x0C;
const KEY_J: u8 = 0x0D;
const KEY_K: u8 = 0x0E;
const KEY_L: u8 = 0x0F;
const KEY_M: u8 = 0x10;
const KEY_N: u8 = 0x11;
const KEY_O: u8 = 0x12;
const KEY_P: u8 = 0x13;
const KEY_Q: u8 = 0x14;
const KEY_R: u8 = 0x15;
const KEY_S: u8 = 0x16;
const KEY_T: u8 = 0x17;
const KEY_U: u8 = 0x18;
const KEY_V: u8 = 0x19;
const KEY_W: u8 = 0x1A;
const KEY_X: u8 = 0x1B;
const KEY_Y: u8 = 0x1C;
const KEY_Z: u8 = 0x1D;
const KEY_1: u8 = 0x1E;
const KEY_2: u8 = 0x1F;
const KEY_3: u8 = 0x20;
const KEY_4: u8 = 0x21;
const KEY_5: u8 = 0x22;
const KEY_6: u8 = 0x23;
const KEY_7: u8 = 0x24;
const KEY_8: u8 = 0x25;
const KEY_9: u8 = 0x26;
const KEY_0: u8 = 0x27;
const KEY_ENTER: u8 = 0x28;
const KEY_ESC: u8 = 0x29;
const KEY_DELETE: u8 = 0x2A;
const KEY_TAB: u8 = 0x2B;
const KEY_SPACE: u8 = 0x2C;
const KEY_MINUS: u8 = 0x2D;
const KEY_EQUAL: u8 = 0x2E;
const KEY_LEFTBRACE: u8 = 0x2F;
const KEY_RIGHTBRACE: u8 = 0x30;
const KEY_BACKSLASH: u8 = 0x31;
const KEY_NON_US_SHARP: u8 = 0x32;
const KEY_SEMICOLON: u8 = 0x33;
const KEY_APOSTROPHE: u8 = 0x34;
const KEY_GRAVE: u8 = 0x35;
const KEY_COMMA: u8 = 0x36;
const KEY_DOT: u8 = 0x37;
const KEY_SLASH: u8 = 0x38;
const KEY_CAPSLOCK: u8 = 0x39;
const KEY_F1: u8 = 0x3A;
const KEY_F2: u8 = 0x3B;
const KEY_F3: u8 = 0x3C;
const KEY_F4: u8 = 0x3D;
const KEY_F5: u8 = 0x3E;
const KEY_F6: u8 = 0x3F;
const KEY_F7: u8 = 0x40;
const KEY_F8: u8 = 0x41;
const KEY_F9: u8 = 0x42;
const KEY_F10: u8 = 0x43;
const KEY_F11: u8 = 0x44;
const KEY_F12: u8 = 0x45;
const KEY_SYSRQ: u8 = 0x46;
const KEY_SCROLLLOCK: u8 = 0x47;
const KEY_PAUSE: u8 = 0x48;
const KEY_INSERT: u8 = 0x49;
const KEY_HOME: u8 = 0x4A;
const KEY_PAGEUP: u8 = 0x4B;
const KEY_DELETE_FORWARD: u8 = 0x4C;
const KEY_END: u8 = 0x4D;
const KEY_PAGEDOWN: u8 = 0x4E;
const KEY_RIGHT: u8 = 0x4F;
const KEY_LEFT: u8 = 0x50;
const KEY_DOWN: u8 = 0x51;
const KEY_UP: u8 = 0x52;
const KEY_NUMLOCK: u8 = 0x53;
const KEY_KPSLASH: u8 = 0x54;
const KEY_KPASTERISK: u8 = 0x55;
const KEY_KPMINUS: u8 = 0x56;
const KEY_KPPLUS: u8 = 0x57;
const KEY_KPENTER: u8 = 0x58;
const KEY_KP1: u8 = 0x59;
const KEY_KP2: u8 = 0x5A;
const KEY_KP3: u8 = 0x5B;
const KEY_KP4: u8 = 0x5C;
const KEY_KP5: u8 = 0x5D;
const KEY_KP6: u8 = 0x5E;
const KEY_KP7: u8 = 0x5F;
const KEY_KP8: u8 = 0x60;
const KEY_KP9: u8 = 0x61;
const KEY_KP0: u8 = 0x62;
const KEY_KPDOT: u8 = 0x63;
const KEY_102ND: u8 = 0x64;
const KEY_COMPOSE: u8 = 0x65;
const KEY_POWER: u8 = 0x66;
const KEY_KPEQUAL: u8 = 0x67;
const KEY_F13: u8 = 0x68;
const KEY_F14: u8 = 0x69;
const KEY_F15: u8 = 0x6A;
const KEY_F16: u8 = 0x6B;
const KEY_F17: u8 = 0x6C;
const KEY_F18: u8 = 0x6D;
const KEY_F19: u8 = 0x6E;
const KEY_F20: u8 = 0x6F;
const KEY_F21: u8 = 0x70;
const KEY_F22: u8 = 0x71;
const KEY_F23: u8 = 0x72;
const KEY_F24: u8 = 0x73;
const KEY_OPEN: u8 = 0x74;
const KEY_HELP: u8 = 0x75;
const KEY_PROPS: u8 = 0x76;
const KEY_FRONT: u8 = 0x77;
const KEY_STOP: u8 = 0x78;
const KEY_AGAIN: u8 = 0x79;
const KEY_UNDO: u8 = 0x7A;
const KEY_CUT: u8 = 0x7B;
const KEY_COPY: u8 = 0x7C;
const KEY_PASTE: u8 = 0x7D;
const KEY_FIND: u8 = 0x7E;
const KEY_MUTE: u8 = 0x7F;
const KEY_VOLUMEUP: u8 = 0x80;
const KEY_VOLUMEDOWN: u8 = 0x81;
const KEY_LOCKING_CAPSLOCK: u8 = 0x82;
const KEY_LOCKING_NUMLOCK: u8 = 0x83;
const KEY_LOCKING_SCROLLOCK: u8 = 0x84;
const KEY_KPCOMMA: u8 = 0x85;
const KEY_KPEQUALSIGN: u8 = 0x86;
const KEY_RO: u8 = 0x87;
const KEY_KATAKANAHIRAGANA: u8 = 0x88;
const KEY_YEN: u8 = 0x89;
const KEY_HENKAN: u8 = 0x8A;
const KEY_MUHENKAN: u8 = 0x8B;
const KEY_KPJPCOMMA: u8 = 0x8C;
const KEY_INTERNATIONAL7: u8 = 0x8D;
const KEY_INTERNATIONAL8: u8 = 0x8E;
const KEY_INTERNATIONAL9: u8 = 0x8F;
const KEY_HANGUEL: u8 = 0x90;
const KEY_HANJA: u8 = 0x91;
const KEY_KATAKANA: u8 = 0x92;
const KEY_HIRAGANA: u8 = 0x93;
const KEY_ZENKAKUHANKAKU: u8 = 0x94;
const KEY_LANG6: u8 = 0x95;
const KEY_LANG7: u8 = 0x96;
const KEY_LANG8: u8 = 0x97;
const KEY_LANG9: u8 = 0x98;
const KEY_ALTERNATE_ERASE: u8 = 0x99;
const KEY_ATTENTION: u8 = 0x9A;
const KEY_CANCEL: u8 = 0x9B;
const KEY_PRIOR: u8 = 0x9D;
const KEY_RETURN: u8 = 0x9E;
const KEY_SEPARATOR: u8 = 0x9F;
const KEY_OUT: u8 = 0xA0;
const KEY_CLEAR_AGAIN: u8 = 0xA2;
const KEY_SRSEL_PROPS: u8 = 0xA3;
const KEY_EXSEL: u8 = 0xA4;
const KEY_KP00: u8 = 0xB0;
const KEY_KP000: u8 = 0xB1;
const KEY_THOUSANDS_SEPARATOR: u8 = 0xB2;
const KEY_DECIMAL_SEPARATOR: u8 = 0xB3;
const KEY_CURRENCY_UNIT: u8 = 0xB4;
const KEY_CURRENCY_SUBUNIT: u8 = 0xB5;
const KEY_KPLEFTPAREN: u8 = 0xB6;
const KEY_KPRIGHTPAREN: u8 = 0xB7;
const KEY_KPLEFTBRACE: u8 = 0xB8;
const KEY_KPRIGHTBRACE: u8 = 0xB9;
const KEY_KPTAB: u8 = 0xBA;
const KEY_KPBACKSPACE: u8 = 0xBB;
const KEY_KPA: u8 = 0xBC;
const KEY_KPB: u8 = 0xBD;
const KEY_KPC: u8 = 0xBE;
const KEY_KPD: u8 = 0xBF;
const KEY_KPE: u8 = 0xC0;
const KEY_KPF: u8 = 0xC1;
const KEY_KPXOR: u8 = 0xC2;
const KEY_KPCIRCUMEFLEX_ACCENT: u8 = 0xC3;
const KEY_KPPERCENT: u8 = 0xC4;
const KEY_KPLESS_THAN: u8 = 0xC5;
const KEY_KPGREATER_THAN: u8 = 0xC6;
const KEY_KPAMP: u8 = 0xC7;
const KEY_KPDOUBLE_AMP: u8 = 0xC8;
const KEY_KPOR: u8 = 0xC9;
const KEY_KPDOUBLE_OR: u8 = 0xCA;
const KEY_KPCOLON: u8 = 0xCB;
const KEY_KPSHARP: u8 = 0xCC;
const KEY_KPSPACE: u8 = 0xCD;
const KEY_KPATMARK: u8 = 0xCE;
const KEY_KPEXCLAMATION: u8 = 0xCF;
const KEY_KPMEMORY_STORE: u8 = 0xD0;
const KEY_KPMEMORY_RECALL: u8 = 0xD1;
const KEY_KPMEMORY_CLEAR: u8 = 0xD2;
const KEY_KPMEMORY_ADD: u8 = 0xD3;
const KEY_KPMEMORY_SUBTRACT: u8 = 0xD4;
const KEY_KPMEMORY_MULTIPLE: u8 = 0xD5;
const KEY_KPMEMORY_DIVIDE: u8 = 0xD6;
const KEY_KPMEMORY_PLUSMINUS: u8 = 0xD7;
const KEY_KPCLEAR: u8 = 0xD8;
const KEY_KPCLEARENTRY: u8 = 0xD9;
const KEY_KPBINARY: u8 = 0xDA;
const KEY_KPOCTAL: u8 = 0xDB;
const KEY_KPDECIMAL: u8 = 0xDC;
const KEY_KPHEX: u8 = 0xDD;
const KEY_LEFT_CTRL: u8 = 0xE0;
const KEY_LEFT_SHIFT: u8 = 0xE1;
const KEY_LEFT_ALT: u8 = 0xE2;
const KEY_LEFT_GUI: u8 = 0xE3;
const KEY_RIGHT_CTRL: u8 = 0xE4;
const KEY_RIGHT_SHIFT: u8 = 0xE5;
const KEY_RIGHT_ALT: u8 = 0xE6;
const KEY_RIGHT_GUI: u8 = 0xE7;
const KEY_PLAYPAUSE: u8 = 0xE8;
const KEY_STOPCD: u8 = 0xE9;
const KEY_PREVIOUSSONG: u8 = 0xEA;
const KEY_NEXTSONG: u8 = 0xEB;
const KEY_EJECTCD: u8 = 0xEC;
const KEY_WWW: u8 = 0xF0;
const KEY_BACK: u8 = 0xF1;
const KEY_FORWARD: u8 = 0xF2;
const KEY_SCROLLUP: u8 = 0xF5;
const KEY_SCROLLDOWN: u8 = 0xF6;
const KEY_EDIT: u8 = 0xF7;
const KEY_SLEEP: u8 = 0xF8;
const KEY_COFFEE: u8 = 0xF9;
const KEY_REFRESH: u8 = 0xFA;
const KEY_CALC: u8 = 0xFB;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum KeyboardUsageId {
    KEY_ERROR_ROLLOVER,
    KEY_POST_FAIL,
    KEY_ERROR_UNDEFINED,
    KEY_A,
    KEY_B,
    KEY_C,
    KEY_D,
    KEY_E,
    KEY_F,
    KEY_G,
    KEY_H,
    KEY_I,
    KEY_J,
    KEY_K,
    KEY_L,
    KEY_M,
    KEY_N,
    KEY_O,
    KEY_P,
    KEY_Q,
    KEY_R,
    KEY_S,
    KEY_T,
    KEY_U,
    KEY_V,
    KEY_W,
    KEY_X,
    KEY_Y,
    KEY_Z,
    KEY_1,
    KEY_2,
    KEY_3,
    KEY_4,
    KEY_5,
    KEY_6,
    KEY_7,
    KEY_8,
    KEY_9,
    KEY_0,
    KEY_ENTER,
    KEY_ESC,
    KEY_DELETE,
    KEY_TAB,
    KEY_SPACE,
    KEY_MINUS,
    KEY_EQUAL,
    KEY_LEFTBRACE,
    KEY_RIGHTBRACE,
    KEY_BACKSLASH,
    KEY_NON_US_SHARP,
    KEY_SEMICOLON,
    KEY_APOSTROPHE,
    KEY_GRAVE,
    KEY_COMMA,
    KEY_DOT,
    KEY_SLASH,
    KEY_CAPSLOCK,
    KEY_F1,
    KEY_F2,
    KEY_F3,
    KEY_F4,
    KEY_F5,
    KEY_F6,
    KEY_F7,
    KEY_F8,
    KEY_F9,
    KEY_F10,
    KEY_F11,
    KEY_F12,
    KEY_SYSRQ,
    KEY_SCROLLLOCK,
    KEY_PAUSE,
    KEY_INSERT,
    KEY_HOME,
    KEY_PAGEUP,
    KEY_DELETE_FORWARD,
    KEY_END,
    KEY_PAGEDOWN,
    KEY_RIGHT,
    KEY_LEFT,
    KEY_DOWN,
    KEY_UP,
    KEY_NUMLOCK,
    KEY_KPSLASH,
    KEY_KPASTERISK,
    KEY_KPMINUS,
    KEY_KPPLUS,
    KEY_KPENTER,
    KEY_KP1,
    KEY_KP2,
    KEY_KP3,
    KEY_KP4,
    KEY_KP5,
    KEY_KP6,
    KEY_KP7,
    KEY_KP8,
    KEY_KP9,
    KEY_KP0,
    KEY_KPDOT,
    KEY_102ND,
    KEY_COMPOSE,
    KEY_POWER,
    KEY_KPEQUAL,
    KEY_F13,
    KEY_F14,
    KEY_F15,
    KEY_F16,
    KEY_F17,
    KEY_F18,
    KEY_F19,
    KEY_F20,
    KEY_F21,
    KEY_F22,
    KEY_F23,
    KEY_F24,
    KEY_OPEN,
    KEY_HELP,
    KEY_PROPS,
    KEY_FRONT,
    KEY_STOP,
    KEY_AGAIN,
    KEY_UNDO,
    KEY_CUT,
    KEY_COPY,
    KEY_PASTE,
    KEY_FIND,
    KEY_MUTE,
    KEY_VOLUMEUP,
    KEY_VOLUMEDOWN,
    KEY_LOCKING_CAPSLOCK,
    KEY_LOCKING_NUMLOCK,
    KEY_LOCKING_SCROLLOCK,
    KEY_KPCOMMA,
    KEY_KPEQUALSIGN,
    KEY_RO,
    KEY_KATAKANAHIRAGANA,
    KEY_YEN,
    KEY_HENKAN,
    KEY_MUHENKAN,
    KEY_KPJPCOMMA,
    KEY_INTERNATIONAL7,
    KEY_INTERNATIONAL8,
    KEY_INTERNATIONAL9,
    KEY_HANGUEL,
    KEY_HANJA,
    KEY_KATAKANA,
    KEY_HIRAGANA,
    KEY_ZENKAKUHANKAKU,
    KEY_LANG6,
    KEY_LANG7,
    KEY_LANG8,
    KEY_LANG9,
    KEY_ALTERNATE_ERASE,
    KEY_ATTENTION,
    KEY_CANCEL,
    KEY_PRIOR,
    KEY_RETURN,
    KEY_SEPARATOR,
    KEY_OUT,
    KEY_CLEAR_AGAIN,
    KEY_SRSEL_PROPS,
    KEY_EXSEL,
    KEY_KP00,
    KEY_KP000,
    KEY_THOUSANDS_SEPARATOR,
    KEY_DECIMAL_SEPARATOR,
    KEY_CURRENCY_UNIT,
    KEY_CURRENCY_SUBUNIT,
    KEY_KPLEFTPAREN,
    KEY_KPRIGHTPAREN,
    KEY_KPLEFTBRACE,
    KEY_KPRIGHTBRACE,
    KEY_KPTAB,
    KEY_KPBACKSPACE,
    KEY_KPA,
    KEY_KPB,
    KEY_KPC,
    KEY_KPD,
    KEY_KPE,
    KEY_KPF,
    KEY_KPXOR,
    KEY_KPCIRCUMEFLEX_ACCENT,
    KEY_KPPERCENT,
    KEY_KPLESS_THAN,
    KEY_KPGREATER_THAN,
    KEY_KPAMP,
    KEY_KPDOUBLE_AMP,
    KEY_KPOR,
    KEY_KPDOUBLE_OR,
    KEY_KPCOLON,
    KEY_KPSHARP,
    KEY_KPSPACE,
    KEY_KPATMARK,
    KEY_KPEXCLAMATION,
    KEY_KPMEMORY_STORE,
    KEY_KPMEMORY_RECALL,
    KEY_KPMEMORY_CLEAR,
    KEY_KPMEMORY_ADD,
    KEY_KPMEMORY_SUBTRACT,
    KEY_KPMEMORY_MULTIPLE,
    KEY_KPMEMORY_DIVIDE,
    KEY_KPMEMORY_PLUSMINUS,
    KEY_KPCLEAR,
    KEY_KPCLEARENTRY,
    KEY_KPBINARY,
    KEY_KPOCTAL,
    KEY_KPDECIMAL,
    KEY_KPHEX,
    KEY_LEFT_CTRL,
    KEY_LEFT_SHIFT,
    KEY_LEFT_ALT,
    KEY_LEFT_GUI,
    KEY_RIGHT_CTRL,
    KEY_RIGHT_SHIFT,
    KEY_RIGHT_ALT,
    KEY_RIGHT_GUI,
    KEY_PLAYPAUSE,
    KEY_STOPCD,
    KEY_PREVIOUSSONG,
    KEY_NEXTSONG,
    KEY_EJECTCD,
    KEY_WWW,
    KEY_BACK,
    KEY_FORWARD,
    KEY_SCROLLUP,
    KEY_SCROLLDOWN,
    KEY_EDIT,
    KEY_SLEEP,
    KEY_COFFEE,
    KEY_REFRESH,
    KEY_CALC,
    Unknown(u8),
}

impl KeyboardUsageId {
    pub fn from_keycodes(v: KeyCodes) -> Option<Self> {
        Some(match v {
            //KeyCodes::KEY_ERROR_ROLLOVER => Self::KEY_ERROR_ROLLOVER,
            //KeyCodes::KEY_POST_FAIL => Self::KEY_POST_FAIL,
            //KeyCodes::KEY_ERROR_UNDEFINED => Self::KEY_ERROR_UNDEFINED,
            KeyCodes::KEY_A => Self::KEY_A,
            KeyCodes::KEY_B => Self::KEY_B,
            KeyCodes::KEY_C => Self::KEY_C,
            KeyCodes::KEY_D => Self::KEY_D,
            KeyCodes::KEY_E => Self::KEY_E,
            KeyCodes::KEY_F => Self::KEY_F,
            KeyCodes::KEY_G => Self::KEY_G,
            KeyCodes::KEY_H => Self::KEY_H,
            KeyCodes::KEY_I => Self::KEY_I,
            KeyCodes::KEY_J => Self::KEY_J,
            KeyCodes::KEY_K => Self::KEY_K,
            KeyCodes::KEY_L => Self::KEY_L,
            KeyCodes::KEY_M => Self::KEY_M,
            KeyCodes::KEY_N => Self::KEY_N,
            KeyCodes::KEY_O => Self::KEY_O,
            KeyCodes::KEY_P => Self::KEY_P,
            KeyCodes::KEY_Q => Self::KEY_Q,
            KeyCodes::KEY_R => Self::KEY_R,
            KeyCodes::KEY_S => Self::KEY_S,
            KeyCodes::KEY_T => Self::KEY_T,
            KeyCodes::KEY_U => Self::KEY_U,
            KeyCodes::KEY_V => Self::KEY_V,
            KeyCodes::KEY_W => Self::KEY_W,
            KeyCodes::KEY_X => Self::KEY_X,
            KeyCodes::KEY_Y => Self::KEY_Y,
            KeyCodes::KEY_Z => Self::KEY_Z,
            KeyCodes::KEY_1 => Self::KEY_1,
            KeyCodes::KEY_2 => Self::KEY_2,
            KeyCodes::KEY_3 => Self::KEY_3,
            KeyCodes::KEY_4 => Self::KEY_4,
            KeyCodes::KEY_5 => Self::KEY_5,
            KeyCodes::KEY_6 => Self::KEY_6,
            KeyCodes::KEY_7 => Self::KEY_7,
            KeyCodes::KEY_8 => Self::KEY_8,
            KeyCodes::KEY_9 => Self::KEY_9,
            KeyCodes::KEY_0 => Self::KEY_0,
            KeyCodes::KEY_ENTER => Self::KEY_ENTER,
            KeyCodes::KEY_ESC => Self::KEY_ESC,
            KeyCodes::KEY_BACKSPACE => Self::KEY_DELETE,
            KeyCodes::KEY_TAB => Self::KEY_TAB,
            KeyCodes::KEY_SPACE => Self::KEY_SPACE,
            KeyCodes::KEY_MINUS => Self::KEY_MINUS,
            KeyCodes::KEY_EQUAL => Self::KEY_EQUAL,
            KeyCodes::KEY_LEFTBRACE => Self::KEY_LEFTBRACE,
            KeyCodes::KEY_RIGHTBRACE => Self::KEY_RIGHTBRACE,
            KeyCodes::KEY_BACKSLASH => Self::KEY_BACKSLASH,
            //KeyCodes::KEY_NON_US_SHARP => Self::KEY_NON_US_SHARP,
            KeyCodes::KEY_SEMICOLON => Self::KEY_SEMICOLON,
            KeyCodes::KEY_APOSTROPHE => Self::KEY_APOSTROPHE,
            KeyCodes::KEY_GRAVE => Self::KEY_GRAVE,
            KeyCodes::KEY_COMMA => Self::KEY_COMMA,
            KeyCodes::KEY_DOT => Self::KEY_DOT,
            KeyCodes::KEY_SLASH => Self::KEY_SLASH,
            KeyCodes::KEY_CAPSLOCK => Self::KEY_CAPSLOCK,
            KeyCodes::KEY_F1 => Self::KEY_F1,
            KeyCodes::KEY_F2 => Self::KEY_F2,
            KeyCodes::KEY_F3 => Self::KEY_F3,
            KeyCodes::KEY_F4 => Self::KEY_F4,
            KeyCodes::KEY_F5 => Self::KEY_F5,
            KeyCodes::KEY_F6 => Self::KEY_F6,
            KeyCodes::KEY_F7 => Self::KEY_F7,
            KeyCodes::KEY_F8 => Self::KEY_F8,
            KeyCodes::KEY_F9 => Self::KEY_F9,
            KeyCodes::KEY_F10 => Self::KEY_F10,
            KeyCodes::KEY_F11 => Self::KEY_F11,
            KeyCodes::KEY_F12 => Self::KEY_F12,
            KeyCodes::KEY_SYSRQ => Self::KEY_SYSRQ,
            KeyCodes::KEY_SCROLLLOCK => Self::KEY_SCROLLLOCK,
            KeyCodes::KEY_PAUSE => Self::KEY_PAUSE,
            KeyCodes::KEY_INSERT => Self::KEY_INSERT,
            KeyCodes::KEY_HOME => Self::KEY_HOME,
            KeyCodes::KEY_PAGEUP => Self::KEY_PAGEUP,
            KeyCodes::KEY_DELETE => Self::KEY_DELETE_FORWARD,
            KeyCodes::KEY_END => Self::KEY_END,
            KeyCodes::KEY_PAGEDOWN => Self::KEY_PAGEDOWN,
            KeyCodes::KEY_RIGHT => Self::KEY_RIGHT,
            KeyCodes::KEY_LEFT => Self::KEY_LEFT,
            KeyCodes::KEY_DOWN => Self::KEY_DOWN,
            KeyCodes::KEY_UP => Self::KEY_UP,
            KeyCodes::KEY_NUMLOCK => Self::KEY_NUMLOCK,
            KeyCodes::KEY_KPSLASH => Self::KEY_KPSLASH,
            KeyCodes::KEY_KPASTERISK => Self::KEY_KPASTERISK,
            KeyCodes::KEY_KPMINUS => Self::KEY_KPMINUS,
            KeyCodes::KEY_KPPLUS => Self::KEY_KPPLUS,
            KeyCodes::KEY_KPENTER => Self::KEY_KPENTER,
            KeyCodes::KEY_KP1 => Self::KEY_KP1,
            KeyCodes::KEY_KP2 => Self::KEY_KP2,
            KeyCodes::KEY_KP3 => Self::KEY_KP3,
            KeyCodes::KEY_KP4 => Self::KEY_KP4,
            KeyCodes::KEY_KP5 => Self::KEY_KP5,
            KeyCodes::KEY_KP6 => Self::KEY_KP6,
            KeyCodes::KEY_KP7 => Self::KEY_KP7,
            KeyCodes::KEY_KP8 => Self::KEY_KP8,
            KeyCodes::KEY_KP9 => Self::KEY_KP9,
            KeyCodes::KEY_KP0 => Self::KEY_KP0,
            KeyCodes::KEY_KPDOT => Self::KEY_KPDOT,
            KeyCodes::KEY_102ND => Self::KEY_102ND,
            KeyCodes::KEY_COMPOSE => Self::KEY_COMPOSE,
            KeyCodes::KEY_POWER => Self::KEY_POWER,
            KeyCodes::KEY_KPEQUAL => Self::KEY_KPEQUAL,
            KeyCodes::KEY_F13 => Self::KEY_F13,
            KeyCodes::KEY_F14 => Self::KEY_F14,
            KeyCodes::KEY_F15 => Self::KEY_F15,
            KeyCodes::KEY_F16 => Self::KEY_F16,
            KeyCodes::KEY_F17 => Self::KEY_F17,
            KeyCodes::KEY_F18 => Self::KEY_F18,
            KeyCodes::KEY_F19 => Self::KEY_F19,
            KeyCodes::KEY_F20 => Self::KEY_F20,
            KeyCodes::KEY_F21 => Self::KEY_F21,
            KeyCodes::KEY_F22 => Self::KEY_F22,
            KeyCodes::KEY_F23 => Self::KEY_F23,
            KeyCodes::KEY_F24 => Self::KEY_F24,
            KeyCodes::KEY_OPEN => Self::KEY_OPEN,
            KeyCodes::KEY_HELP => Self::KEY_HELP,
            KeyCodes::KEY_PROPS => Self::KEY_PROPS,
            KeyCodes::KEY_FRONT => Self::KEY_FRONT,
            KeyCodes::KEY_STOP => Self::KEY_STOP,
            KeyCodes::KEY_AGAIN => Self::KEY_AGAIN,
            KeyCodes::KEY_UNDO => Self::KEY_UNDO,
            KeyCodes::KEY_CUT => Self::KEY_CUT,
            KeyCodes::KEY_COPY => Self::KEY_COPY,
            KeyCodes::KEY_PASTE => Self::KEY_PASTE,
            KeyCodes::KEY_FIND => Self::KEY_FIND,
            KeyCodes::KEY_MUTE => Self::KEY_MUTE,
            KeyCodes::KEY_VOLUMEUP => Self::KEY_VOLUMEUP,
            KeyCodes::KEY_VOLUMEDOWN => Self::KEY_VOLUMEDOWN,
            //KeyCodes::KEY_LOCKING_CAPSLOCK => Self::KEY_LOCKING_CAPSLOCK,
            //KeyCodes::KEY_LOCKING_NUMLOCK => Self::KEY_LOCKING_NUMLOCK,
            //KeyCodes::KEY_LOCKING_SCROLLOCK => Self::KEY_LOCKING_SCROLLOCK,
            KeyCodes::KEY_KPCOMMA => Self::KEY_KPCOMMA,
            //KeyCodes::KEY_KPEQUALSIGN => Self::KEY_KPEQUALSIGN,
            KeyCodes::KEY_RO => Self::KEY_RO,
            KeyCodes::KEY_KATAKANAHIRAGANA => Self::KEY_KATAKANAHIRAGANA,
            KeyCodes::KEY_YEN => Self::KEY_YEN,
            KeyCodes::KEY_HENKAN => Self::KEY_HENKAN,
            KeyCodes::KEY_MUHENKAN => Self::KEY_MUHENKAN,
            KeyCodes::KEY_KPJPCOMMA => Self::KEY_KPJPCOMMA,
            //KeyCodes::KEY_INTERNATIONAL7 => Self::KEY_INTERNATIONAL7,
            //KeyCodes::KEY_INTERNATIONAL8 => Self::KEY_INTERNATIONAL8,
            //KeyCodes::KEY_INTERNATIONAL9 => Self::KEY_INTERNATIONAL9,
            //KeyCodes::KEY_HANGUEL => Self::KEY_HANGUEL,
            KeyCodes::KEY_HANJA => Self::KEY_HANJA,
            KeyCodes::KEY_KATAKANA => Self::KEY_KATAKANA,
            KeyCodes::KEY_HIRAGANA => Self::KEY_HIRAGANA,
            KeyCodes::KEY_ZENKAKUHANKAKU => Self::KEY_ZENKAKUHANKAKU,
            //KeyCodes::KEY_LANG6 => Self::KEY_LANG6,
            //KeyCodes::KEY_LANG7 => Self::KEY_LANG7,
            //KeyCodes::KEY_LANG8 => Self::KEY_LANG8,
            //KeyCodes::KEY_LANG9 => Self::KEY_LANG9,
            //KeyCodes::KEY_ALTERNATE_ERASE => Self::KEY_ALTERNATE_ERASE,
            //KeyCodes::KEY_ATTENTION => Self::KEY_ATTENTION,
            KeyCodes::KEY_CANCEL => Self::KEY_CANCEL,
            //KeyCodes::KEY_PRIOR => Self::KEY_PRIOR,
            //KeyCodes::KEY_RETURN => Self::KEY_RETURN,
            //KeyCodes::KEY_SEPARATOR => Self::KEY_SEPARATOR,
            //KeyCodes::KEY_OUT => Self::KEY_OUT,
            //KeyCodes::KEY_CLEAR_AGAIN => Self::KEY_CLEAR_AGAIN,
            //KeyCodes::KEY_SRSEL_PROPS => Self::KEY_SRSEL_PROPS,
            //KeyCodes::KEY_EXSEL => Self::KEY_EXSEL,
            //KeyCodes::KEY_KP00 => Self::KEY_KP00,
            //KeyCodes::KEY_KP000 => Self::KEY_KP000,
            //KeyCodes::KEY_THOUSANDS_SEPARATOR => Self::KEY_THOUSANDS_SEPARATOR,
            //KeyCodes::KEY_DECIMAL_SEPARATOR => Self::KEY_DECIMAL_SEPARATOR,
            //KeyCodes::KEY_CURRENCY_UNIT => Self::KEY_CURRENCY_UNIT,
            //KeyCodes::KEY_CURRENCY_SUBUNIT => Self::KEY_CURRENCY_SUBUNIT,
            KeyCodes::KEY_KPLEFTPAREN => Self::KEY_KPLEFTPAREN,
            KeyCodes::KEY_KPRIGHTPAREN => Self::KEY_KPRIGHTPAREN,
            //KeyCodes::KEY_KPLEFTBRACE => Self::KEY_KPLEFTBRACE,
            //KeyCodes::KEY_KPRIGHTBRACE => Self::KEY_KPRIGHTBRACE,
            //KeyCodes::KEY_KPTAB => Self::KEY_KPTAB,
            //KeyCodes::KEY_KPBACKSPACE => Self::KEY_KPBACKSPACE,
            //KeyCodes::KEY_KPA => Self::KEY_KPA,
            //KeyCodes::KEY_KPB => Self::KEY_KPB,
            //KeyCodes::KEY_KPC => Self::KEY_KPC,
            //KeyCodes::KEY_KPD => Self::KEY_KPD,
            //KeyCodes::KEY_KPE => Self::KEY_KPE,
            //KeyCodes::KEY_KPF => Self::KEY_KPF,
            //KeyCodes::KEY_KPXOR => Self::KEY_KPXOR,
            //KeyCodes::KEY_KPCIRCUMEFLEX_ACCENT => Self::KEY_KPCIRCUMEFLEX_ACCENT,
            //KeyCodes::KEY_KPPERCENT => Self::KEY_KPPERCENT,
            //KeyCodes::KEY_KPLESS_THAN => Self::KEY_KPLESS_THAN,
            //KeyCodes::KEY_KPGREATER_THAN => Self::KEY_KPGREATER_THAN,
            //KeyCodes::KEY_KPAMP => Self::KEY_KPAMP,
            //KeyCodes::KEY_KPDOUBLE_AMP => Self::KEY_KPDOUBLE_AMP,
            //KeyCodes::KEY_KPOR => Self::KEY_KPOR,
            //KeyCodes::KEY_KPDOUBLE_OR => Self::KEY_KPDOUBLE_OR,
            //KeyCodes::KEY_KPCOLON => Self::KEY_KPCOLON,
            //KeyCodes::KEY_KPSHARP => Self::KEY_KPSHARP,
            //KeyCodes::KEY_KPSPACE => Self::KEY_KPSPACE,
            //KeyCodes::KEY_KPATMARK => Self::KEY_KPATMARK,
            //KeyCodes::KEY_KPEXCLAMATION => Self::KEY_KPEXCLAMATION,
            //KeyCodes::KEY_KPMEMORY_STORE => Self::KEY_KPMEMORY_STORE,
            //KeyCodes::KEY_KPMEMORY_RECALL => Self::KEY_KPMEMORY_RECALL,
            //KeyCodes::KEY_KPMEMORY_CLEAR => Self::KEY_KPMEMORY_CLEAR,
            //KeyCodes::KEY_KPMEMORY_ADD => Self::KEY_KPMEMORY_ADD,
            //KeyCodes::KEY_KPMEMORY_SUBTRACT => Self::KEY_KPMEMORY_SUBTRACT,
            //KeyCodes::KEY_KPMEMORY_MULTIPLE => Self::KEY_KPMEMORY_MULTIPLE,
            //KeyCodes::KEY_KPMEMORY_DIVIDE => Self::KEY_KPMEMORY_DIVIDE,
            //KeyCodes::KEY_KPMEMORY_PLUSMINUS => Self::KEY_KPMEMORY_PLUSMINUS,
            //KeyCodes::KEY_KPCLEAR => Self::KEY_KPCLEAR,
            //KeyCodes::KEY_KPCLEARENTRY => Self::KEY_KPCLEARENTRY,
            //KeyCodes::KEY_KPBINARY => Self::KEY_KPBINARY,
            //KeyCodes::KEY_KPOCTAL => Self::KEY_KPOCTAL,
            //KeyCodes::KEY_KPDECIMAL => Self::KEY_KPDECIMAL,
            //KeyCodes::KEY_KPHEX => Self::KEY_KPHEX,
            KeyCodes::KEY_LEFTCTRL => Self::KEY_LEFT_CTRL,
            KeyCodes::KEY_LEFTSHIFT => Self::KEY_LEFT_SHIFT,
            KeyCodes::KEY_LEFTALT => Self::KEY_LEFT_ALT,
            KeyCodes::KEY_LEFTMETA => Self::KEY_LEFT_GUI,
            KeyCodes::KEY_RIGHTCTRL => Self::KEY_RIGHT_CTRL,
            KeyCodes::KEY_RIGHTSHIFT => Self::KEY_RIGHT_SHIFT,
            KeyCodes::KEY_RIGHTALT => Self::KEY_RIGHT_ALT,
            KeyCodes::KEY_RIGHTMETA => Self::KEY_RIGHT_GUI,
            KeyCodes::KEY_PLAYPAUSE => Self::KEY_PLAYPAUSE,
            KeyCodes::KEY_STOPCD => Self::KEY_STOPCD,
            KeyCodes::KEY_PREVIOUSSONG => Self::KEY_PREVIOUSSONG,
            KeyCodes::KEY_NEXTSONG => Self::KEY_NEXTSONG,
            KeyCodes::KEY_EJECTCD => Self::KEY_EJECTCD,
            KeyCodes::KEY_WWW => Self::KEY_WWW,
            KeyCodes::KEY_BACK => Self::KEY_BACK,
            KeyCodes::KEY_FORWARD => Self::KEY_FORWARD,
            KeyCodes::KEY_SCROLLUP => Self::KEY_SCROLLUP,
            KeyCodes::KEY_SCROLLDOWN => Self::KEY_SCROLLDOWN,
            KeyCodes::KEY_EDIT => Self::KEY_EDIT,
            KeyCodes::KEY_SLEEP => Self::KEY_SLEEP,
            KeyCodes::KEY_COFFEE => Self::KEY_COFFEE,
            KeyCodes::KEY_REFRESH => Self::KEY_REFRESH,
            KeyCodes::KEY_CALC => Self::KEY_CALC,
            _ => return None,
        })
    }
}

impl From<KeyboardUsageId> for u8 {
    fn from(v: KeyboardUsageId) -> Self {
        match v {
            KeyboardUsageId::KEY_ERROR_ROLLOVER => KEY_ERROR_ROLLOVER,
            KeyboardUsageId::KEY_POST_FAIL => KEY_POST_FAIL,
            KeyboardUsageId::KEY_ERROR_UNDEFINED => KEY_ERROR_UNDEFINED,
            KeyboardUsageId::KEY_A => KEY_A,
            KeyboardUsageId::KEY_B => KEY_B,
            KeyboardUsageId::KEY_C => KEY_C,
            KeyboardUsageId::KEY_D => KEY_D,
            KeyboardUsageId::KEY_E => KEY_E,
            KeyboardUsageId::KEY_F => KEY_F,
            KeyboardUsageId::KEY_G => KEY_G,
            KeyboardUsageId::KEY_H => KEY_H,
            KeyboardUsageId::KEY_I => KEY_I,
            KeyboardUsageId::KEY_J => KEY_J,
            KeyboardUsageId::KEY_K => KEY_K,
            KeyboardUsageId::KEY_L => KEY_L,
            KeyboardUsageId::KEY_M => KEY_M,
            KeyboardUsageId::KEY_N => KEY_N,
            KeyboardUsageId::KEY_O => KEY_O,
            KeyboardUsageId::KEY_P => KEY_P,
            KeyboardUsageId::KEY_Q => KEY_Q,
            KeyboardUsageId::KEY_R => KEY_R,
            KeyboardUsageId::KEY_S => KEY_S,
            KeyboardUsageId::KEY_T => KEY_T,
            KeyboardUsageId::KEY_U => KEY_U,
            KeyboardUsageId::KEY_V => KEY_V,
            KeyboardUsageId::KEY_W => KEY_W,
            KeyboardUsageId::KEY_X => KEY_X,
            KeyboardUsageId::KEY_Y => KEY_Y,
            KeyboardUsageId::KEY_Z => KEY_Z,
            KeyboardUsageId::KEY_1 => KEY_1,
            KeyboardUsageId::KEY_2 => KEY_2,
            KeyboardUsageId::KEY_3 => KEY_3,
            KeyboardUsageId::KEY_4 => KEY_4,
            KeyboardUsageId::KEY_5 => KEY_5,
            KeyboardUsageId::KEY_6 => KEY_6,
            KeyboardUsageId::KEY_7 => KEY_7,
            KeyboardUsageId::KEY_8 => KEY_8,
            KeyboardUsageId::KEY_9 => KEY_9,
            KeyboardUsageId::KEY_0 => KEY_0,
            KeyboardUsageId::KEY_ENTER => KEY_ENTER,
            KeyboardUsageId::KEY_ESC => KEY_ESC,
            KeyboardUsageId::KEY_DELETE => KEY_DELETE,
            KeyboardUsageId::KEY_TAB => KEY_TAB,
            KeyboardUsageId::KEY_SPACE => KEY_SPACE,
            KeyboardUsageId::KEY_MINUS => KEY_MINUS,
            KeyboardUsageId::KEY_EQUAL => KEY_EQUAL,
            KeyboardUsageId::KEY_LEFTBRACE => KEY_LEFTBRACE,
            KeyboardUsageId::KEY_RIGHTBRACE => KEY_RIGHTBRACE,
            KeyboardUsageId::KEY_BACKSLASH => KEY_BACKSLASH,
            KeyboardUsageId::KEY_NON_US_SHARP => KEY_NON_US_SHARP,
            KeyboardUsageId::KEY_SEMICOLON => KEY_SEMICOLON,
            KeyboardUsageId::KEY_APOSTROPHE => KEY_APOSTROPHE,
            KeyboardUsageId::KEY_GRAVE => KEY_GRAVE,
            KeyboardUsageId::KEY_COMMA => KEY_COMMA,
            KeyboardUsageId::KEY_DOT => KEY_DOT,
            KeyboardUsageId::KEY_SLASH => KEY_SLASH,
            KeyboardUsageId::KEY_CAPSLOCK => KEY_CAPSLOCK,
            KeyboardUsageId::KEY_F1 => KEY_F1,
            KeyboardUsageId::KEY_F2 => KEY_F2,
            KeyboardUsageId::KEY_F3 => KEY_F3,
            KeyboardUsageId::KEY_F4 => KEY_F4,
            KeyboardUsageId::KEY_F5 => KEY_F5,
            KeyboardUsageId::KEY_F6 => KEY_F6,
            KeyboardUsageId::KEY_F7 => KEY_F7,
            KeyboardUsageId::KEY_F8 => KEY_F8,
            KeyboardUsageId::KEY_F9 => KEY_F9,
            KeyboardUsageId::KEY_F10 => KEY_F10,
            KeyboardUsageId::KEY_F11 => KEY_F11,
            KeyboardUsageId::KEY_F12 => KEY_F12,
            KeyboardUsageId::KEY_SYSRQ => KEY_SYSRQ,
            KeyboardUsageId::KEY_SCROLLLOCK => KEY_SCROLLLOCK,
            KeyboardUsageId::KEY_PAUSE => KEY_PAUSE,
            KeyboardUsageId::KEY_INSERT => KEY_INSERT,
            KeyboardUsageId::KEY_HOME => KEY_HOME,
            KeyboardUsageId::KEY_PAGEUP => KEY_PAGEUP,
            KeyboardUsageId::KEY_DELETE_FORWARD => KEY_DELETE_FORWARD,
            KeyboardUsageId::KEY_END => KEY_END,
            KeyboardUsageId::KEY_PAGEDOWN => KEY_PAGEDOWN,
            KeyboardUsageId::KEY_RIGHT => KEY_RIGHT,
            KeyboardUsageId::KEY_LEFT => KEY_LEFT,
            KeyboardUsageId::KEY_DOWN => KEY_DOWN,
            KeyboardUsageId::KEY_UP => KEY_UP,
            KeyboardUsageId::KEY_NUMLOCK => KEY_NUMLOCK,
            KeyboardUsageId::KEY_KPSLASH => KEY_KPSLASH,
            KeyboardUsageId::KEY_KPASTERISK => KEY_KPASTERISK,
            KeyboardUsageId::KEY_KPMINUS => KEY_KPMINUS,
            KeyboardUsageId::KEY_KPPLUS => KEY_KPPLUS,
            KeyboardUsageId::KEY_KPENTER => KEY_KPENTER,
            KeyboardUsageId::KEY_KP1 => KEY_KP1,
            KeyboardUsageId::KEY_KP2 => KEY_KP2,
            KeyboardUsageId::KEY_KP3 => KEY_KP3,
            KeyboardUsageId::KEY_KP4 => KEY_KP4,
            KeyboardUsageId::KEY_KP5 => KEY_KP5,
            KeyboardUsageId::KEY_KP6 => KEY_KP6,
            KeyboardUsageId::KEY_KP7 => KEY_KP7,
            KeyboardUsageId::KEY_KP8 => KEY_KP8,
            KeyboardUsageId::KEY_KP9 => KEY_KP9,
            KeyboardUsageId::KEY_KP0 => KEY_KP0,
            KeyboardUsageId::KEY_KPDOT => KEY_KPDOT,
            KeyboardUsageId::KEY_102ND => KEY_102ND,
            KeyboardUsageId::KEY_COMPOSE => KEY_COMPOSE,
            KeyboardUsageId::KEY_POWER => KEY_POWER,
            KeyboardUsageId::KEY_KPEQUAL => KEY_KPEQUAL,
            KeyboardUsageId::KEY_F13 => KEY_F13,
            KeyboardUsageId::KEY_F14 => KEY_F14,
            KeyboardUsageId::KEY_F15 => KEY_F15,
            KeyboardUsageId::KEY_F16 => KEY_F16,
            KeyboardUsageId::KEY_F17 => KEY_F17,
            KeyboardUsageId::KEY_F18 => KEY_F18,
            KeyboardUsageId::KEY_F19 => KEY_F19,
            KeyboardUsageId::KEY_F20 => KEY_F20,
            KeyboardUsageId::KEY_F21 => KEY_F21,
            KeyboardUsageId::KEY_F22 => KEY_F22,
            KeyboardUsageId::KEY_F23 => KEY_F23,
            KeyboardUsageId::KEY_F24 => KEY_F24,
            KeyboardUsageId::KEY_OPEN => KEY_OPEN,
            KeyboardUsageId::KEY_HELP => KEY_HELP,
            KeyboardUsageId::KEY_PROPS => KEY_PROPS,
            KeyboardUsageId::KEY_FRONT => KEY_FRONT,
            KeyboardUsageId::KEY_STOP => KEY_STOP,
            KeyboardUsageId::KEY_AGAIN => KEY_AGAIN,
            KeyboardUsageId::KEY_UNDO => KEY_UNDO,
            KeyboardUsageId::KEY_CUT => KEY_CUT,
            KeyboardUsageId::KEY_COPY => KEY_COPY,
            KeyboardUsageId::KEY_PASTE => KEY_PASTE,
            KeyboardUsageId::KEY_FIND => KEY_FIND,
            KeyboardUsageId::KEY_MUTE => KEY_MUTE,
            KeyboardUsageId::KEY_VOLUMEUP => KEY_VOLUMEUP,
            KeyboardUsageId::KEY_VOLUMEDOWN => KEY_VOLUMEDOWN,
            KeyboardUsageId::KEY_LOCKING_CAPSLOCK => KEY_LOCKING_CAPSLOCK,
            KeyboardUsageId::KEY_LOCKING_NUMLOCK => KEY_LOCKING_NUMLOCK,
            KeyboardUsageId::KEY_LOCKING_SCROLLOCK => KEY_LOCKING_SCROLLOCK,
            KeyboardUsageId::KEY_KPCOMMA => KEY_KPCOMMA,
            KeyboardUsageId::KEY_KPEQUALSIGN => KEY_KPEQUALSIGN,
            KeyboardUsageId::KEY_RO => KEY_RO,
            KeyboardUsageId::KEY_KATAKANAHIRAGANA => KEY_KATAKANAHIRAGANA,
            KeyboardUsageId::KEY_YEN => KEY_YEN,
            KeyboardUsageId::KEY_HENKAN => KEY_HENKAN,
            KeyboardUsageId::KEY_MUHENKAN => KEY_MUHENKAN,
            KeyboardUsageId::KEY_KPJPCOMMA => KEY_KPJPCOMMA,
            KeyboardUsageId::KEY_INTERNATIONAL7 => KEY_INTERNATIONAL7,
            KeyboardUsageId::KEY_INTERNATIONAL8 => KEY_INTERNATIONAL8,
            KeyboardUsageId::KEY_INTERNATIONAL9 => KEY_INTERNATIONAL9,
            KeyboardUsageId::KEY_HANGUEL => KEY_HANGUEL,
            KeyboardUsageId::KEY_HANJA => KEY_HANJA,
            KeyboardUsageId::KEY_KATAKANA => KEY_KATAKANA,
            KeyboardUsageId::KEY_HIRAGANA => KEY_HIRAGANA,
            KeyboardUsageId::KEY_ZENKAKUHANKAKU => KEY_ZENKAKUHANKAKU,
            KeyboardUsageId::KEY_LANG6 => KEY_LANG6,
            KeyboardUsageId::KEY_LANG7 => KEY_LANG7,
            KeyboardUsageId::KEY_LANG8 => KEY_LANG8,
            KeyboardUsageId::KEY_LANG9 => KEY_LANG9,
            KeyboardUsageId::KEY_ALTERNATE_ERASE => KEY_ALTERNATE_ERASE,
            KeyboardUsageId::KEY_ATTENTION => KEY_ATTENTION,
            KeyboardUsageId::KEY_CANCEL => KEY_CANCEL,
            KeyboardUsageId::KEY_PRIOR => KEY_PRIOR,
            KeyboardUsageId::KEY_RETURN => KEY_RETURN,
            KeyboardUsageId::KEY_SEPARATOR => KEY_SEPARATOR,
            KeyboardUsageId::KEY_OUT => KEY_OUT,
            KeyboardUsageId::KEY_CLEAR_AGAIN => KEY_CLEAR_AGAIN,
            KeyboardUsageId::KEY_SRSEL_PROPS => KEY_SRSEL_PROPS,
            KeyboardUsageId::KEY_EXSEL => KEY_EXSEL,
            KeyboardUsageId::KEY_KP00 => KEY_KP00,
            KeyboardUsageId::KEY_KP000 => KEY_KP000,
            KeyboardUsageId::KEY_THOUSANDS_SEPARATOR => KEY_THOUSANDS_SEPARATOR,
            KeyboardUsageId::KEY_DECIMAL_SEPARATOR => KEY_DECIMAL_SEPARATOR,
            KeyboardUsageId::KEY_CURRENCY_UNIT => KEY_CURRENCY_UNIT,
            KeyboardUsageId::KEY_CURRENCY_SUBUNIT => KEY_CURRENCY_SUBUNIT,
            KeyboardUsageId::KEY_KPLEFTPAREN => KEY_KPLEFTPAREN,
            KeyboardUsageId::KEY_KPRIGHTPAREN => KEY_KPRIGHTPAREN,
            KeyboardUsageId::KEY_KPLEFTBRACE => KEY_KPLEFTBRACE,
            KeyboardUsageId::KEY_KPRIGHTBRACE => KEY_KPRIGHTBRACE,
            KeyboardUsageId::KEY_KPTAB => KEY_KPTAB,
            KeyboardUsageId::KEY_KPBACKSPACE => KEY_KPBACKSPACE,
            KeyboardUsageId::KEY_KPA => KEY_KPA,
            KeyboardUsageId::KEY_KPB => KEY_KPB,
            KeyboardUsageId::KEY_KPC => KEY_KPC,
            KeyboardUsageId::KEY_KPD => KEY_KPD,
            KeyboardUsageId::KEY_KPE => KEY_KPE,
            KeyboardUsageId::KEY_KPF => KEY_KPF,
            KeyboardUsageId::KEY_KPXOR => KEY_KPXOR,
            KeyboardUsageId::KEY_KPCIRCUMEFLEX_ACCENT => KEY_KPCIRCUMEFLEX_ACCENT,
            KeyboardUsageId::KEY_KPPERCENT => KEY_KPPERCENT,
            KeyboardUsageId::KEY_KPLESS_THAN => KEY_KPLESS_THAN,
            KeyboardUsageId::KEY_KPGREATER_THAN => KEY_KPGREATER_THAN,
            KeyboardUsageId::KEY_KPAMP => KEY_KPAMP,
            KeyboardUsageId::KEY_KPDOUBLE_AMP => KEY_KPDOUBLE_AMP,
            KeyboardUsageId::KEY_KPOR => KEY_KPOR,
            KeyboardUsageId::KEY_KPDOUBLE_OR => KEY_KPDOUBLE_OR,
            KeyboardUsageId::KEY_KPCOLON => KEY_KPCOLON,
            KeyboardUsageId::KEY_KPSHARP => KEY_KPSHARP,
            KeyboardUsageId::KEY_KPSPACE => KEY_KPSPACE,
            KeyboardUsageId::KEY_KPATMARK => KEY_KPATMARK,
            KeyboardUsageId::KEY_KPEXCLAMATION => KEY_KPEXCLAMATION,
            KeyboardUsageId::KEY_KPMEMORY_STORE => KEY_KPMEMORY_STORE,
            KeyboardUsageId::KEY_KPMEMORY_RECALL => KEY_KPMEMORY_RECALL,
            KeyboardUsageId::KEY_KPMEMORY_CLEAR => KEY_KPMEMORY_CLEAR,
            KeyboardUsageId::KEY_KPMEMORY_ADD => KEY_KPMEMORY_ADD,
            KeyboardUsageId::KEY_KPMEMORY_SUBTRACT => KEY_KPMEMORY_SUBTRACT,
            KeyboardUsageId::KEY_KPMEMORY_MULTIPLE => KEY_KPMEMORY_MULTIPLE,
            KeyboardUsageId::KEY_KPMEMORY_DIVIDE => KEY_KPMEMORY_DIVIDE,
            KeyboardUsageId::KEY_KPMEMORY_PLUSMINUS => KEY_KPMEMORY_PLUSMINUS,
            KeyboardUsageId::KEY_KPCLEAR => KEY_KPCLEAR,
            KeyboardUsageId::KEY_KPCLEARENTRY => KEY_KPCLEARENTRY,
            KeyboardUsageId::KEY_KPBINARY => KEY_KPBINARY,
            KeyboardUsageId::KEY_KPOCTAL => KEY_KPOCTAL,
            KeyboardUsageId::KEY_KPDECIMAL => KEY_KPDECIMAL,
            KeyboardUsageId::KEY_KPHEX => KEY_KPHEX,
            KeyboardUsageId::KEY_LEFT_CTRL => KEY_LEFT_CTRL,
            KeyboardUsageId::KEY_LEFT_SHIFT => KEY_LEFT_SHIFT,
            KeyboardUsageId::KEY_LEFT_ALT => KEY_LEFT_ALT,
            KeyboardUsageId::KEY_LEFT_GUI => KEY_LEFT_GUI,
            KeyboardUsageId::KEY_RIGHT_CTRL => KEY_RIGHT_CTRL,
            KeyboardUsageId::KEY_RIGHT_SHIFT => KEY_RIGHT_SHIFT,
            KeyboardUsageId::KEY_RIGHT_ALT => KEY_RIGHT_ALT,
            KeyboardUsageId::KEY_RIGHT_GUI => KEY_RIGHT_GUI,
            KeyboardUsageId::KEY_PLAYPAUSE => KEY_PLAYPAUSE,
            KeyboardUsageId::KEY_STOPCD => KEY_STOPCD,
            KeyboardUsageId::KEY_PREVIOUSSONG => KEY_PREVIOUSSONG,
            KeyboardUsageId::KEY_NEXTSONG => KEY_NEXTSONG,
            KeyboardUsageId::KEY_EJECTCD => KEY_EJECTCD,
            KeyboardUsageId::KEY_WWW => KEY_WWW,
            KeyboardUsageId::KEY_BACK => KEY_BACK,
            KeyboardUsageId::KEY_FORWARD => KEY_FORWARD,
            KeyboardUsageId::KEY_SCROLLUP => KEY_SCROLLUP,
            KeyboardUsageId::KEY_SCROLLDOWN => KEY_SCROLLDOWN,
            KeyboardUsageId::KEY_EDIT => KEY_EDIT,
            KeyboardUsageId::KEY_SLEEP => KEY_SLEEP,
            KeyboardUsageId::KEY_COFFEE => KEY_COFFEE,
            KeyboardUsageId::KEY_REFRESH => KEY_REFRESH,
            KeyboardUsageId::KEY_CALC => KEY_CALC,
            KeyboardUsageId::Unknown(v) => v,
        }
    }
}
