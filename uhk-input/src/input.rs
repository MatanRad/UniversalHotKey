use crate::{events::InputEvent, modifiers::ModifiersState};
use anyhow::Result;

use crate::os::OsDispatcher;

pub trait IDispatcher {
    fn dispatch(&mut self) -> Result<Option<InputEvent>>;
    fn set_listening(&mut self, listening: bool);
}

pub struct InputManager {
    os_dispatcher: OsDispatcher,
    modifier_states: ModifiersState,
}

impl IDispatcher for InputManager {
    fn dispatch(&mut self) -> Result<Option<InputEvent>> {
        let ev = self.os_dispatcher.dispatch()?;

        if let Some(i) = ev {
            match i {
                InputEvent::KeyboardUpEvent(code) => self.modifier_states.key_up(&code),
                InputEvent::KeyboardDownEvent(code) => self.modifier_states.key_down(&code),
                _ => {}
            }
        }

        Ok(ev)
    }

    fn set_listening(&mut self, listening: bool) {
        self.os_dispatcher.set_listening(listening)
    }
}

impl InputManager {
    pub fn new() -> Result<Self> {
        let dispatcher = OsDispatcher::new()?;
        return Ok(Self {
            os_dispatcher: dispatcher,
            modifier_states: ModifiersState::new(),
        });
    }

    pub fn modifiers(&self) -> &ModifiersState {
        &self.modifier_states
    }
}
