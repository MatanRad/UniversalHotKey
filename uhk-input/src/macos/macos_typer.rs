use anyhow::Result;

use crate::typer::ITyper;

pub struct MacOSTyper {}

impl ITyper for MacOSTyper {
    fn key_down(&self, keycode: &crate::keycode::KeyCode) -> Result<()> {
        Ok(())
    }

    fn key_set(&self, keycode: &crate::keycode::KeyCode, down: bool) -> Result<()> {
        Ok(())
    }

    fn key_up(&self, keycode: &crate::keycode::KeyCode) -> Result<()> {
        Ok(())
    }
}

impl MacOSTyper {
    pub fn new() -> Result<Self> {
        Ok(Self {})
    }
}
