use anyhow::{anyhow, Result};
use core_graphics::event::{CGEvent, CGEventFlags};
use core_graphics::event_source::CGEventSource;

use crate::modifiers::{Modifiers, ModifiersState};
use crate::typer::ITyper;

pub struct MacOSTyper {
    mod_states: std::cell::RefCell<ModifiersState>,
}

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

        if keycode.is_modifier() {
            return Ok(());
        }

        let source = match CGEventSource::new(
            core_graphics::event_source::CGEventSourceStateID::CombinedSessionState,
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
        if state.contains_key(&Modifiers::LCtrl) || state.contains_key(&Modifiers::RCtrl) {
            flags.insert(CGEventFlags::CGEventFlagControl);
        }

        if state.contains_key(&Modifiers::LShift) || state.contains_key(&Modifiers::RShift) {
            flags.insert(CGEventFlags::CGEventFlagShift);
        }

        if state.contains_key(&Modifiers::LAlt) || state.contains_key(&Modifiers::RAlt) {
            flags.insert(CGEventFlags::CGEventFlagShift);
        }

        if state.contains_key(&Modifiers::Winkey) {
            flags.insert(CGEventFlags::CGEventFlagCommand);
        }

        event.set_flags(flags);

        event.post(core_graphics::event::CGEventTapLocation::Session);

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
