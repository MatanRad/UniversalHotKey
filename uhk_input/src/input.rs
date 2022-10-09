use crate::keycode::KeyCode;
use crate::modifiers::Modifiers;
use crate::{events::InputEvent, modifiers::ModifiersState};
use anyhow::Result;

#[cfg_attr(
    all(target_os = "linux", target_pointer_width = "64"),
    path = "linux/linux.rs"
)]
#[cfg_attr(target_os = "windows", path = "windows/windows.rs")]
#[cfg_attr(target_os = "macos", path = "macos/macos.rs")]
mod os;
use os::OsDispatcher;
use os::OsTyper;

pub trait IDispatcher {
    fn dispatch(&mut self) -> Result<Option<InputEvent>>;
}

pub trait ITyper {
    fn type_str(&mut self, text: &str) -> Result<()>;
    fn type_single(&mut self, keycode: &KeyCode, modifiers: &Vec<Modifiers>) -> Result<()>;
}

pub struct InputManager {
    os_dispatcher: OsDispatcher,
    pub os_typer: OsTyper,
    modifier_states: ModifiersState,
}

impl IDispatcher for InputManager {
    fn dispatch(&mut self) -> Result<Option<InputEvent>> {
        let ev = self.os_dispatcher.dispatch()?;

        if let Some(i) = ev {
            match i {
                InputEvent::KeyboardUpEvent(code) => self.modifier_states.key_up(code),
                InputEvent::KeyboardDownEvent(code) => self.modifier_states.key_down(code),
                _ => {}
            }
        }

        Ok(ev)
    }
}

impl InputManager {
    pub fn new() -> Result<Self> {
        let dispatcher = OsDispatcher::new()?;
        return Ok(Self {
            os_dispatcher: dispatcher,
            os_typer: OsTyper::new()?,
            modifier_states: ModifiersState::new(),
        });
    }

    pub fn modifiers(&self) -> &ModifiersState {
        &self.modifier_states
    }
}
