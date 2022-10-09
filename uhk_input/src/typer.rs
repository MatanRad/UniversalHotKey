use crate::input::InputManager;
use crate::keycode::{KeyCode, CHAR_TO_KEYCODE};
use crate::modifiers::Modifiers;
use crate::os::OsTyper;
use anyhow::Result;
use std::collections::HashSet;

pub trait ITyper {
    fn key_set(&self, keycode: &KeyCode, down: bool) -> Result<()>;
    fn key_down(&self, keycode: &KeyCode) -> Result<()>;
    fn key_up(&self, keycode: &KeyCode) -> Result<()>;
}

pub struct InputTyper {
    typer: OsTyper,
}

impl ITyper for InputTyper {
    fn key_set(&self, keycode: &KeyCode, down: bool) -> Result<()> {
        self.typer.key_set(keycode, down)
    }
    fn key_down(&self, keycode: &KeyCode) -> Result<()> {
        self.typer.key_down(keycode)
    }
    fn key_up(&self, keycode: &KeyCode) -> Result<()> {
        self.typer.key_up(keycode)
    }
}

impl InputTyper {
    pub fn new() -> Result<Self> {
        Ok(Self {
            typer: OsTyper::new()?,
        })
    }

    fn set_modifiers(
        &self,
        modifiers: &HashSet<Modifiers>,
        down: bool,
        keep: Option<&HashSet<Modifiers>>,
    ) -> Result<()> {
        for m in modifiers.iter() {
            if let Some(keep) = keep {
                if keep.contains(m) {
                    continue;
                }
            }

            self.typer.key_set(&m.to_keycode(), down)?;
        }

        Ok(())
    }

    pub fn type_key(
        &self,
        keycode: &crate::keycode::KeyCode,
        modifiers: Option<&HashSet<Modifiers>>,
        current_modifiers: Option<&HashSet<Modifiers>>,
    ) -> Result<()> {
        if let Some(curr_modifiers) = current_modifiers {
            self.set_modifiers(curr_modifiers, false, modifiers)?;
        }

        if let Some(modifiers) = modifiers {
            for m in modifiers.iter() {
                self.key_down(&m.to_keycode())?;
            }
        }

        self.key_down(keycode)?;
        self.key_up(keycode)?;

        if let Some(modifiers) = modifiers {
            for m in modifiers.iter() {
                self.key_up(&m.to_keycode())?;
            }
        }

        if let Some(curr_modifiers) = current_modifiers {
            self.set_modifiers(curr_modifiers, true, modifiers)?;
        }

        // TODO: add modifiers

        Ok(())
    }

    pub fn type_str(&self, text: &str, reset_modifiers: Option<&HashSet<Modifiers>>) -> Result<()> {
        if let Some(curr_modifiers) = current_modifiers {
            self.set_modifiers(curr_modifiers, false, None)?;
        }

        for i in text.chars() {
            if !CHAR_TO_KEYCODE.contains_key(&i) {
                return Err(anyhow::anyhow!("Invalid char in typed string: '{}'!", i));
            }
        }
        for i in text.chars() {
            let (keycode, shift) = CHAR_TO_KEYCODE.get(&i).unwrap();
            if *shift {
                self.key_down(&KeyCode::LEFTSHIFT)?;
            }

            self.key_down(keycode)?;
            self.key_up(keycode)?;

            if *shift {
                self.key_up(&KeyCode::LEFTSHIFT)?;
            }
        }

        // if let Some(curr_modifiers) = current_modifiers {
        //     self.set_modifiers(curr_modifiers, true, None)?;
        // }

        Ok(())
    }
}
