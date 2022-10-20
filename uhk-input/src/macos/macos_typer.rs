use anyhow::{anyhow, Result};
use core_graphics::event::{CGEvent, CGEventFlags};
use core_graphics::event_source::CGEventSource;

use crate::modifiers::{Modifiers, ModifiersState};
use crate::typer::ITyper;

pub struct MacOSTyper {
    mod_states: std::cell::RefCell<ModifiersState>,
}

const SLEEP_DURATION: std::time::Duration = std::time::Duration::from_millis(10);

impl ITyper for MacOSTyper {
    fn key_down(&self, keycode: &crate::keycode::KeyCode) -> Result<()> {
        self.key_set(keycode, true)
    }

    fn key_set(&self, keycode: &crate::keycode::KeyCode, down: bool) -> Result<()> {
        if down {
            self.mod_states.borrow_mut().key_down(keycode);
        } else {
            self.mod_states.borrow_mut().key_up(keycode);
        }

        let source = match CGEventSource::new(
            core_graphics::event_source::CGEventSourceStateID::HIDSystemState,
        ) {
            Ok(s) => s,
            Err(e) => return Err(anyhow!("[key_set] error creating event source: {:?}", e)),
        };

        let event = match CGEvent::new_keyboard_event(source, *keycode as u16, down) {
            Ok(s) => s,
            Err(e) => return Err(anyhow!("[key_set] error creating event: {:?}", e)),
        };

        let mut flags: CGEventFlags = CGEventFlags::empty();

        let state = &self.mod_states.borrow().state;
        if state[&Modifiers::LCtrl] || state[&Modifiers::RCtrl] {
            flags.insert(CGEventFlags::CGEventFlagControl);
        }

        if state[&Modifiers::LShift] || state[&Modifiers::RShift] {
            flags.insert(CGEventFlags::CGEventFlagShift);
        }

        if state[&Modifiers::LAlt] || state[&Modifiers::RAlt] {
            flags.insert(CGEventFlags::CGEventFlagShift);
        }

        if state[&Modifiers::Winkey] {
            flags.insert(CGEventFlags::CGEventFlagCommand);
        }

        event.set_flags(flags);

        event.post(core_graphics::event::CGEventTapLocation::HID);
        std::thread::sleep(SLEEP_DURATION);

        Ok(())
    }

    fn key_up(&self, keycode: &crate::keycode::KeyCode) -> Result<()> {
        self.key_set(keycode, false)
    }
}

impl MacOSTyper {
    pub fn new() -> Result<Self> {
        Ok(Self {
            mod_states: std::cell::RefCell::new(ModifiersState::new()),
        })
    }
}
