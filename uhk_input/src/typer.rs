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

    fn reset_modifiers(&self) -> Result<()> {
        for m in Modifiers::iter() {
            self.typer.key_set(&m.to_keycode(), false)?;
        }

        Ok(())
    }

    pub fn type_key(
        &self,
        keycode: &crate::keycode::KeyCode,
        modifiers: Option<&HashSet<Modifiers>>,
        reset_modifiers: bool,
    ) -> Result<()> {
        if reset_modifiers {
            self.reset_modifiers()?;
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

        Ok(())
    }

    pub fn type_str(&self, text: &str, reset_modifiers: bool) -> Result<()> {
        if reset_modifiers {
            self.reset_modifiers()?;
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

        Ok(())
    }
}
