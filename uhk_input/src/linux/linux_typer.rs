use std::ptr::{null, null_mut};

use crate::keycode::KeyCode;
use crate::typer::ITyper;
use anyhow::{Ok, Result};

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

    fn internal_key_set(&self, keycode: &KeyCode, down: bool) {
        unsafe {
            let state = if down { 1 } else { 0 };
            x11::xtest::XTestFakeKeyEvent(self.disp, *keycode as u32 + 8, state, 0);
            sleep(SLEEP_DURATION);
            xlib::XFlush(self.disp);
        }
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
    fn key_set(&self, keycode: &KeyCode, down: bool) -> Result<()> {
        self.curr_window()?;
        self.internal_key_set(keycode, down);
        Ok(())
    }

    fn key_down(&self, keycode: &KeyCode) -> Result<()> {
        self.key_set(keycode, true)
    }

    fn key_up(&self, keycode: &KeyCode) -> Result<()> {
        self.key_set(keycode, false)
    }
}
