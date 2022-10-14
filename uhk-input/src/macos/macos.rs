mod macos_keycode;
mod macos_tap;
mod macos_typer;

use crate::events::InputEvent;
use crate::input::IDispatcher;
use anyhow::Result;

use self::macos_keycode::MacOSKeycode;
use self::macos_tap::MacOSTap;
use self::macos_typer::MacOSTyper;

impl<'a> IDispatcher for MacOSTap<'a> {
    fn dispatch(&mut self) -> Result<Option<InputEvent>> {
        let mut events = self.events.lock().unwrap();
        if events.len() == 0 {
            return Ok(None);
        }

        return Ok(Some(events.remove(0).clone()));
    }
}

pub type OsDispatcher<'a> = MacOSTap<'a>;
pub type OsTyper = MacOSTyper;
pub type OsKeycode = MacOSKeycode;
