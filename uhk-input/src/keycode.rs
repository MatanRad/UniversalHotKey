use crate::modifiers::Modifiers;
use crate::os::OsKeycode;

pub type KeyCode = OsKeycode;

impl From<u16> for KeyCode {
    fn from(n: u16) -> Self {
        num::FromPrimitive::from_u16(n).unwrap()
    }
}

impl From<i64> for KeyCode {
    fn from(n: i64) -> Self {
        num::FromPrimitive::from_i64(n).unwrap()
    }
}

impl From<char> for KeyCode {
    fn from(n: char) -> Self {
        let (code, _) = CHAR_TO_KEYCODE
            .get(&n)
            .unwrap_or(&(KeyCode::UNKNOWN, false));
        code.clone()
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
