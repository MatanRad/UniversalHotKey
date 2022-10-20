use crate::keycode::KeyCode;
use std::slice::Iter;
use std::{collections::HashMap, collections::HashSet, ops::Index};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
pub enum Modifiers {
    LShift,
    RShift,
    LCtrl,
    RCtrl,
    LAlt,
    RAlt,
    Winkey,
    Command,
    Option,
    Unknown,
}

#[derive(Clone)]
pub struct ModifiersState {
    pub state: HashMap<Modifiers, bool>,
}

impl Modifiers {
    pub fn to_keycode(&self) -> KeyCode {
        match *self {
            Self::LShift => KeyCode::LEFTSHIFT,
            Self::RShift => KeyCode::RIGHTSHIFT,
            Self::LCtrl => KeyCode::LEFTCTRL,
            Self::RCtrl => KeyCode::RIGHTCTRL,
            Self::LAlt => KeyCode::LEFTALT,
            Self::RAlt => KeyCode::RIGHTALT,
            Self::Winkey => KeyCode::LEFTMETA,
            _ => KeyCode::UNKNOWN,
        }
    }

    pub fn iter() -> Iter<'static, Modifiers> {
        static ALL: [Modifiers; 7] = [
            Modifiers::LShift,
            Modifiers::RShift,
            Modifiers::LCtrl,
            Modifiers::RCtrl,
            Modifiers::LAlt,
            Modifiers::RAlt,
            Modifiers::Winkey,
        ];
        ALL.iter()
    }

    pub fn switch_side(&self) -> Modifiers {
        match *self {
            Self::LShift => Self::RShift,
            Self::LCtrl => Self::RCtrl,
            Self::LAlt => Self::RAlt,
            Self::RShift => Self::LShift,
            Self::RCtrl => Self::LCtrl,
            Self::RAlt => Self::LAlt,
            Self::Winkey => Self::Winkey,
            Self::Option => Self::Option,
            Self::Command => Self::Command,
            Self::Unknown => Self::Unknown,
        }
    }
}

impl From<KeyCode> for Modifiers {
    fn from(code: KeyCode) -> Self {
        Modifiers::from(&code)
    }
}

impl From<&KeyCode> for Modifiers {
    fn from(code: &KeyCode) -> Self {
        match code {
            KeyCode::LEFTSHIFT => Modifiers::LShift,
            KeyCode::RIGHTSHIFT => Modifiers::RShift,
            KeyCode::LEFTALT => Modifiers::LAlt,
            KeyCode::RIGHTALT => Modifiers::RAlt,
            KeyCode::LEFTCTRL => Modifiers::LCtrl,
            KeyCode::RIGHTCTRL => Modifiers::RCtrl,
            KeyCode::LEFTMETA => Modifiers::Winkey,
            _ => Modifiers::Unknown,
        }
    }
}

impl ModifiersState {
    pub fn key_up(&mut self, code: &KeyCode) {
        let key: Modifiers = code.into();

        if key == Modifiers::Unknown {
            return;
        }

        self.state
            .entry(key)
            .and_modify(|e| *e = false)
            .or_insert(false);
    }

    pub fn key_down(&mut self, code: &KeyCode) {
        let key: Modifiers = code.into();

        if key == Modifiers::Unknown {
            return;
        }

        self.state
            .entry(key)
            .and_modify(|e| *e = true)
            .or_insert(true);
    }

    pub fn get_pressed(&self) -> HashSet<Modifiers> {
        let mut pressed = HashSet::new();

        for (k, v) in self.state.iter() {
            if *v {
                pressed.insert(k.clone());
            }
        }

        pressed
    }
}

impl Index<&Modifiers> for ModifiersState {
    type Output = bool;
    fn index(&self, index: &Modifiers) -> &Self::Output {
        &self.state[index]
    }
}

impl ModifiersState {
    pub fn new() -> Self {
        ModifiersState {
            state: HashMap::from([
                (Modifiers::LShift, false),
                (Modifiers::RShift, false),
                (Modifiers::LAlt, false),
                (Modifiers::RAlt, false),
                (Modifiers::LCtrl, false),
                (Modifiers::RCtrl, false),
                (Modifiers::Winkey, false),
            ]),
        }
    }
}
