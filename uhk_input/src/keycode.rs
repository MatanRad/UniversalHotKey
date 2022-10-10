use num_derive::{FromPrimitive, ToPrimitive};

use crate::modifiers::Modifiers;

#[derive(FromPrimitive, ToPrimitive, Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyCode {
    RESERVED = 0,
    ESC = 1,
    ALPHA1 = 2,
    ALPHA2 = 3,
    ALPHA3 = 4,
    ALPHA4 = 5,
    ALPHA5 = 6,
    ALPHA6 = 7,
    ALPHA7 = 8,
    ALPHA8 = 9,
    ALPHA9 = 10,
    ALPHA0 = 11,
    MINUS = 12,
    EQUAL = 13,
    BACKSPACE = 14,
    TAB = 15,
    Q = 16,
    W = 17,
    E = 18,
    R = 19,
    T = 20,
    Y = 21,
    U = 22,
    I = 23,
    O = 24,
    P = 25,
    LEFTBRACE = 26,
    RIGHTBRACE = 27,
    ENTER = 28,
    LEFTCTRL = 29,
    A = 30,
    S = 31,
    D = 32,
    F = 33,
    G = 34,
    H = 35,
    J = 36,
    K = 37,
    L = 38,
    SEMICOLON = 39,
    APOSTROPHE = 40,
    GRAVE = 41,
    LEFTSHIFT = 42,
    BACKSLASH = 43,
    Z = 44,
    X = 45,
    C = 46,
    V = 47,
    B = 48,
    N = 49,
    M = 50,
    COMMA = 51,
    DOT = 52,
    SLASH = 53,
    RIGHTSHIFT = 54,
    KPASTERISK = 55,
    LEFTALT = 56,
    SPACE = 57,
    CAPSLOCK = 58,
    F1 = 59,
    F2 = 60,
    F3 = 61,
    F4 = 62,
    F5 = 63,
    F6 = 64,
    F7 = 65,
    F8 = 66,
    F9 = 67,
    F10 = 68,
    NUMLOCK = 69,
    SCROLLLOCK = 70,
    KP7 = 71,
    KP8 = 72,
    KP9 = 73,
    KPMINUS = 74,
    KP4 = 75,
    KP5 = 76,
    KP6 = 77,
    KPPLUS = 78,
    KP1 = 79,
    KP2 = 80,
    KP3 = 81,
    KP0 = 82,
    KPDOT = 83,

    ZENKAKUHANKAKU = 85,
    KEY102ND = 86,
    F11 = 87,
    F12 = 88,
    RO = 89,
    KATAKANA = 90,
    HIRAGANA = 91,
    HENKAN = 92,
    KATAKANAHIRAGANA = 93,
    MUHENKAN = 94,
    KPJPCOMMA = 95,
    KPENTER = 96,
    RIGHTCTRL = 97,
    KPSLASH = 98,
    SYSRQ = 99,
    RIGHTALT = 100,
    LINEFEED = 101,
    HOME = 102,
    UP = 103,
    PAGEUP = 104,
    LEFT = 105,
    RIGHT = 106,
    END = 107,
    DOWN = 108,
    PAGEDOWN = 109,
    INSERT = 110,
    DELETE = 111,
    MACRO = 112,
    MUTE = 113,
    VOLUMEDOWN = 114,
    VOLUMEUP = 115,
    POWER = 116,
    KPEQUAL = 117,
    KPPLUSMINUS = 118,
    PAUSE = 119,
    SCALE = 120,

    KPCOMMA = 121,
    HANGEUL = 122,
    HANJA = 123,
    YEN = 124,
    LEFTMETA = 125,
    RIGHTMETA = 126,
    COMPOSE = 127,

    STOP = 128,
    AGAIN = 129,
    PROPS = 130,
    UNDO = 131,
    FRONT = 132,
    COPY = 133,
    OPEN = 134,
    PASTE = 135,
    FIND = 136,
    CUT = 137,
    HELP = 138,
    MENU = 139,
    CALC = 140,
    SETUP = 141,
    SLEEP = 142,
    WAKEUP = 143,
    FILE = 144,
    SENDFILE = 145,
    DELETEFILE = 146,
    XFER = 147,
    PROG1 = 148,
    PROG2 = 149,
    WWW = 150,
    MSDOS = 151,
    SCREENLOCK = 152,
    DIRECTION = 153,
    CYCLEWINDOWS = 154,
    MAIL = 155,
    BOOKMARKS = 156,
    COMPUTER = 157,
    BACK = 158,
    FORWARD = 159,
    CLOSECD = 160,
    EJECTCD = 161,
    EJECTCLOSECD = 162,
    NEXTSONG = 163,
    PLAYPAUSE = 164,
    PREVIOUSSONG = 165,
    STOPCD = 166,
    RECORD = 167,
    REWIND = 168,
    PHONE = 169,
    ISO = 170,
    CONFIG = 171,
    HOMEPAGE = 172,
    REFRESH = 173,
    EXIT = 174,
    MOVE = 175,
    EDIT = 176,
    SCROLLUP = 177,
    SCROLLDOWN = 178,
    KPLEFTPAREN = 179,
    KPRIGHTPAREN = 180,
    NEW = 181,
    REDO = 182,

    F13 = 183,
    F14 = 184,
    F15 = 185,
    F16 = 186,
    F17 = 187,
    F18 = 188,
    F19 = 189,
    F20 = 190,
    F21 = 191,
    F22 = 192,
    F23 = 193,
    F24 = 194,

    PLAYCD = 200,
    PAUSECD = 201,
    PROG3 = 202,
    PROG4 = 203,
    DASHBOARD = 204,
    SUSPEND = 205,
    CLOSE = 206,
    PLAY = 207,
    FASTFORWARD = 208,
    BASSBOOST = 209,
    PRINT = 210,
    HP = 211,
    CAMERA = 212,
    SOUND = 213,
    QUESTION = 214,
    EMAIL = 215,
    CHAT = 216,
    SEARCH = 217,
    CONNECT = 218,
    FINANCE = 219,
    SPORT = 220,
    SHOP = 221,
    ALTERASE = 222,
    CANCEL = 223,
    BRIGHTNESSDOWN = 224,
    BRIGHTNESSUP = 225,
    MEDIA = 226,

    SWITCHVIDEOMODE = 227,
    KBDILLUMTOGGLE = 228,
    KBDILLUMDOWN = 229,
    KBDILLUMUP = 230,

    SEND = 231,
    REPLY = 232,
    FORWARDMAIL = 233,
    SAVE = 234,
    DOCUMENTS = 235,

    BATTERY = 236,

    BLUETOOTH = 237,
    WLAN = 238,
    UWB = 239,

    UNKNOWN = 240,

    VIDEONEXT = 241,
    VIDEOPREV = 242,
    BRIGHTNESSCYCLE = 243,
    BRIGHTNESSZERO = 244,
    DISPLAYOFF = 245,

    WIMAX = 246,
    RFKILL = 247,
}

impl From<u16> for KeyCode {
    fn from(n: u16) -> Self {
        num::FromPrimitive::from_u16(n).unwrap()
    }
}

impl std::fmt::Display for KeyCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl KeyCode {
    pub fn is_modifier(&self) -> bool {
        Modifiers::Unknown != self.into()
    }
}

// static char_to_keycode: HashMap<char, (KeyCode, bool)> = HashMap::from([
pub static CHAR_TO_KEYCODE: phf::Map<char, (KeyCode, bool)> = phf::phf_map! {
    '\t' => (KeyCode::TAB, false),
    '\n' => (KeyCode::ENTER, false),
    ' ' => (KeyCode::SPACE, false),
    '!' => (KeyCode::ALPHA1, true),
    '"' => (KeyCode::APOSTROPHE, true),
    '#' => (KeyCode::ALPHA3, true),
    '$' => (KeyCode::ALPHA4, true),
    '%' => (KeyCode::ALPHA5, true),
    '&' => (KeyCode::ALPHA7, true),
    '\'' => (KeyCode::APOSTROPHE, false),
    '(' => (KeyCode::ALPHA9, true),
    ')' => (KeyCode::ALPHA0, true),
    '*' => (KeyCode::ALPHA8, true),
    '+' => (KeyCode::EQUAL, true),
    ',' => (KeyCode::COMMA, false),
    '-' => (KeyCode::MINUS, false),
    '.' => (KeyCode::DOT, false),
    '/' => (KeyCode::SLASH, false),
    '0' => (KeyCode::ALPHA0, false),
    '1' => (KeyCode::ALPHA1, false),
    '2' => (KeyCode::ALPHA2, false),
    '3' => (KeyCode::ALPHA3, false),
    '4' => (KeyCode::ALPHA4, false),
    '5' => (KeyCode::ALPHA5, false),
    '6' => (KeyCode::ALPHA6, false),
    '7' => (KeyCode::ALPHA7, false),
    '8' => (KeyCode::ALPHA8, false),
    '9' => (KeyCode::ALPHA9, false),
    ':' => (KeyCode::SEMICOLON, true),
    ';' => (KeyCode::SEMICOLON, false),
    '<' => (KeyCode::COMMA, true),
    '=' => (KeyCode::EQUAL, false),
    '>' => (KeyCode::DOT, true),
    '?' => (KeyCode::SLASH, true),
    '@' => (KeyCode::ALPHA2, true),
    'A' => (KeyCode::A, true),
    'B' => (KeyCode::B, true),
    'C' => (KeyCode::C, true),
    'D' => (KeyCode::D, true),
    'E' => (KeyCode::E, true),
    'F' => (KeyCode::F, true),
    'G' => (KeyCode::G, true),
    'H' => (KeyCode::H, true),
    'I' => (KeyCode::I, true),
    'J' => (KeyCode::J, true),
    'K' => (KeyCode::K, true),
    'L' => (KeyCode::L, true),
    'M' => (KeyCode::M, true),
    'N' => (KeyCode::N, true),
    'O' => (KeyCode::O, true),
    'P' => (KeyCode::P, true),
    'Q' => (KeyCode::Q, true),
    'R' => (KeyCode::R, true),
    'S' => (KeyCode::S, true),
    'T' => (KeyCode::T, true),
    'U' => (KeyCode::U, true),
    'V' => (KeyCode::V, true),
    'W' => (KeyCode::W, true),
    'X' => (KeyCode::X, true),
    'Y' => (KeyCode::Y, true),
    'Z' => (KeyCode::Z, true),
    '[' => (KeyCode::LEFTBRACE, false),
    '\\' => (KeyCode::BACKSLASH, false),
    ']' => (KeyCode::RIGHTBRACE, false),
    '^' => (KeyCode::ALPHA6, true),
    '_' => (KeyCode::MINUS, true),
    '`' => (KeyCode::GRAVE, false),
    'a' => (KeyCode::A, false),
    'b' => (KeyCode::B, false),
    'c' => (KeyCode::C, false),
    'd' => (KeyCode::D, false),
    'e' => (KeyCode::E, false),
    'f' => (KeyCode::F, false),
    'g' => (KeyCode::G, false),
    'h' => (KeyCode::H, false),
    'i' => (KeyCode::I, false),
    'j' => (KeyCode::J, false),
    'k' => (KeyCode::K, false),
    'l' => (KeyCode::L, false),
    'm' => (KeyCode::M, false),
    'n' => (KeyCode::N, false),
    'o' => (KeyCode::O, false),
    'p' => (KeyCode::P, false),
    'q' => (KeyCode::Q, false),
    'r' => (KeyCode::R, false),
    's' => (KeyCode::S, false),
    't' => (KeyCode::T, false),
    'u' => (KeyCode::U, false),
    'v' => (KeyCode::V, false),
    'w' => (KeyCode::W, false),
    'x' => (KeyCode::X, false),
    'y' => (KeyCode::Y, false),
    'z' => (KeyCode::Z, false),
    '{' => (KeyCode::LEFTBRACE, true),
    '|' => (KeyCode::BACKSLASH, true),
    '}' => (KeyCode::RIGHTBRACE, true),
    '~' => (KeyCode::GRAVE, true),
};