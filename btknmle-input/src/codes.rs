#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::sys::linux_input_event_codes::*;

#[derive(Debug)]
pub enum KeyCodes {
    KEY_RESERVED,
    KEY_ESC,
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
    KEY_MINUS,
    KEY_EQUAL,
    KEY_BACKSPACE,
    KEY_TAB,
    KEY_Q,
    KEY_W,
    KEY_E,
    KEY_R,
    KEY_T,
    KEY_Y,
    KEY_U,
    KEY_I,
    KEY_O,
    KEY_P,
    KEY_LEFTBRACE,
    KEY_RIGHTBRACE,
    KEY_ENTER,
    KEY_LEFTCTRL,
    KEY_A,
    KEY_S,
    KEY_D,
    KEY_F,
    KEY_G,
    KEY_H,
    KEY_J,
    KEY_K,
    KEY_L,
    KEY_SEMICOLON,
    KEY_APOSTROPHE,
    KEY_GRAVE,
    KEY_LEFTSHIFT,
    KEY_BACKSLASH,
    KEY_Z,
    KEY_X,
    KEY_C,
    KEY_V,
    KEY_B,
    KEY_N,
    KEY_M,
    KEY_COMMA,
    KEY_DOT,
    KEY_SLASH,
    KEY_RIGHTSHIFT,
    KEY_KPASTERISK,
    KEY_LEFTALT,
    KEY_SPACE,
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
    KEY_NUMLOCK,
    KEY_SCROLLLOCK,
    KEY_KP7,
    KEY_KP8,
    KEY_KP9,
    KEY_KPMINUS,
    KEY_KP4,
    KEY_KP5,
    KEY_KP6,
    KEY_KPPLUS,
    KEY_KP1,
    KEY_KP2,
    KEY_KP3,
    KEY_KP0,
    KEY_KPDOT,
    KEY_ZENKAKUHANKAKU,
    KEY_102ND,
    KEY_F11,
    KEY_F12,
    KEY_RO,
    KEY_KATAKANA,
    KEY_HIRAGANA,
    KEY_HENKAN,
    KEY_KATAKANAHIRAGANA,
    KEY_MUHENKAN,
    KEY_KPJPCOMMA,
    KEY_KPENTER,
    KEY_RIGHTCTRL,
    KEY_KPSLASH,
    KEY_SYSRQ,
    KEY_RIGHTALT,
    KEY_LINEFEED,
    KEY_HOME,
    KEY_UP,
    KEY_PAGEUP,
    KEY_LEFT,
    KEY_RIGHT,
    KEY_END,
    KEY_DOWN,
    KEY_PAGEDOWN,
    KEY_INSERT,
    KEY_DELETE,
    KEY_MACRO,
    KEY_MUTE,
    KEY_VOLUMEDOWN,
    KEY_VOLUMEUP,
    KEY_POWER,
    KEY_KPEQUAL,
    KEY_KPPLUSMINUS,
    KEY_PAUSE,
    KEY_SCALE,
    KEY_KPCOMMA,
    KEY_HANGEUL,
    //KEY_HANGUEL,
    KEY_HANJA,
    KEY_YEN,
    KEY_LEFTMETA,
    KEY_RIGHTMETA,
    KEY_COMPOSE,
    KEY_STOP,
    KEY_AGAIN,
    KEY_PROPS,
    KEY_UNDO,
    KEY_FRONT,
    KEY_COPY,
    KEY_OPEN,
    KEY_PASTE,
    KEY_FIND,
    KEY_CUT,
    KEY_HELP,
    KEY_MENU,
    KEY_CALC,
    KEY_SETUP,
    KEY_SLEEP,
    KEY_WAKEUP,
    KEY_FILE,
    KEY_SENDFILE,
    KEY_DELETEFILE,
    KEY_XFER,
    KEY_PROG1,
    KEY_PROG2,
    KEY_WWW,
    KEY_MSDOS,
    KEY_COFFEE,
    //KEY_SCREENLOCK,
    KEY_ROTATE_DISPLAY,
    //KEY_DIRECTION,
    KEY_CYCLEWINDOWS,
    KEY_MAIL,
    KEY_BOOKMARKS,
    KEY_COMPUTER,
    KEY_BACK,
    KEY_FORWARD,
    KEY_CLOSECD,
    KEY_EJECTCD,
    KEY_EJECTCLOSECD,
    KEY_NEXTSONG,
    KEY_PLAYPAUSE,
    KEY_PREVIOUSSONG,
    KEY_STOPCD,
    KEY_RECORD,
    KEY_REWIND,
    KEY_PHONE,
    KEY_ISO,
    KEY_CONFIG,
    KEY_HOMEPAGE,
    KEY_REFRESH,
    KEY_EXIT,
    KEY_MOVE,
    KEY_EDIT,
    KEY_SCROLLUP,
    KEY_SCROLLDOWN,
    KEY_KPLEFTPAREN,
    KEY_KPRIGHTPAREN,
    KEY_NEW,
    KEY_REDO,
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
    KEY_PLAYCD,
    KEY_PAUSECD,
    KEY_PROG3,
    KEY_PROG4,
    KEY_DASHBOARD,
    KEY_SUSPEND,
    KEY_CLOSE,
    KEY_PLAY,
    KEY_FASTFORWARD,
    KEY_BASSBOOST,
    KEY_PRINT,
    KEY_HP,
    KEY_CAMERA,
    KEY_SOUND,
    KEY_QUESTION,
    KEY_EMAIL,
    KEY_CHAT,
    KEY_SEARCH,
    KEY_CONNECT,
    KEY_FINANCE,
    KEY_SPORT,
    KEY_SHOP,
    KEY_ALTERASE,
    KEY_CANCEL,
    KEY_BRIGHTNESSDOWN,
    KEY_BRIGHTNESSUP,
    KEY_MEDIA,
    KEY_SWITCHVIDEOMODE,
    KEY_KBDILLUMTOGGLE,
    KEY_KBDILLUMDOWN,
    KEY_KBDILLUMUP,
    KEY_SEND,
    KEY_REPLY,
    KEY_FORWARDMAIL,
    KEY_SAVE,
    KEY_DOCUMENTS,
    KEY_BATTERY,
    KEY_BLUETOOTH,
    KEY_WLAN,
    KEY_UWB,
    KEY_UNKNOWN,
    KEY_VIDEO_NEXT,
    KEY_VIDEO_PREV,
    KEY_BRIGHTNESS_CYCLE,
    KEY_BRIGHTNESS_AUTO,
    //KEY_BRIGHTNESS_ZERO,
    KEY_DISPLAY_OFF,
    KEY_WWAN,
    //KEY_WIMAX,
    KEY_RFKILL,
    KEY_MICMUTE,
    KEY_OK,
    KEY_SELECT,
    KEY_GOTO,
    KEY_CLEAR,
    KEY_POWER2,
    KEY_OPTION,
    KEY_INFO,
    KEY_TIME,
    KEY_VENDOR,
    KEY_ARCHIVE,
    KEY_PROGRAM,
    KEY_CHANNEL,
    KEY_FAVORITES,
    KEY_EPG,
    KEY_PVR,
    KEY_MHP,
    KEY_LANGUAGE,
    KEY_TITLE,
    KEY_SUBTITLE,
    KEY_ANGLE,
    KEY_FULL_SCREEN,
    //KEY_ZOOM,
    KEY_MODE,
    KEY_KEYBOARD,
    KEY_ASPECT_RATIO,
    //KEY_SCREEN,
    KEY_PC,
    KEY_TV,
    KEY_TV2,
    KEY_VCR,
    KEY_VCR2,
    KEY_SAT,
    KEY_SAT2,
    KEY_CD,
    KEY_TAPE,
    KEY_RADIO,
    KEY_TUNER,
    KEY_PLAYER,
    KEY_TEXT,
    KEY_DVD,
    KEY_AUX,
    KEY_MP3,
    KEY_AUDIO,
    KEY_VIDEO,
    KEY_DIRECTORY,
    KEY_LIST,
    KEY_MEMO,
    KEY_CALENDAR,
    KEY_RED,
    KEY_GREEN,
    KEY_YELLOW,
    KEY_BLUE,
    KEY_CHANNELUP,
    KEY_CHANNELDOWN,
    KEY_FIRST,
    KEY_LAST,
    KEY_AB,
    KEY_NEXT,
    KEY_RESTART,
    KEY_SLOW,
    KEY_SHUFFLE,
    KEY_BREAK,
    KEY_PREVIOUS,
    KEY_DIGITS,
    KEY_TEEN,
    KEY_TWEN,
    KEY_VIDEOPHONE,
    KEY_GAMES,
    KEY_ZOOMIN,
    KEY_ZOOMOUT,
    KEY_ZOOMRESET,
    KEY_WORDPROCESSOR,
    KEY_EDITOR,
    KEY_SPREADSHEET,
    KEY_GRAPHICSEDITOR,
    KEY_PRESENTATION,
    KEY_DATABASE,
    KEY_NEWS,
    KEY_VOICEMAIL,
    KEY_ADDRESSBOOK,
    KEY_MESSENGER,
    KEY_DISPLAYTOGGLE,
    //KEY_BRIGHTNESS_TOGGLE,
    KEY_SPELLCHECK,
    KEY_LOGOFF,
    KEY_DOLLAR,
    KEY_EURO,
    KEY_FRAMEBACK,
    KEY_FRAMEFORWARD,
    KEY_CONTEXT_MENU,
    KEY_MEDIA_REPEAT,
    KEY_10CHANNELSUP,
    KEY_10CHANNELSDOWN,
    KEY_IMAGES,
    KEY_DEL_EOL,
    KEY_DEL_EOS,
    KEY_INS_LINE,
    KEY_DEL_LINE,
    KEY_FN,
    KEY_FN_ESC,
    KEY_FN_F1,
    KEY_FN_F2,
    KEY_FN_F3,
    KEY_FN_F4,
    KEY_FN_F5,
    KEY_FN_F6,
    KEY_FN_F7,
    KEY_FN_F8,
    KEY_FN_F9,
    KEY_FN_F10,
    KEY_FN_F11,
    KEY_FN_F12,
    KEY_FN_1,
    KEY_FN_2,
    KEY_FN_D,
    KEY_FN_E,
    KEY_FN_F,
    KEY_FN_S,
    KEY_FN_B,
    KEY_BRL_DOT1,
    KEY_BRL_DOT2,
    KEY_BRL_DOT3,
    KEY_BRL_DOT4,
    KEY_BRL_DOT5,
    KEY_BRL_DOT6,
    KEY_BRL_DOT7,
    KEY_BRL_DOT8,
    KEY_BRL_DOT9,
    KEY_BRL_DOT10,
    KEY_NUMERIC_0,
    KEY_NUMERIC_1,
    KEY_NUMERIC_2,
    KEY_NUMERIC_3,
    KEY_NUMERIC_4,
    KEY_NUMERIC_5,
    KEY_NUMERIC_6,
    KEY_NUMERIC_7,
    KEY_NUMERIC_8,
    KEY_NUMERIC_9,
    KEY_NUMERIC_STAR,
    KEY_NUMERIC_POUND,
    KEY_NUMERIC_A,
    KEY_NUMERIC_B,
    KEY_NUMERIC_C,
    KEY_NUMERIC_D,
    KEY_CAMERA_FOCUS,
    KEY_WPS_BUTTON,
    KEY_TOUCHPAD_TOGGLE,
    KEY_TOUCHPAD_ON,
    KEY_TOUCHPAD_OFF,
    KEY_CAMERA_ZOOMIN,
    KEY_CAMERA_ZOOMOUT,
    KEY_CAMERA_UP,
    KEY_CAMERA_DOWN,
    KEY_CAMERA_LEFT,
    KEY_CAMERA_RIGHT,
    KEY_ATTENDANT_ON,
    KEY_ATTENDANT_OFF,
    KEY_ATTENDANT_TOGGLE,
    KEY_LIGHTS_TOGGLE,
    KEY_ALS_TOGGLE,
    KEY_ROTATE_LOCK_TOGGLE,
    KEY_BUTTONCONFIG,
    KEY_TASKMANAGER,
    KEY_JOURNAL,
    KEY_CONTROLPANEL,
    KEY_APPSELECT,
    KEY_SCREENSAVER,
    KEY_VOICECOMMAND,
    KEY_ASSISTANT,
    KEY_KBD_LAYOUT_NEXT,
    KEY_BRIGHTNESS_MIN,
    KEY_BRIGHTNESS_MAX,
    KEY_KBDINPUTASSIST_PREV,
    KEY_KBDINPUTASSIST_NEXT,
    KEY_KBDINPUTASSIST_PREVGROUP,
    KEY_KBDINPUTASSIST_NEXTGROUP,
    KEY_KBDINPUTASSIST_ACCEPT,
    KEY_KBDINPUTASSIST_CANCEL,
    KEY_RIGHT_UP,
    KEY_RIGHT_DOWN,
    KEY_LEFT_UP,
    KEY_LEFT_DOWN,
    KEY_ROOT_MENU,
    KEY_MEDIA_TOP_MENU,
    KEY_NUMERIC_11,
    KEY_NUMERIC_12,
    KEY_AUDIO_DESC,
    KEY_3D_MODE,
    KEY_NEXT_FAVORITE,
    KEY_STOP_RECORD,
    KEY_PAUSE_RECORD,
    KEY_VOD,
    KEY_UNMUTE,
    KEY_FASTREVERSE,
    KEY_SLOWREVERSE,
    KEY_DATA,
    KEY_ONSCREEN_KEYBOARD,
    Unknown(u32),
}

impl From<u32> for KeyCodes {
    fn from(v: u32) -> Self {
        match v {
            KEY_RESERVED => Self::KEY_RESERVED,
            KEY_ESC => Self::KEY_ESC,
            KEY_1 => Self::KEY_1,
            KEY_2 => Self::KEY_2,
            KEY_3 => Self::KEY_3,
            KEY_4 => Self::KEY_4,
            KEY_5 => Self::KEY_5,
            KEY_6 => Self::KEY_6,
            KEY_7 => Self::KEY_7,
            KEY_8 => Self::KEY_8,
            KEY_9 => Self::KEY_9,
            KEY_0 => Self::KEY_0,
            KEY_MINUS => Self::KEY_MINUS,
            KEY_EQUAL => Self::KEY_EQUAL,
            KEY_BACKSPACE => Self::KEY_BACKSPACE,
            KEY_TAB => Self::KEY_TAB,
            KEY_Q => Self::KEY_Q,
            KEY_W => Self::KEY_W,
            KEY_E => Self::KEY_E,
            KEY_R => Self::KEY_R,
            KEY_T => Self::KEY_T,
            KEY_Y => Self::KEY_Y,
            KEY_U => Self::KEY_U,
            KEY_I => Self::KEY_I,
            KEY_O => Self::KEY_O,
            KEY_P => Self::KEY_P,
            KEY_LEFTBRACE => Self::KEY_LEFTBRACE,
            KEY_RIGHTBRACE => Self::KEY_RIGHTBRACE,
            KEY_ENTER => Self::KEY_ENTER,
            KEY_LEFTCTRL => Self::KEY_LEFTCTRL,
            KEY_A => Self::KEY_A,
            KEY_S => Self::KEY_S,
            KEY_D => Self::KEY_D,
            KEY_F => Self::KEY_F,
            KEY_G => Self::KEY_G,
            KEY_H => Self::KEY_H,
            KEY_J => Self::KEY_J,
            KEY_K => Self::KEY_K,
            KEY_L => Self::KEY_L,
            KEY_SEMICOLON => Self::KEY_SEMICOLON,
            KEY_APOSTROPHE => Self::KEY_APOSTROPHE,
            KEY_GRAVE => Self::KEY_GRAVE,
            KEY_LEFTSHIFT => Self::KEY_LEFTSHIFT,
            KEY_BACKSLASH => Self::KEY_BACKSLASH,
            KEY_Z => Self::KEY_Z,
            KEY_X => Self::KEY_X,
            KEY_C => Self::KEY_C,
            KEY_V => Self::KEY_V,
            KEY_B => Self::KEY_B,
            KEY_N => Self::KEY_N,
            KEY_M => Self::KEY_M,
            KEY_COMMA => Self::KEY_COMMA,
            KEY_DOT => Self::KEY_DOT,
            KEY_SLASH => Self::KEY_SLASH,
            KEY_RIGHTSHIFT => Self::KEY_RIGHTSHIFT,
            KEY_KPASTERISK => Self::KEY_KPASTERISK,
            KEY_LEFTALT => Self::KEY_LEFTALT,
            KEY_SPACE => Self::KEY_SPACE,
            KEY_CAPSLOCK => Self::KEY_CAPSLOCK,
            KEY_F1 => Self::KEY_F1,
            KEY_F2 => Self::KEY_F2,
            KEY_F3 => Self::KEY_F3,
            KEY_F4 => Self::KEY_F4,
            KEY_F5 => Self::KEY_F5,
            KEY_F6 => Self::KEY_F6,
            KEY_F7 => Self::KEY_F7,
            KEY_F8 => Self::KEY_F8,
            KEY_F9 => Self::KEY_F9,
            KEY_F10 => Self::KEY_F10,
            KEY_NUMLOCK => Self::KEY_NUMLOCK,
            KEY_SCROLLLOCK => Self::KEY_SCROLLLOCK,
            KEY_KP7 => Self::KEY_KP7,
            KEY_KP8 => Self::KEY_KP8,
            KEY_KP9 => Self::KEY_KP9,
            KEY_KPMINUS => Self::KEY_KPMINUS,
            KEY_KP4 => Self::KEY_KP4,
            KEY_KP5 => Self::KEY_KP5,
            KEY_KP6 => Self::KEY_KP6,
            KEY_KPPLUS => Self::KEY_KPPLUS,
            KEY_KP1 => Self::KEY_KP1,
            KEY_KP2 => Self::KEY_KP2,
            KEY_KP3 => Self::KEY_KP3,
            KEY_KP0 => Self::KEY_KP0,
            KEY_KPDOT => Self::KEY_KPDOT,
            KEY_ZENKAKUHANKAKU => Self::KEY_ZENKAKUHANKAKU,
            KEY_102ND => Self::KEY_102ND,
            KEY_F11 => Self::KEY_F11,
            KEY_F12 => Self::KEY_F12,
            KEY_RO => Self::KEY_RO,
            KEY_KATAKANA => Self::KEY_KATAKANA,
            KEY_HIRAGANA => Self::KEY_HIRAGANA,
            KEY_HENKAN => Self::KEY_HENKAN,
            KEY_KATAKANAHIRAGANA => Self::KEY_KATAKANAHIRAGANA,
            KEY_MUHENKAN => Self::KEY_MUHENKAN,
            KEY_KPJPCOMMA => Self::KEY_KPJPCOMMA,
            KEY_KPENTER => Self::KEY_KPENTER,
            KEY_RIGHTCTRL => Self::KEY_RIGHTCTRL,
            KEY_KPSLASH => Self::KEY_KPSLASH,
            KEY_SYSRQ => Self::KEY_SYSRQ,
            KEY_RIGHTALT => Self::KEY_RIGHTALT,
            KEY_LINEFEED => Self::KEY_LINEFEED,
            KEY_HOME => Self::KEY_HOME,
            KEY_UP => Self::KEY_UP,
            KEY_PAGEUP => Self::KEY_PAGEUP,
            KEY_LEFT => Self::KEY_LEFT,
            KEY_RIGHT => Self::KEY_RIGHT,
            KEY_END => Self::KEY_END,
            KEY_DOWN => Self::KEY_DOWN,
            KEY_PAGEDOWN => Self::KEY_PAGEDOWN,
            KEY_INSERT => Self::KEY_INSERT,
            KEY_DELETE => Self::KEY_DELETE,
            KEY_MACRO => Self::KEY_MACRO,
            KEY_MUTE => Self::KEY_MUTE,
            KEY_VOLUMEDOWN => Self::KEY_VOLUMEDOWN,
            KEY_VOLUMEUP => Self::KEY_VOLUMEUP,
            KEY_POWER => Self::KEY_POWER,
            KEY_KPEQUAL => Self::KEY_KPEQUAL,
            KEY_KPPLUSMINUS => Self::KEY_KPPLUSMINUS,
            KEY_PAUSE => Self::KEY_PAUSE,
            KEY_SCALE => Self::KEY_SCALE,
            KEY_KPCOMMA => Self::KEY_KPCOMMA,
            KEY_HANGEUL => Self::KEY_HANGEUL,
            //KEY_HANGUEL => Self::KEY_HANGUEL,
            KEY_HANJA => Self::KEY_HANJA,
            KEY_YEN => Self::KEY_YEN,
            KEY_LEFTMETA => Self::KEY_LEFTMETA,
            KEY_RIGHTMETA => Self::KEY_RIGHTMETA,
            KEY_COMPOSE => Self::KEY_COMPOSE,
            KEY_STOP => Self::KEY_STOP,
            KEY_AGAIN => Self::KEY_AGAIN,
            KEY_PROPS => Self::KEY_PROPS,
            KEY_UNDO => Self::KEY_UNDO,
            KEY_FRONT => Self::KEY_FRONT,
            KEY_COPY => Self::KEY_COPY,
            KEY_OPEN => Self::KEY_OPEN,
            KEY_PASTE => Self::KEY_PASTE,
            KEY_FIND => Self::KEY_FIND,
            KEY_CUT => Self::KEY_CUT,
            KEY_HELP => Self::KEY_HELP,
            KEY_MENU => Self::KEY_MENU,
            KEY_CALC => Self::KEY_CALC,
            KEY_SETUP => Self::KEY_SETUP,
            KEY_SLEEP => Self::KEY_SLEEP,
            KEY_WAKEUP => Self::KEY_WAKEUP,
            KEY_FILE => Self::KEY_FILE,
            KEY_SENDFILE => Self::KEY_SENDFILE,
            KEY_DELETEFILE => Self::KEY_DELETEFILE,
            KEY_XFER => Self::KEY_XFER,
            KEY_PROG1 => Self::KEY_PROG1,
            KEY_PROG2 => Self::KEY_PROG2,
            KEY_WWW => Self::KEY_WWW,
            KEY_MSDOS => Self::KEY_MSDOS,
            KEY_COFFEE => Self::KEY_COFFEE,
            //KEY_SCREENLOCK => Self::KEY_SCREENLOCK,
            KEY_ROTATE_DISPLAY => Self::KEY_ROTATE_DISPLAY,
            //KEY_DIRECTION => Self::KEY_DIRECTION,
            KEY_CYCLEWINDOWS => Self::KEY_CYCLEWINDOWS,
            KEY_MAIL => Self::KEY_MAIL,
            KEY_BOOKMARKS => Self::KEY_BOOKMARKS,
            KEY_COMPUTER => Self::KEY_COMPUTER,
            KEY_BACK => Self::KEY_BACK,
            KEY_FORWARD => Self::KEY_FORWARD,
            KEY_CLOSECD => Self::KEY_CLOSECD,
            KEY_EJECTCD => Self::KEY_EJECTCD,
            KEY_EJECTCLOSECD => Self::KEY_EJECTCLOSECD,
            KEY_NEXTSONG => Self::KEY_NEXTSONG,
            KEY_PLAYPAUSE => Self::KEY_PLAYPAUSE,
            KEY_PREVIOUSSONG => Self::KEY_PREVIOUSSONG,
            KEY_STOPCD => Self::KEY_STOPCD,
            KEY_RECORD => Self::KEY_RECORD,
            KEY_REWIND => Self::KEY_REWIND,
            KEY_PHONE => Self::KEY_PHONE,
            KEY_ISO => Self::KEY_ISO,
            KEY_CONFIG => Self::KEY_CONFIG,
            KEY_HOMEPAGE => Self::KEY_HOMEPAGE,
            KEY_REFRESH => Self::KEY_REFRESH,
            KEY_EXIT => Self::KEY_EXIT,
            KEY_MOVE => Self::KEY_MOVE,
            KEY_EDIT => Self::KEY_EDIT,
            KEY_SCROLLUP => Self::KEY_SCROLLUP,
            KEY_SCROLLDOWN => Self::KEY_SCROLLDOWN,
            KEY_KPLEFTPAREN => Self::KEY_KPLEFTPAREN,
            KEY_KPRIGHTPAREN => Self::KEY_KPRIGHTPAREN,
            KEY_NEW => Self::KEY_NEW,
            KEY_REDO => Self::KEY_REDO,
            KEY_F13 => Self::KEY_F13,
            KEY_F14 => Self::KEY_F14,
            KEY_F15 => Self::KEY_F15,
            KEY_F16 => Self::KEY_F16,
            KEY_F17 => Self::KEY_F17,
            KEY_F18 => Self::KEY_F18,
            KEY_F19 => Self::KEY_F19,
            KEY_F20 => Self::KEY_F20,
            KEY_F21 => Self::KEY_F21,
            KEY_F22 => Self::KEY_F22,
            KEY_F23 => Self::KEY_F23,
            KEY_F24 => Self::KEY_F24,
            KEY_PLAYCD => Self::KEY_PLAYCD,
            KEY_PAUSECD => Self::KEY_PAUSECD,
            KEY_PROG3 => Self::KEY_PROG3,
            KEY_PROG4 => Self::KEY_PROG4,
            KEY_DASHBOARD => Self::KEY_DASHBOARD,
            KEY_SUSPEND => Self::KEY_SUSPEND,
            KEY_CLOSE => Self::KEY_CLOSE,
            KEY_PLAY => Self::KEY_PLAY,
            KEY_FASTFORWARD => Self::KEY_FASTFORWARD,
            KEY_BASSBOOST => Self::KEY_BASSBOOST,
            KEY_PRINT => Self::KEY_PRINT,
            KEY_HP => Self::KEY_HP,
            KEY_CAMERA => Self::KEY_CAMERA,
            KEY_SOUND => Self::KEY_SOUND,
            KEY_QUESTION => Self::KEY_QUESTION,
            KEY_EMAIL => Self::KEY_EMAIL,
            KEY_CHAT => Self::KEY_CHAT,
            KEY_SEARCH => Self::KEY_SEARCH,
            KEY_CONNECT => Self::KEY_CONNECT,
            KEY_FINANCE => Self::KEY_FINANCE,
            KEY_SPORT => Self::KEY_SPORT,
            KEY_SHOP => Self::KEY_SHOP,
            KEY_ALTERASE => Self::KEY_ALTERASE,
            KEY_CANCEL => Self::KEY_CANCEL,
            KEY_BRIGHTNESSDOWN => Self::KEY_BRIGHTNESSDOWN,
            KEY_BRIGHTNESSUP => Self::KEY_BRIGHTNESSUP,
            KEY_MEDIA => Self::KEY_MEDIA,
            KEY_SWITCHVIDEOMODE => Self::KEY_SWITCHVIDEOMODE,
            KEY_KBDILLUMTOGGLE => Self::KEY_KBDILLUMTOGGLE,
            KEY_KBDILLUMDOWN => Self::KEY_KBDILLUMDOWN,
            KEY_KBDILLUMUP => Self::KEY_KBDILLUMUP,
            KEY_SEND => Self::KEY_SEND,
            KEY_REPLY => Self::KEY_REPLY,
            KEY_FORWARDMAIL => Self::KEY_FORWARDMAIL,
            KEY_SAVE => Self::KEY_SAVE,
            KEY_DOCUMENTS => Self::KEY_DOCUMENTS,
            KEY_BATTERY => Self::KEY_BATTERY,
            KEY_BLUETOOTH => Self::KEY_BLUETOOTH,
            KEY_WLAN => Self::KEY_WLAN,
            KEY_UWB => Self::KEY_UWB,
            KEY_UNKNOWN => Self::KEY_UNKNOWN,
            KEY_VIDEO_NEXT => Self::KEY_VIDEO_NEXT,
            KEY_VIDEO_PREV => Self::KEY_VIDEO_PREV,
            KEY_BRIGHTNESS_CYCLE => Self::KEY_BRIGHTNESS_CYCLE,
            KEY_BRIGHTNESS_AUTO => Self::KEY_BRIGHTNESS_AUTO,
            //KEY_BRIGHTNESS_ZERO => Self::KEY_BRIGHTNESS_ZERO,
            KEY_DISPLAY_OFF => Self::KEY_DISPLAY_OFF,
            KEY_WWAN => Self::KEY_WWAN,
            //KEY_WIMAX => Self::KEY_WIMAX,
            KEY_RFKILL => Self::KEY_RFKILL,
            KEY_MICMUTE => Self::KEY_MICMUTE,
            KEY_OK => Self::KEY_OK,
            KEY_SELECT => Self::KEY_SELECT,
            KEY_GOTO => Self::KEY_GOTO,
            KEY_CLEAR => Self::KEY_CLEAR,
            KEY_POWER2 => Self::KEY_POWER2,
            KEY_OPTION => Self::KEY_OPTION,
            KEY_INFO => Self::KEY_INFO,
            KEY_TIME => Self::KEY_TIME,
            KEY_VENDOR => Self::KEY_VENDOR,
            KEY_ARCHIVE => Self::KEY_ARCHIVE,
            KEY_PROGRAM => Self::KEY_PROGRAM,
            KEY_CHANNEL => Self::KEY_CHANNEL,
            KEY_FAVORITES => Self::KEY_FAVORITES,
            KEY_EPG => Self::KEY_EPG,
            KEY_PVR => Self::KEY_PVR,
            KEY_MHP => Self::KEY_MHP,
            KEY_LANGUAGE => Self::KEY_LANGUAGE,
            KEY_TITLE => Self::KEY_TITLE,
            KEY_SUBTITLE => Self::KEY_SUBTITLE,
            KEY_ANGLE => Self::KEY_ANGLE,
            KEY_FULL_SCREEN => Self::KEY_FULL_SCREEN,
            //KEY_ZOOM => Self::KEY_ZOOM,
            KEY_MODE => Self::KEY_MODE,
            KEY_KEYBOARD => Self::KEY_KEYBOARD,
            KEY_ASPECT_RATIO => Self::KEY_ASPECT_RATIO,
            //KEY_SCREEN => Self::KEY_SCREEN,
            KEY_PC => Self::KEY_PC,
            KEY_TV => Self::KEY_TV,
            KEY_TV2 => Self::KEY_TV2,
            KEY_VCR => Self::KEY_VCR,
            KEY_VCR2 => Self::KEY_VCR2,
            KEY_SAT => Self::KEY_SAT,
            KEY_SAT2 => Self::KEY_SAT2,
            KEY_CD => Self::KEY_CD,
            KEY_TAPE => Self::KEY_TAPE,
            KEY_RADIO => Self::KEY_RADIO,
            KEY_TUNER => Self::KEY_TUNER,
            KEY_PLAYER => Self::KEY_PLAYER,
            KEY_TEXT => Self::KEY_TEXT,
            KEY_DVD => Self::KEY_DVD,
            KEY_AUX => Self::KEY_AUX,
            KEY_MP3 => Self::KEY_MP3,
            KEY_AUDIO => Self::KEY_AUDIO,
            KEY_VIDEO => Self::KEY_VIDEO,
            KEY_DIRECTORY => Self::KEY_DIRECTORY,
            KEY_LIST => Self::KEY_LIST,
            KEY_MEMO => Self::KEY_MEMO,
            KEY_CALENDAR => Self::KEY_CALENDAR,
            KEY_RED => Self::KEY_RED,
            KEY_GREEN => Self::KEY_GREEN,
            KEY_YELLOW => Self::KEY_YELLOW,
            KEY_BLUE => Self::KEY_BLUE,
            KEY_CHANNELUP => Self::KEY_CHANNELUP,
            KEY_CHANNELDOWN => Self::KEY_CHANNELDOWN,
            KEY_FIRST => Self::KEY_FIRST,
            KEY_LAST => Self::KEY_LAST,
            KEY_AB => Self::KEY_AB,
            KEY_NEXT => Self::KEY_NEXT,
            KEY_RESTART => Self::KEY_RESTART,
            KEY_SLOW => Self::KEY_SLOW,
            KEY_SHUFFLE => Self::KEY_SHUFFLE,
            KEY_BREAK => Self::KEY_BREAK,
            KEY_PREVIOUS => Self::KEY_PREVIOUS,
            KEY_DIGITS => Self::KEY_DIGITS,
            KEY_TEEN => Self::KEY_TEEN,
            KEY_TWEN => Self::KEY_TWEN,
            KEY_VIDEOPHONE => Self::KEY_VIDEOPHONE,
            KEY_GAMES => Self::KEY_GAMES,
            KEY_ZOOMIN => Self::KEY_ZOOMIN,
            KEY_ZOOMOUT => Self::KEY_ZOOMOUT,
            KEY_ZOOMRESET => Self::KEY_ZOOMRESET,
            KEY_WORDPROCESSOR => Self::KEY_WORDPROCESSOR,
            KEY_EDITOR => Self::KEY_EDITOR,
            KEY_SPREADSHEET => Self::KEY_SPREADSHEET,
            KEY_GRAPHICSEDITOR => Self::KEY_GRAPHICSEDITOR,
            KEY_PRESENTATION => Self::KEY_PRESENTATION,
            KEY_DATABASE => Self::KEY_DATABASE,
            KEY_NEWS => Self::KEY_NEWS,
            KEY_VOICEMAIL => Self::KEY_VOICEMAIL,
            KEY_ADDRESSBOOK => Self::KEY_ADDRESSBOOK,
            KEY_MESSENGER => Self::KEY_MESSENGER,
            KEY_DISPLAYTOGGLE => Self::KEY_DISPLAYTOGGLE,
            //KEY_BRIGHTNESS_TOGGLE => Self::KEY_BRIGHTNESS_TOGGLE,
            KEY_SPELLCHECK => Self::KEY_SPELLCHECK,
            KEY_LOGOFF => Self::KEY_LOGOFF,
            KEY_DOLLAR => Self::KEY_DOLLAR,
            KEY_EURO => Self::KEY_EURO,
            KEY_FRAMEBACK => Self::KEY_FRAMEBACK,
            KEY_FRAMEFORWARD => Self::KEY_FRAMEFORWARD,
            KEY_CONTEXT_MENU => Self::KEY_CONTEXT_MENU,
            KEY_MEDIA_REPEAT => Self::KEY_MEDIA_REPEAT,
            KEY_10CHANNELSUP => Self::KEY_10CHANNELSUP,
            KEY_10CHANNELSDOWN => Self::KEY_10CHANNELSDOWN,
            KEY_IMAGES => Self::KEY_IMAGES,
            KEY_DEL_EOL => Self::KEY_DEL_EOL,
            KEY_DEL_EOS => Self::KEY_DEL_EOS,
            KEY_INS_LINE => Self::KEY_INS_LINE,
            KEY_DEL_LINE => Self::KEY_DEL_LINE,
            KEY_FN => Self::KEY_FN,
            KEY_FN_ESC => Self::KEY_FN_ESC,
            KEY_FN_F1 => Self::KEY_FN_F1,
            KEY_FN_F2 => Self::KEY_FN_F2,
            KEY_FN_F3 => Self::KEY_FN_F3,
            KEY_FN_F4 => Self::KEY_FN_F4,
            KEY_FN_F5 => Self::KEY_FN_F5,
            KEY_FN_F6 => Self::KEY_FN_F6,
            KEY_FN_F7 => Self::KEY_FN_F7,
            KEY_FN_F8 => Self::KEY_FN_F8,
            KEY_FN_F9 => Self::KEY_FN_F9,
            KEY_FN_F10 => Self::KEY_FN_F10,
            KEY_FN_F11 => Self::KEY_FN_F11,
            KEY_FN_F12 => Self::KEY_FN_F12,
            KEY_FN_1 => Self::KEY_FN_1,
            KEY_FN_2 => Self::KEY_FN_2,
            KEY_FN_D => Self::KEY_FN_D,
            KEY_FN_E => Self::KEY_FN_E,
            KEY_FN_F => Self::KEY_FN_F,
            KEY_FN_S => Self::KEY_FN_S,
            KEY_FN_B => Self::KEY_FN_B,
            KEY_BRL_DOT1 => Self::KEY_BRL_DOT1,
            KEY_BRL_DOT2 => Self::KEY_BRL_DOT2,
            KEY_BRL_DOT3 => Self::KEY_BRL_DOT3,
            KEY_BRL_DOT4 => Self::KEY_BRL_DOT4,
            KEY_BRL_DOT5 => Self::KEY_BRL_DOT5,
            KEY_BRL_DOT6 => Self::KEY_BRL_DOT6,
            KEY_BRL_DOT7 => Self::KEY_BRL_DOT7,
            KEY_BRL_DOT8 => Self::KEY_BRL_DOT8,
            KEY_BRL_DOT9 => Self::KEY_BRL_DOT9,
            KEY_BRL_DOT10 => Self::KEY_BRL_DOT10,
            KEY_NUMERIC_0 => Self::KEY_NUMERIC_0,
            KEY_NUMERIC_1 => Self::KEY_NUMERIC_1,
            KEY_NUMERIC_2 => Self::KEY_NUMERIC_2,
            KEY_NUMERIC_3 => Self::KEY_NUMERIC_3,
            KEY_NUMERIC_4 => Self::KEY_NUMERIC_4,
            KEY_NUMERIC_5 => Self::KEY_NUMERIC_5,
            KEY_NUMERIC_6 => Self::KEY_NUMERIC_6,
            KEY_NUMERIC_7 => Self::KEY_NUMERIC_7,
            KEY_NUMERIC_8 => Self::KEY_NUMERIC_8,
            KEY_NUMERIC_9 => Self::KEY_NUMERIC_9,
            KEY_NUMERIC_STAR => Self::KEY_NUMERIC_STAR,
            KEY_NUMERIC_POUND => Self::KEY_NUMERIC_POUND,
            KEY_NUMERIC_A => Self::KEY_NUMERIC_A,
            KEY_NUMERIC_B => Self::KEY_NUMERIC_B,
            KEY_NUMERIC_C => Self::KEY_NUMERIC_C,
            KEY_NUMERIC_D => Self::KEY_NUMERIC_D,
            KEY_CAMERA_FOCUS => Self::KEY_CAMERA_FOCUS,
            KEY_WPS_BUTTON => Self::KEY_WPS_BUTTON,
            KEY_TOUCHPAD_TOGGLE => Self::KEY_TOUCHPAD_TOGGLE,
            KEY_TOUCHPAD_ON => Self::KEY_TOUCHPAD_ON,
            KEY_TOUCHPAD_OFF => Self::KEY_TOUCHPAD_OFF,
            KEY_CAMERA_ZOOMIN => Self::KEY_CAMERA_ZOOMIN,
            KEY_CAMERA_ZOOMOUT => Self::KEY_CAMERA_ZOOMOUT,
            KEY_CAMERA_UP => Self::KEY_CAMERA_UP,
            KEY_CAMERA_DOWN => Self::KEY_CAMERA_DOWN,
            KEY_CAMERA_LEFT => Self::KEY_CAMERA_LEFT,
            KEY_CAMERA_RIGHT => Self::KEY_CAMERA_RIGHT,
            KEY_ATTENDANT_ON => Self::KEY_ATTENDANT_ON,
            KEY_ATTENDANT_OFF => Self::KEY_ATTENDANT_OFF,
            KEY_ATTENDANT_TOGGLE => Self::KEY_ATTENDANT_TOGGLE,
            KEY_LIGHTS_TOGGLE => Self::KEY_LIGHTS_TOGGLE,
            KEY_ALS_TOGGLE => Self::KEY_ALS_TOGGLE,
            KEY_ROTATE_LOCK_TOGGLE => Self::KEY_ROTATE_LOCK_TOGGLE,
            KEY_BUTTONCONFIG => Self::KEY_BUTTONCONFIG,
            KEY_TASKMANAGER => Self::KEY_TASKMANAGER,
            KEY_JOURNAL => Self::KEY_JOURNAL,
            KEY_CONTROLPANEL => Self::KEY_CONTROLPANEL,
            KEY_APPSELECT => Self::KEY_APPSELECT,
            KEY_SCREENSAVER => Self::KEY_SCREENSAVER,
            KEY_VOICECOMMAND => Self::KEY_VOICECOMMAND,
            KEY_ASSISTANT => Self::KEY_ASSISTANT,
            KEY_KBD_LAYOUT_NEXT => Self::KEY_KBD_LAYOUT_NEXT,
            KEY_BRIGHTNESS_MIN => Self::KEY_BRIGHTNESS_MIN,
            KEY_BRIGHTNESS_MAX => Self::KEY_BRIGHTNESS_MAX,
            KEY_KBDINPUTASSIST_PREV => Self::KEY_KBDINPUTASSIST_PREV,
            KEY_KBDINPUTASSIST_NEXT => Self::KEY_KBDINPUTASSIST_NEXT,
            KEY_KBDINPUTASSIST_PREVGROUP => Self::KEY_KBDINPUTASSIST_PREVGROUP,
            KEY_KBDINPUTASSIST_NEXTGROUP => Self::KEY_KBDINPUTASSIST_NEXTGROUP,
            KEY_KBDINPUTASSIST_ACCEPT => Self::KEY_KBDINPUTASSIST_ACCEPT,
            KEY_KBDINPUTASSIST_CANCEL => Self::KEY_KBDINPUTASSIST_CANCEL,
            KEY_RIGHT_UP => Self::KEY_RIGHT_UP,
            KEY_RIGHT_DOWN => Self::KEY_RIGHT_DOWN,
            KEY_LEFT_UP => Self::KEY_LEFT_UP,
            KEY_LEFT_DOWN => Self::KEY_LEFT_DOWN,
            KEY_ROOT_MENU => Self::KEY_ROOT_MENU,
            KEY_MEDIA_TOP_MENU => Self::KEY_MEDIA_TOP_MENU,
            KEY_NUMERIC_11 => Self::KEY_NUMERIC_11,
            KEY_NUMERIC_12 => Self::KEY_NUMERIC_12,
            KEY_AUDIO_DESC => Self::KEY_AUDIO_DESC,
            KEY_3D_MODE => Self::KEY_3D_MODE,
            KEY_NEXT_FAVORITE => Self::KEY_NEXT_FAVORITE,
            KEY_STOP_RECORD => Self::KEY_STOP_RECORD,
            KEY_PAUSE_RECORD => Self::KEY_PAUSE_RECORD,
            KEY_VOD => Self::KEY_VOD,
            KEY_UNMUTE => Self::KEY_UNMUTE,
            KEY_FASTREVERSE => Self::KEY_FASTREVERSE,
            KEY_SLOWREVERSE => Self::KEY_SLOWREVERSE,
            KEY_DATA => Self::KEY_DATA,
            KEY_ONSCREEN_KEYBOARD => Self::KEY_ONSCREEN_KEYBOARD,
            x => Self::Unknown(x),
        }
    }
}

#[derive(Debug)]
pub enum ButtonCodes {
    //BTN_MISC,
    BTN_0,
    BTN_1,
    BTN_2,
    BTN_3,
    BTN_4,
    BTN_5,
    BTN_6,
    BTN_7,
    BTN_8,
    BTN_9,
    //BTN_MOUSE,
    BTN_LEFT,
    BTN_RIGHT,
    BTN_MIDDLE,
    BTN_SIDE,
    BTN_EXTRA,
    BTN_FORWARD,
    BTN_BACK,
    BTN_TASK,
    BTN_JOYSTICK,
    //BTN_TRIGGER,
    BTN_THUMB,
    BTN_THUMB2,
    BTN_TOP,
    BTN_TOP2,
    BTN_PINKIE,
    BTN_BASE,
    BTN_BASE2,
    BTN_BASE3,
    BTN_BASE4,
    BTN_BASE5,
    BTN_BASE6,
    BTN_DEAD,
    //BTN_GAMEPAD,
    //BTN_SOUTH,
    BTN_A,
    //BTN_EAST,
    BTN_B,
    BTN_C,
    //BTN_NORTH,
    BTN_X,
    //BTN_WEST,
    BTN_Y,
    BTN_Z,
    BTN_TL,
    BTN_TR,
    BTN_TL2,
    BTN_TR2,
    BTN_SELECT,
    BTN_START,
    BTN_MODE,
    BTN_THUMBL,
    BTN_THUMBR,
    BTN_DIGI,
    //BTN_TOOL_PEN,
    BTN_TOOL_RUBBER,
    BTN_TOOL_BRUSH,
    BTN_TOOL_PENCIL,
    BTN_TOOL_AIRBRUSH,
    BTN_TOOL_FINGER,
    BTN_TOOL_MOUSE,
    BTN_TOOL_LENS,
    BTN_TOOL_QUINTTAP, /* Five fingers on trackpad */
    BTN_STYLUS3,
    BTN_TOUCH,
    BTN_STYLUS,
    BTN_STYLUS2,
    BTN_TOOL_DOUBLETAP,
    BTN_TOOL_TRIPLETAP,
    BTN_TOOL_QUADTAP, /* Four fingers on trackpad */
    BTN_WHEEL,
    //BTN_GEAR_DOWN,
    BTN_GEAR_UP,
    BTN_DPAD_UP,
    BTN_DPAD_DOWN,
    BTN_DPAD_LEFT,
    BTN_DPAD_RIGHT,
    //BTN_TRIGGER_HAPPY,
    BTN_TRIGGER_HAPPY1,
    BTN_TRIGGER_HAPPY2,
    BTN_TRIGGER_HAPPY3,
    BTN_TRIGGER_HAPPY4,
    BTN_TRIGGER_HAPPY5,
    BTN_TRIGGER_HAPPY6,
    BTN_TRIGGER_HAPPY7,
    BTN_TRIGGER_HAPPY8,
    BTN_TRIGGER_HAPPY9,
    BTN_TRIGGER_HAPPY10,
    BTN_TRIGGER_HAPPY11,
    BTN_TRIGGER_HAPPY12,
    BTN_TRIGGER_HAPPY13,
    BTN_TRIGGER_HAPPY14,
    BTN_TRIGGER_HAPPY15,
    BTN_TRIGGER_HAPPY16,
    BTN_TRIGGER_HAPPY17,
    BTN_TRIGGER_HAPPY18,
    BTN_TRIGGER_HAPPY19,
    BTN_TRIGGER_HAPPY20,
    BTN_TRIGGER_HAPPY21,
    BTN_TRIGGER_HAPPY22,
    BTN_TRIGGER_HAPPY23,
    BTN_TRIGGER_HAPPY24,
    BTN_TRIGGER_HAPPY25,
    BTN_TRIGGER_HAPPY26,
    BTN_TRIGGER_HAPPY27,
    BTN_TRIGGER_HAPPY28,
    BTN_TRIGGER_HAPPY29,
    BTN_TRIGGER_HAPPY30,
    BTN_TRIGGER_HAPPY31,
    BTN_TRIGGER_HAPPY32,
    BTN_TRIGGER_HAPPY33,
    BTN_TRIGGER_HAPPY34,
    BTN_TRIGGER_HAPPY35,
    BTN_TRIGGER_HAPPY36,
    BTN_TRIGGER_HAPPY37,
    BTN_TRIGGER_HAPPY38,
    BTN_TRIGGER_HAPPY39,
    BTN_TRIGGER_HAPPY40,
    Unknown(u32),
}

impl From<u32> for ButtonCodes {
    fn from(v: u32) -> Self {
        match v {
            //BTN_MISC => Self::BTN_MISC,
            BTN_0 => Self::BTN_0,
            BTN_1 => Self::BTN_1,
            BTN_2 => Self::BTN_2,
            BTN_3 => Self::BTN_3,
            BTN_4 => Self::BTN_4,
            BTN_5 => Self::BTN_5,
            BTN_6 => Self::BTN_6,
            BTN_7 => Self::BTN_7,
            BTN_8 => Self::BTN_8,
            BTN_9 => Self::BTN_9,
            //BTN_MOUSE => Self::BTN_MOUSE,
            BTN_LEFT => Self::BTN_LEFT,
            BTN_RIGHT => Self::BTN_RIGHT,
            BTN_MIDDLE => Self::BTN_MIDDLE,
            BTN_SIDE => Self::BTN_SIDE,
            BTN_EXTRA => Self::BTN_EXTRA,
            BTN_FORWARD => Self::BTN_FORWARD,
            BTN_BACK => Self::BTN_BACK,
            BTN_TASK => Self::BTN_TASK,
            BTN_JOYSTICK => Self::BTN_JOYSTICK,
            //BTN_TRIGGER => Self::BTN_TRIGGER,
            BTN_THUMB => Self::BTN_THUMB,
            BTN_THUMB2 => Self::BTN_THUMB2,
            BTN_TOP => Self::BTN_TOP,
            BTN_TOP2 => Self::BTN_TOP2,
            BTN_PINKIE => Self::BTN_PINKIE,
            BTN_BASE => Self::BTN_BASE,
            BTN_BASE2 => Self::BTN_BASE2,
            BTN_BASE3 => Self::BTN_BASE3,
            BTN_BASE4 => Self::BTN_BASE4,
            BTN_BASE5 => Self::BTN_BASE5,
            BTN_BASE6 => Self::BTN_BASE6,
            BTN_DEAD => Self::BTN_DEAD,
            //BTN_GAMEPAD => Self::BTN_GAMEPAD,
            //BTN_SOUTH => Self::BTN_SOUTH,
            BTN_A => Self::BTN_A,
            //BTN_EAST => Self::BTN_EAST,
            BTN_B => Self::BTN_B,
            BTN_C => Self::BTN_C,
            //BTN_NORTH => Self::BTN_NORTH,
            BTN_X => Self::BTN_X,
            //BTN_WEST => Self::BTN_WEST,
            BTN_Y => Self::BTN_Y,
            BTN_Z => Self::BTN_Z,
            BTN_TL => Self::BTN_TL,
            BTN_TR => Self::BTN_TR,
            BTN_TL2 => Self::BTN_TL2,
            BTN_TR2 => Self::BTN_TR2,
            BTN_SELECT => Self::BTN_SELECT,
            BTN_START => Self::BTN_START,
            BTN_MODE => Self::BTN_MODE,
            BTN_THUMBL => Self::BTN_THUMBL,
            BTN_THUMBR => Self::BTN_THUMBR,
            BTN_DIGI => Self::BTN_DIGI,
            //BTN_TOOL_PEN => Self::BTN_TOOL_PEN,
            BTN_TOOL_RUBBER => Self::BTN_TOOL_RUBBER,
            BTN_TOOL_BRUSH => Self::BTN_TOOL_BRUSH,
            BTN_TOOL_PENCIL => Self::BTN_TOOL_PENCIL,
            BTN_TOOL_AIRBRUSH => Self::BTN_TOOL_AIRBRUSH,
            BTN_TOOL_FINGER => Self::BTN_TOOL_FINGER,
            BTN_TOOL_MOUSE => Self::BTN_TOOL_MOUSE,
            BTN_TOOL_LENS => Self::BTN_TOOL_LENS,
            BTN_TOOL_QUINTTAP => Self::BTN_TOOL_QUINTTAP, /* Five fingers on trackpad */
            BTN_STYLUS3 => Self::BTN_STYLUS3,
            BTN_TOUCH => Self::BTN_TOUCH,
            BTN_STYLUS => Self::BTN_STYLUS,
            BTN_STYLUS2 => Self::BTN_STYLUS2,
            BTN_TOOL_DOUBLETAP => Self::BTN_TOOL_DOUBLETAP,
            BTN_TOOL_TRIPLETAP => Self::BTN_TOOL_TRIPLETAP,
            BTN_TOOL_QUADTAP => Self::BTN_TOOL_QUADTAP, /* Four fingers on trackpad */
            BTN_WHEEL => Self::BTN_WHEEL,
            //BTN_GEAR_DOWN => Self::BTN_GEAR_DOWN,
            BTN_GEAR_UP => Self::BTN_GEAR_UP,
            BTN_DPAD_UP => Self::BTN_DPAD_UP,
            BTN_DPAD_DOWN => Self::BTN_DPAD_DOWN,
            BTN_DPAD_LEFT => Self::BTN_DPAD_LEFT,
            BTN_DPAD_RIGHT => Self::BTN_DPAD_RIGHT,
            //BTN_TRIGGER_HAPPY => Self::BTN_TRIGGER_HAPPY,
            BTN_TRIGGER_HAPPY1 => Self::BTN_TRIGGER_HAPPY1,
            BTN_TRIGGER_HAPPY2 => Self::BTN_TRIGGER_HAPPY2,
            BTN_TRIGGER_HAPPY3 => Self::BTN_TRIGGER_HAPPY3,
            BTN_TRIGGER_HAPPY4 => Self::BTN_TRIGGER_HAPPY4,
            BTN_TRIGGER_HAPPY5 => Self::BTN_TRIGGER_HAPPY5,
            BTN_TRIGGER_HAPPY6 => Self::BTN_TRIGGER_HAPPY6,
            BTN_TRIGGER_HAPPY7 => Self::BTN_TRIGGER_HAPPY7,
            BTN_TRIGGER_HAPPY8 => Self::BTN_TRIGGER_HAPPY8,
            BTN_TRIGGER_HAPPY9 => Self::BTN_TRIGGER_HAPPY9,
            BTN_TRIGGER_HAPPY10 => Self::BTN_TRIGGER_HAPPY10,
            BTN_TRIGGER_HAPPY11 => Self::BTN_TRIGGER_HAPPY11,
            BTN_TRIGGER_HAPPY12 => Self::BTN_TRIGGER_HAPPY12,
            BTN_TRIGGER_HAPPY13 => Self::BTN_TRIGGER_HAPPY13,
            BTN_TRIGGER_HAPPY14 => Self::BTN_TRIGGER_HAPPY14,
            BTN_TRIGGER_HAPPY15 => Self::BTN_TRIGGER_HAPPY15,
            BTN_TRIGGER_HAPPY16 => Self::BTN_TRIGGER_HAPPY16,
            BTN_TRIGGER_HAPPY17 => Self::BTN_TRIGGER_HAPPY17,
            BTN_TRIGGER_HAPPY18 => Self::BTN_TRIGGER_HAPPY18,
            BTN_TRIGGER_HAPPY19 => Self::BTN_TRIGGER_HAPPY19,
            BTN_TRIGGER_HAPPY20 => Self::BTN_TRIGGER_HAPPY20,
            BTN_TRIGGER_HAPPY21 => Self::BTN_TRIGGER_HAPPY21,
            BTN_TRIGGER_HAPPY22 => Self::BTN_TRIGGER_HAPPY22,
            BTN_TRIGGER_HAPPY23 => Self::BTN_TRIGGER_HAPPY23,
            BTN_TRIGGER_HAPPY24 => Self::BTN_TRIGGER_HAPPY24,
            BTN_TRIGGER_HAPPY25 => Self::BTN_TRIGGER_HAPPY25,
            BTN_TRIGGER_HAPPY26 => Self::BTN_TRIGGER_HAPPY26,
            BTN_TRIGGER_HAPPY27 => Self::BTN_TRIGGER_HAPPY27,
            BTN_TRIGGER_HAPPY28 => Self::BTN_TRIGGER_HAPPY28,
            BTN_TRIGGER_HAPPY29 => Self::BTN_TRIGGER_HAPPY29,
            BTN_TRIGGER_HAPPY30 => Self::BTN_TRIGGER_HAPPY30,
            BTN_TRIGGER_HAPPY31 => Self::BTN_TRIGGER_HAPPY31,
            BTN_TRIGGER_HAPPY32 => Self::BTN_TRIGGER_HAPPY32,
            BTN_TRIGGER_HAPPY33 => Self::BTN_TRIGGER_HAPPY33,
            BTN_TRIGGER_HAPPY34 => Self::BTN_TRIGGER_HAPPY34,
            BTN_TRIGGER_HAPPY35 => Self::BTN_TRIGGER_HAPPY35,
            BTN_TRIGGER_HAPPY36 => Self::BTN_TRIGGER_HAPPY36,
            BTN_TRIGGER_HAPPY37 => Self::BTN_TRIGGER_HAPPY37,
            BTN_TRIGGER_HAPPY38 => Self::BTN_TRIGGER_HAPPY38,
            BTN_TRIGGER_HAPPY39 => Self::BTN_TRIGGER_HAPPY39,
            BTN_TRIGGER_HAPPY40 => Self::BTN_TRIGGER_HAPPY40,
            x => Self::Unknown(x),
        }
    }
}
