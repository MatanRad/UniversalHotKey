use std::ptr::{null, null_mut};

use crate::input::ITyper;
use crate::keycode::{KeyCode, CHAR_TO_KEYCODE};
use crate::modifiers::Modifiers;
use anyhow::Result;

use std::thread::sleep;

use x11::xlib;
use x11::xlib::Display;

const SLEEP_DURATION: std::time::Duration = std::time::Duration::from_millis(20);

pub struct LinuxTyper {
    disp: *mut Display,
}

impl LinuxTyper {
    pub fn new() -> Result<Self> {
        unsafe {
            let disp = xlib::XOpenDisplay(null());
            if disp == null_mut() {
                return Err(anyhow::anyhow!("Failed to open display!"));
            }

            Ok(LinuxTyper { disp: disp })
        }
    }

    fn key_set(&self, keycode: &KeyCode, down: bool) {
        unsafe {
            let state = if down { 1 } else { 0 };
            x11::xtest::XTestFakeKeyEvent(self.disp, *keycode as u32 + 8, state, 0);
            sleep(SLEEP_DURATION);
            xlib::XFlush(self.disp);
        }
    }

    fn key_down(&self, keycode: &KeyCode) {
        self.key_set(keycode, true);
    }

    fn key_up(&self, keycode: &KeyCode) {
        self.key_set(keycode, false);
    }

    fn curr_window(&self) -> Result<xlib::Window> {
        let mut window: xlib::Window = 0xffffffffffffffff;
        let mut revert: i32 = 0;
        unsafe {
            xlib::XGetInputFocus(self.disp, &mut window, &mut revert);
        }

        if window == 0xffffffffffffffff {
            return Err(anyhow::anyhow!("Couldn't get current window!"));
        }

        Ok(window)
    }
}

impl Drop for LinuxTyper {
    fn drop(&mut self) {
        unsafe {
            if self.disp != null_mut() {
                xlib::XCloseDisplay(self.disp);
                self.disp = null_mut();
            }
        }
    }
}

impl ITyper for LinuxTyper {
    fn type_single(
        &mut self,
        keycode: &crate::keycode::KeyCode,
        modifiers: &Vec<Modifiers>,
    ) -> Result<()> {
        self.curr_window()?;

        for m in modifiers {
            self.key_down(&m.to_keycode());
        }

        self.key_down(keycode);
        self.key_up(keycode);

        for m in modifiers {
            self.key_up(&m.to_keycode());
        }

        Ok(())
    }

    fn type_str(&mut self, text: &str) -> Result<()> {
        for i in text.chars() {
            if !CHAR_TO_KEYCODE.contains_key(&i) {
                return Err(anyhow::anyhow!("Invalid char in typed string: '{}'!", i));
            }
        }

        self.curr_window()?;

        for i in text.chars() {
            let (keycode, shift) = CHAR_TO_KEYCODE.get(&i).unwrap();
            if *shift {
                self.key_down(&KeyCode::LEFTSHIFT);
            }

            self.key_down(keycode);
            self.key_up(keycode);

            if *shift {
                self.key_up(&KeyCode::LEFTSHIFT);
            }
        }

        Ok(())
    }
}
