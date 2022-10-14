mod macos_tap;

use crate::events::InputEvent;
use crate::input::IDispatcher;
use anyhow::Result;

use self::macos_tap::MacOSTap;

impl<'a> IDispatcher for MacOSTap<'a> {
    fn dispatch(&mut self) -> Result<Option<InputEvent>> {
        if self.events.len() == 0 {
            return Ok(None);
        }

        return Ok(Some(self.events[0].clone()));
    }
}

pub type OsDispatcher<'a> = MacOSTap<'a>;
pub type OsState<'a> = MacOSTap<'a>;
// pub type OsTyper = LinuxTyper;
