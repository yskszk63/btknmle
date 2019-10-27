#![allow(non_camel_case_types)]

const KEY_RESERVED: u32 = 0;
const KEY_ESC: u32 = 1;
const KEY_1: u32 = 2;
const KEY_2: u32 = 3;
const KEY_3: u32 = 4;
const KEY_4: u32 = 5;
const KEY_5: u32 = 6;
const KEY_6: u32 = 7;
const KEY_7: u32 = 8;
const KEY_8: u32 = 9;
const KEY_9: u32 = 10;
const KEY_0: u32 = 11;
const KEY_MINUS: u32 = 12;
const KEY_EQUAL: u32 = 13;
const KEY_BACKSPACE: u32 = 14;
const KEY_TAB: u32 = 15;
const KEY_Q: u32 = 16;
const KEY_W: u32 = 17;
const KEY_E: u32 = 18;
const KEY_R: u32 = 19;
const KEY_T: u32 = 20;
const KEY_Y: u32 = 21;
const KEY_U: u32 = 22;
const KEY_I: u32 = 23;
const KEY_O: u32 = 24;
const KEY_P: u32 = 25;
const KEY_LEFTBRACE: u32 = 26;
const KEY_RIGHTBRACE: u32 = 27;
const KEY_ENTER: u32 = 28;
const KEY_LEFTCTRL: u32 = 29;
const KEY_A: u32 = 30;
const KEY_S: u32 = 31;
const KEY_D: u32 = 32;
const KEY_F: u32 = 33;
const KEY_G: u32 = 34;
const KEY_H: u32 = 35;
const KEY_J: u32 = 36;
const KEY_K: u32 = 37;
const KEY_L: u32 = 38;
const KEY_SEMICOLON: u32 = 39;
const KEY_APOSTROPHE: u32 = 40;
const KEY_GRAVE: u32 = 41;
const KEY_LEFTSHIFT: u32 = 42;
const KEY_BACKSLASH: u32 = 43;
const KEY_Z: u32 = 44;
const KEY_X: u32 = 45;
const KEY_C: u32 = 46;
const KEY_V: u32 = 47;
const KEY_B: u32 = 48;
const KEY_N: u32 = 49;
const KEY_M: u32 = 50;
const KEY_COMMA: u32 = 51;
const KEY_DOT: u32 = 52;
const KEY_SLASH: u32 = 53;
const KEY_RIGHTSHIFT: u32 = 54;
const KEY_KPASTERISK: u32 = 55;
const KEY_LEFTALT: u32 = 56;
const KEY_SPACE: u32 = 57;
const KEY_CAPSLOCK: u32 = 58;
const KEY_F1: u32 = 59;
const KEY_F2: u32 = 60;
const KEY_F3: u32 = 61;
const KEY_F4: u32 = 62;
const KEY_F5: u32 = 63;
const KEY_F6: u32 = 64;
const KEY_F7: u32 = 65;
const KEY_F8: u32 = 66;
const KEY_F9: u32 = 67;
const KEY_F10: u32 = 68;
const KEY_NUMLOCK: u32 = 69;
const KEY_SCROLLLOCK: u32 = 70;
const KEY_KP7: u32 = 71;
const KEY_KP8: u32 = 72;
const KEY_KP9: u32 = 73;
const KEY_KPMINUS: u32 = 74;
const KEY_KP4: u32 = 75;
const KEY_KP5: u32 = 76;
const KEY_KP6: u32 = 77;
const KEY_KPPLUS: u32 = 78;
const KEY_KP1: u32 = 79;
const KEY_KP2: u32 = 80;
const KEY_KP3: u32 = 81;
const KEY_KP0: u32 = 82;
const KEY_KPDOT: u32 = 83;
const KEY_ZENKAKUHANKAKU: u32 = 85;
const KEY_102ND: u32 = 86;
const KEY_F11: u32 = 87;
const KEY_F12: u32 = 88;
const KEY_RO: u32 = 89;
const KEY_KATAKANA: u32 = 90;
const KEY_HIRAGANA: u32 = 91;
const KEY_HENKAN: u32 = 92;
const KEY_KATAKANAHIRAGANA: u32 = 93;
const KEY_MUHENKAN: u32 = 94;
const KEY_KPJPCOMMA: u32 = 95;
const KEY_KPENTER: u32 = 96;
const KEY_RIGHTCTRL: u32 = 97;
const KEY_KPSLASH: u32 = 98;
const KEY_SYSRQ: u32 = 99;
const KEY_RIGHTALT: u32 = 100;
const KEY_LINEFEED: u32 = 101;
const KEY_HOME: u32 = 102;
const KEY_UP: u32 = 103;
const KEY_PAGEUP: u32 = 104;
const KEY_LEFT: u32 = 105;
const KEY_RIGHT: u32 = 106;
const KEY_END: u32 = 107;
const KEY_DOWN: u32 = 108;
const KEY_PAGEDOWN: u32 = 109;
const KEY_INSERT: u32 = 110;
const KEY_DELETE: u32 = 111;
const KEY_MACRO: u32 = 112;
const KEY_MUTE: u32 = 113;
const KEY_VOLUMEDOWN: u32 = 114;
const KEY_VOLUMEUP: u32 = 115;
const KEY_POWER: u32 = 116;
const KEY_KPEQUAL: u32 = 117;
const KEY_KPPLUSMINUS: u32 = 118;
const KEY_PAUSE: u32 = 119;
const KEY_SCALE: u32 = 120;
const KEY_KPCOMMA: u32 = 121;
const KEY_HANGEUL: u32 = 122;
//const KEY_HANGUEL: u32 = KEY_HANGEUL;
const KEY_HANJA: u32 = 123;
const KEY_YEN: u32 = 124;
const KEY_LEFTMETA: u32 = 125;
const KEY_RIGHTMETA: u32 = 126;
const KEY_COMPOSE: u32 = 127;
const KEY_STOP: u32 = 128;
const KEY_AGAIN: u32 = 129;
const KEY_PROPS: u32 = 130;
const KEY_UNDO: u32 = 131;
const KEY_FRONT: u32 = 132;
const KEY_COPY: u32 = 133;
const KEY_OPEN: u32 = 134;
const KEY_PASTE: u32 = 135;
const KEY_FIND: u32 = 136;
const KEY_CUT: u32 = 137;
const KEY_HELP: u32 = 138;
const KEY_MENU: u32 = 139;
const KEY_CALC: u32 = 140;
const KEY_SETUP: u32 = 141;
const KEY_SLEEP: u32 = 142;
const KEY_WAKEUP: u32 = 143;
const KEY_FILE: u32 = 144;
const KEY_SENDFILE: u32 = 145;
const KEY_DELETEFILE: u32 = 146;
const KEY_XFER: u32 = 147;
const KEY_PROG1: u32 = 148;
const KEY_PROG2: u32 = 149;
const KEY_WWW: u32 = 150;
const KEY_MSDOS: u32 = 151;
const KEY_COFFEE: u32 = 152;
//const KEY_SCREENLOCK: u32 = KEY_COFFEE;
const KEY_ROTATE_DISPLAY: u32 = 153;
//const KEY_DIRECTION: u32 = KEY_ROTATE_DISPLAY;
const KEY_CYCLEWINDOWS: u32 = 154;
const KEY_MAIL: u32 = 155;
const KEY_BOOKMARKS: u32 = 156;
const KEY_COMPUTER: u32 = 157;
const KEY_BACK: u32 = 158;
const KEY_FORWARD: u32 = 159;
const KEY_CLOSECD: u32 = 160;
const KEY_EJECTCD: u32 = 161;
const KEY_EJECTCLOSECD: u32 = 162;
const KEY_NEXTSONG: u32 = 163;
const KEY_PLAYPAUSE: u32 = 164;
const KEY_PREVIOUSSONG: u32 = 165;
const KEY_STOPCD: u32 = 166;
const KEY_RECORD: u32 = 167;
const KEY_REWIND: u32 = 168;
const KEY_PHONE: u32 = 169;
const KEY_ISO: u32 = 170;
const KEY_CONFIG: u32 = 171;
const KEY_HOMEPAGE: u32 = 172;
const KEY_REFRESH: u32 = 173;
const KEY_EXIT: u32 = 174;
const KEY_MOVE: u32 = 175;
const KEY_EDIT: u32 = 176;
const KEY_SCROLLUP: u32 = 177;
const KEY_SCROLLDOWN: u32 = 178;
const KEY_KPLEFTPAREN: u32 = 179;
const KEY_KPRIGHTPAREN: u32 = 180;
const KEY_NEW: u32 = 181;
const KEY_REDO: u32 = 182;
const KEY_F13: u32 = 183;
const KEY_F14: u32 = 184;
const KEY_F15: u32 = 185;
const KEY_F16: u32 = 186;
const KEY_F17: u32 = 187;
const KEY_F18: u32 = 188;
const KEY_F19: u32 = 189;
const KEY_F20: u32 = 190;
const KEY_F21: u32 = 191;
const KEY_F22: u32 = 192;
const KEY_F23: u32 = 193;
const KEY_F24: u32 = 194;
const KEY_PLAYCD: u32 = 200;
const KEY_PAUSECD: u32 = 201;
const KEY_PROG3: u32 = 202;
const KEY_PROG4: u32 = 203;
const KEY_DASHBOARD: u32 = 204;
const KEY_SUSPEND: u32 = 205;
const KEY_CLOSE: u32 = 206;
const KEY_PLAY: u32 = 207;
const KEY_FASTFORWARD: u32 = 208;
const KEY_BASSBOOST: u32 = 209;
const KEY_PRINT: u32 = 210;
const KEY_HP: u32 = 211;
const KEY_CAMERA: u32 = 212;
const KEY_SOUND: u32 = 213;
const KEY_QUESTION: u32 = 214;
const KEY_EMAIL: u32 = 215;
const KEY_CHAT: u32 = 216;
const KEY_SEARCH: u32 = 217;
const KEY_CONNECT: u32 = 218;
const KEY_FINANCE: u32 = 219;
const KEY_SPORT: u32 = 220;
const KEY_SHOP: u32 = 221;
const KEY_ALTERASE: u32 = 222;
const KEY_CANCEL: u32 = 223;
const KEY_BRIGHTNESSDOWN: u32 = 224;
const KEY_BRIGHTNESSUP: u32 = 225;
const KEY_MEDIA: u32 = 226;
const KEY_SWITCHVIDEOMODE: u32 = 227;
const KEY_KBDILLUMTOGGLE: u32 = 228;
const KEY_KBDILLUMDOWN: u32 = 229;
const KEY_KBDILLUMUP: u32 = 230;
const KEY_SEND: u32 = 231;
const KEY_REPLY: u32 = 232;
const KEY_FORWARDMAIL: u32 = 233;
const KEY_SAVE: u32 = 234;
const KEY_DOCUMENTS: u32 = 235;
const KEY_BATTERY: u32 = 236;
const KEY_BLUETOOTH: u32 = 237;
const KEY_WLAN: u32 = 238;
const KEY_UWB: u32 = 239;
const KEY_UNKNOWN: u32 = 240;
const KEY_VIDEO_NEXT: u32 = 241;
const KEY_VIDEO_PREV: u32 = 242;
const KEY_BRIGHTNESS_CYCLE: u32 = 243;
const KEY_BRIGHTNESS_AUTO: u32 = 244;
//const KEY_BRIGHTNESS_ZERO: u32 = KEY_BRIGHTNESS_AUTO;
const KEY_DISPLAY_OFF: u32 = 245;
const KEY_WWAN: u32 = 246;
//const KEY_WIMAX: u32 = KEY_WWAN;
const KEY_RFKILL: u32 = 247;
const KEY_MICMUTE: u32 = 248;
const KEY_OK: u32 = 0x160;
const KEY_SELECT: u32 = 0x161;
const KEY_GOTO: u32 = 0x162;
const KEY_CLEAR: u32 = 0x163;
const KEY_POWER2: u32 = 0x164;
const KEY_OPTION: u32 = 0x165;
const KEY_INFO: u32 = 0x166;
const KEY_TIME: u32 = 0x167;
const KEY_VENDOR: u32 = 0x168;
const KEY_ARCHIVE: u32 = 0x169;
const KEY_PROGRAM: u32 = 0x16a;
const KEY_CHANNEL: u32 = 0x16b;
const KEY_FAVORITES: u32 = 0x16c;
const KEY_EPG: u32 = 0x16d;
const KEY_PVR: u32 = 0x16e;
const KEY_MHP: u32 = 0x16f;
const KEY_LANGUAGE: u32 = 0x170;
const KEY_TITLE: u32 = 0x171;
const KEY_SUBTITLE: u32 = 0x172;
const KEY_ANGLE: u32 = 0x173;
const KEY_FULL_SCREEN: u32 = 0x174;
//const KEY_ZOOM: u32 = KEY_FULL_SCREEN;
const KEY_MODE: u32 = 0x175;
const KEY_KEYBOARD: u32 = 0x176;
const KEY_ASPECT_RATIO: u32 = 0x177;
//const KEY_SCREEN: u32 = KEY_ASPECT_RATIO;
const KEY_PC: u32 = 0x178;
const KEY_TV: u32 = 0x179;
const KEY_TV2: u32 = 0x17a;
const KEY_VCR: u32 = 0x17b;
const KEY_VCR2: u32 = 0x17c;
const KEY_SAT: u32 = 0x17d;
const KEY_SAT2: u32 = 0x17e;
const KEY_CD: u32 = 0x17f;
const KEY_TAPE: u32 = 0x180;
const KEY_RADIO: u32 = 0x181;
const KEY_TUNER: u32 = 0x182;
const KEY_PLAYER: u32 = 0x183;
const KEY_TEXT: u32 = 0x184;
const KEY_DVD: u32 = 0x185;
const KEY_AUX: u32 = 0x186;
const KEY_MP3: u32 = 0x187;
const KEY_AUDIO: u32 = 0x188;
const KEY_VIDEO: u32 = 0x189;
const KEY_DIRECTORY: u32 = 0x18a;
const KEY_LIST: u32 = 0x18b;
const KEY_MEMO: u32 = 0x18c;
const KEY_CALENDAR: u32 = 0x18d;
const KEY_RED: u32 = 0x18e;
const KEY_GREEN: u32 = 0x18f;
const KEY_YELLOW: u32 = 0x190;
const KEY_BLUE: u32 = 0x191;
const KEY_CHANNELUP: u32 = 0x192;
const KEY_CHANNELDOWN: u32 = 0x193;
const KEY_FIRST: u32 = 0x194;
const KEY_LAST: u32 = 0x195;
const KEY_AB: u32 = 0x196;
const KEY_NEXT: u32 = 0x197;
const KEY_RESTART: u32 = 0x198;
const KEY_SLOW: u32 = 0x199;
const KEY_SHUFFLE: u32 = 0x19a;
const KEY_BREAK: u32 = 0x19b;
const KEY_PREVIOUS: u32 = 0x19c;
const KEY_DIGITS: u32 = 0x19d;
const KEY_TEEN: u32 = 0x19e;
const KEY_TWEN: u32 = 0x19f;
const KEY_VIDEOPHONE: u32 = 0x1a0;
const KEY_GAMES: u32 = 0x1a1;
const KEY_ZOOMIN: u32 = 0x1a2;
const KEY_ZOOMOUT: u32 = 0x1a3;
const KEY_ZOOMRESET: u32 = 0x1a4;
const KEY_WORDPROCESSOR: u32 = 0x1a5;
const KEY_EDITOR: u32 = 0x1a6;
const KEY_SPREADSHEET: u32 = 0x1a7;
const KEY_GRAPHICSEDITOR: u32 = 0x1a8;
const KEY_PRESENTATION: u32 = 0x1a9;
const KEY_DATABASE: u32 = 0x1aa;
const KEY_NEWS: u32 = 0x1ab;
const KEY_VOICEMAIL: u32 = 0x1ac;
const KEY_ADDRESSBOOK: u32 = 0x1ad;
const KEY_MESSENGER: u32 = 0x1ae;
const KEY_DISPLAYTOGGLE: u32 = 0x1af;
//const KEY_BRIGHTNESS_TOGGLE: u32 = KEY_DISPLAYTOGGLE;
const KEY_SPELLCHECK: u32 = 0x1b0;
const KEY_LOGOFF: u32 = 0x1b1;
const KEY_DOLLAR: u32 = 0x1b2;
const KEY_EURO: u32 = 0x1b3;
const KEY_FRAMEBACK: u32 = 0x1b4;
const KEY_FRAMEFORWARD: u32 = 0x1b5;
const KEY_CONTEXT_MENU: u32 = 0x1b6;
const KEY_MEDIA_REPEAT: u32 = 0x1b7;
const KEY_10CHANNELSUP: u32 = 0x1b8;
const KEY_10CHANNELSDOWN: u32 = 0x1b9;
const KEY_IMAGES: u32 = 0x1ba;
const KEY_DEL_EOL: u32 = 0x1c0;
const KEY_DEL_EOS: u32 = 0x1c1;
const KEY_INS_LINE: u32 = 0x1c2;
const KEY_DEL_LINE: u32 = 0x1c3;
const KEY_FN: u32 = 0x1d0;
const KEY_FN_ESC: u32 = 0x1d1;
const KEY_FN_F1: u32 = 0x1d2;
const KEY_FN_F2: u32 = 0x1d3;
const KEY_FN_F3: u32 = 0x1d4;
const KEY_FN_F4: u32 = 0x1d5;
const KEY_FN_F5: u32 = 0x1d6;
const KEY_FN_F6: u32 = 0x1d7;
const KEY_FN_F7: u32 = 0x1d8;
const KEY_FN_F8: u32 = 0x1d9;
const KEY_FN_F9: u32 = 0x1da;
const KEY_FN_F10: u32 = 0x1db;
const KEY_FN_F11: u32 = 0x1dc;
const KEY_FN_F12: u32 = 0x1dd;
const KEY_FN_1: u32 = 0x1de;
const KEY_FN_2: u32 = 0x1df;
const KEY_FN_D: u32 = 0x1e0;
const KEY_FN_E: u32 = 0x1e1;
const KEY_FN_F: u32 = 0x1e2;
const KEY_FN_S: u32 = 0x1e3;
const KEY_FN_B: u32 = 0x1e4;
const KEY_BRL_DOT1: u32 = 0x1f1;
const KEY_BRL_DOT2: u32 = 0x1f2;
const KEY_BRL_DOT3: u32 = 0x1f3;
const KEY_BRL_DOT4: u32 = 0x1f4;
const KEY_BRL_DOT5: u32 = 0x1f5;
const KEY_BRL_DOT6: u32 = 0x1f6;
const KEY_BRL_DOT7: u32 = 0x1f7;
const KEY_BRL_DOT8: u32 = 0x1f8;
const KEY_BRL_DOT9: u32 = 0x1f9;
const KEY_BRL_DOT10: u32 = 0x1fa;
const KEY_NUMERIC_0: u32 = 0x200;
const KEY_NUMERIC_1: u32 = 0x201;
const KEY_NUMERIC_2: u32 = 0x202;
const KEY_NUMERIC_3: u32 = 0x203;
const KEY_NUMERIC_4: u32 = 0x204;
const KEY_NUMERIC_5: u32 = 0x205;
const KEY_NUMERIC_6: u32 = 0x206;
const KEY_NUMERIC_7: u32 = 0x207;
const KEY_NUMERIC_8: u32 = 0x208;
const KEY_NUMERIC_9: u32 = 0x209;
const KEY_NUMERIC_STAR: u32 = 0x20a;
const KEY_NUMERIC_POUND: u32 = 0x20b;
const KEY_NUMERIC_A: u32 = 0x20c;
const KEY_NUMERIC_B: u32 = 0x20d;
const KEY_NUMERIC_C: u32 = 0x20e;
const KEY_NUMERIC_D: u32 = 0x20f;
const KEY_CAMERA_FOCUS: u32 = 0x210;
const KEY_WPS_BUTTON: u32 = 0x211;
const KEY_TOUCHPAD_TOGGLE: u32 = 0x212;
const KEY_TOUCHPAD_ON: u32 = 0x213;
const KEY_TOUCHPAD_OFF: u32 = 0x214;
const KEY_CAMERA_ZOOMIN: u32 = 0x215;
const KEY_CAMERA_ZOOMOUT: u32 = 0x216;
const KEY_CAMERA_UP: u32 = 0x217;
const KEY_CAMERA_DOWN: u32 = 0x218;
const KEY_CAMERA_LEFT: u32 = 0x219;
const KEY_CAMERA_RIGHT: u32 = 0x21a;
const KEY_ATTENDANT_ON: u32 = 0x21b;
const KEY_ATTENDANT_OFF: u32 = 0x21c;
const KEY_ATTENDANT_TOGGLE: u32 = 0x21d;
const KEY_LIGHTS_TOGGLE: u32 = 0x21e;
const KEY_ALS_TOGGLE: u32 = 0x230;
const KEY_ROTATE_LOCK_TOGGLE: u32 = 0x231;
const KEY_BUTTONCONFIG: u32 = 0x240;
const KEY_TASKMANAGER: u32 = 0x241;
const KEY_JOURNAL: u32 = 0x242;
const KEY_CONTROLPANEL: u32 = 0x243;
const KEY_APPSELECT: u32 = 0x244;
const KEY_SCREENSAVER: u32 = 0x245;
const KEY_VOICECOMMAND: u32 = 0x246;
const KEY_ASSISTANT: u32 = 0x247;
const KEY_KBD_LAYOUT_NEXT: u32 = 0x248;
const KEY_BRIGHTNESS_MIN: u32 = 0x250;
const KEY_BRIGHTNESS_MAX: u32 = 0x251;
const KEY_KBDINPUTASSIST_PREV: u32 = 0x260;
const KEY_KBDINPUTASSIST_NEXT: u32 = 0x261;
const KEY_KBDINPUTASSIST_PREVGROUP: u32 = 0x262;
const KEY_KBDINPUTASSIST_NEXTGROUP: u32 = 0x263;
const KEY_KBDINPUTASSIST_ACCEPT: u32 = 0x264;
const KEY_KBDINPUTASSIST_CANCEL: u32 = 0x265;
const KEY_RIGHT_UP: u32 = 0x266;
const KEY_RIGHT_DOWN: u32 = 0x267;
const KEY_LEFT_UP: u32 = 0x268;
const KEY_LEFT_DOWN: u32 = 0x269;
const KEY_ROOT_MENU: u32 = 0x26a;
const KEY_MEDIA_TOP_MENU: u32 = 0x26b;
const KEY_NUMERIC_11: u32 = 0x26c;
const KEY_NUMERIC_12: u32 = 0x26d;
const KEY_AUDIO_DESC: u32 = 0x26e;
const KEY_3D_MODE: u32 = 0x26f;
const KEY_NEXT_FAVORITE: u32 = 0x270;
const KEY_STOP_RECORD: u32 = 0x271;
const KEY_PAUSE_RECORD: u32 = 0x272;
const KEY_VOD: u32 = 0x273;
const KEY_UNMUTE: u32 = 0x274;
const KEY_FASTREVERSE: u32 = 0x275;
const KEY_SLOWREVERSE: u32 = 0x276;
const KEY_DATA: u32 = 0x277;
const KEY_ONSCREEN_KEYBOARD: u32 = 0x278;

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

//const BTN_MISC:u32 = 0x100;
const BTN_0: u32 = 0x100;
const BTN_1: u32 = 0x101;
const BTN_2: u32 = 0x102;
const BTN_3: u32 = 0x103;
const BTN_4: u32 = 0x104;
const BTN_5: u32 = 0x105;
const BTN_6: u32 = 0x106;
const BTN_7: u32 = 0x107;
const BTN_8: u32 = 0x108;
const BTN_9: u32 = 0x109;
//const BTN_MOUSE:u32 = 0x110;
const BTN_LEFT: u32 = 0x110;
const BTN_RIGHT: u32 = 0x111;
const BTN_MIDDLE: u32 = 0x112;
const BTN_SIDE: u32 = 0x113;
const BTN_EXTRA: u32 = 0x114;
const BTN_FORWARD: u32 = 0x115;
const BTN_BACK: u32 = 0x116;
const BTN_TASK: u32 = 0x117;
const BTN_JOYSTICK: u32 = 0x120;
//const BTN_TRIGGER:u32 = 0x120;
const BTN_THUMB: u32 = 0x121;
const BTN_THUMB2: u32 = 0x122;
const BTN_TOP: u32 = 0x123;
const BTN_TOP2: u32 = 0x124;
const BTN_PINKIE: u32 = 0x125;
const BTN_BASE: u32 = 0x126;
const BTN_BASE2: u32 = 0x127;
const BTN_BASE3: u32 = 0x128;
const BTN_BASE4: u32 = 0x129;
const BTN_BASE5: u32 = 0x12a;
const BTN_BASE6: u32 = 0x12b;
const BTN_DEAD: u32 = 0x12f;
//const BTN_GAMEPAD:u32 = 0x130;
const BTN_SOUTH: u32 = 0x130;
const BTN_A: u32 = BTN_SOUTH;
const BTN_EAST: u32 = 0x131;
const BTN_B: u32 = BTN_EAST;
const BTN_C: u32 = 0x132;
const BTN_NORTH: u32 = 0x133;
const BTN_X: u32 = BTN_NORTH;
const BTN_WEST: u32 = 0x134;
const BTN_Y: u32 = BTN_WEST;
const BTN_Z: u32 = 0x135;
const BTN_TL: u32 = 0x136;
const BTN_TR: u32 = 0x137;
const BTN_TL2: u32 = 0x138;
const BTN_TR2: u32 = 0x139;
const BTN_SELECT: u32 = 0x13a;
const BTN_START: u32 = 0x13b;
const BTN_MODE: u32 = 0x13c;
const BTN_THUMBL: u32 = 0x13d;
const BTN_THUMBR: u32 = 0x13e;
const BTN_DIGI: u32 = 0x140;
//const BTN_TOOL_PEN:u32 = 0x140;
const BTN_TOOL_RUBBER: u32 = 0x141;
const BTN_TOOL_BRUSH: u32 = 0x142;
const BTN_TOOL_PENCIL: u32 = 0x143;
const BTN_TOOL_AIRBRUSH: u32 = 0x144;
const BTN_TOOL_FINGER: u32 = 0x145;
const BTN_TOOL_MOUSE: u32 = 0x146;
const BTN_TOOL_LENS: u32 = 0x147;
const BTN_TOOL_QUINTTAP: u32 = 0x148; /* Five fingers on trackpad */
const BTN_STYLUS3: u32 = 0x149;
const BTN_TOUCH: u32 = 0x14a;
const BTN_STYLUS: u32 = 0x14b;
const BTN_STYLUS2: u32 = 0x14c;
const BTN_TOOL_DOUBLETAP: u32 = 0x14d;
const BTN_TOOL_TRIPLETAP: u32 = 0x14e;
const BTN_TOOL_QUADTAP: u32 = 0x14f; /* Four fingers on trackpad */
const BTN_WHEEL: u32 = 0x150;
//const BTN_GEAR_DOWN:u32 = 0x150;
const BTN_GEAR_UP: u32 = 0x151;
const BTN_DPAD_UP: u32 = 0x220;
const BTN_DPAD_DOWN: u32 = 0x221;
const BTN_DPAD_LEFT: u32 = 0x222;
const BTN_DPAD_RIGHT: u32 = 0x223;
//const BTN_TRIGGER_HAPPY:u32 = 0x2c0;
const BTN_TRIGGER_HAPPY1: u32 = 0x2c0;
const BTN_TRIGGER_HAPPY2: u32 = 0x2c1;
const BTN_TRIGGER_HAPPY3: u32 = 0x2c2;
const BTN_TRIGGER_HAPPY4: u32 = 0x2c3;
const BTN_TRIGGER_HAPPY5: u32 = 0x2c4;
const BTN_TRIGGER_HAPPY6: u32 = 0x2c5;
const BTN_TRIGGER_HAPPY7: u32 = 0x2c6;
const BTN_TRIGGER_HAPPY8: u32 = 0x2c7;
const BTN_TRIGGER_HAPPY9: u32 = 0x2c8;
const BTN_TRIGGER_HAPPY10: u32 = 0x2c9;
const BTN_TRIGGER_HAPPY11: u32 = 0x2ca;
const BTN_TRIGGER_HAPPY12: u32 = 0x2cb;
const BTN_TRIGGER_HAPPY13: u32 = 0x2cc;
const BTN_TRIGGER_HAPPY14: u32 = 0x2cd;
const BTN_TRIGGER_HAPPY15: u32 = 0x2ce;
const BTN_TRIGGER_HAPPY16: u32 = 0x2cf;
const BTN_TRIGGER_HAPPY17: u32 = 0x2d0;
const BTN_TRIGGER_HAPPY18: u32 = 0x2d1;
const BTN_TRIGGER_HAPPY19: u32 = 0x2d2;
const BTN_TRIGGER_HAPPY20: u32 = 0x2d3;
const BTN_TRIGGER_HAPPY21: u32 = 0x2d4;
const BTN_TRIGGER_HAPPY22: u32 = 0x2d5;
const BTN_TRIGGER_HAPPY23: u32 = 0x2d6;
const BTN_TRIGGER_HAPPY24: u32 = 0x2d7;
const BTN_TRIGGER_HAPPY25: u32 = 0x2d8;
const BTN_TRIGGER_HAPPY26: u32 = 0x2d9;
const BTN_TRIGGER_HAPPY27: u32 = 0x2da;
const BTN_TRIGGER_HAPPY28: u32 = 0x2db;
const BTN_TRIGGER_HAPPY29: u32 = 0x2dc;
const BTN_TRIGGER_HAPPY30: u32 = 0x2dd;
const BTN_TRIGGER_HAPPY31: u32 = 0x2de;
const BTN_TRIGGER_HAPPY32: u32 = 0x2df;
const BTN_TRIGGER_HAPPY33: u32 = 0x2e0;
const BTN_TRIGGER_HAPPY34: u32 = 0x2e1;
const BTN_TRIGGER_HAPPY35: u32 = 0x2e2;
const BTN_TRIGGER_HAPPY36: u32 = 0x2e3;
const BTN_TRIGGER_HAPPY37: u32 = 0x2e4;
const BTN_TRIGGER_HAPPY38: u32 = 0x2e5;
const BTN_TRIGGER_HAPPY39: u32 = 0x2e6;
const BTN_TRIGGER_HAPPY40: u32 = 0x2e7;

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
