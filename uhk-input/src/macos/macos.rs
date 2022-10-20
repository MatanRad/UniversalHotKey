mod macos_keycode;
mod macos_tap;
mod macos_typer;

use crate::events::InputEvent;
use crate::input::IDispatcher;
use anyhow::Result;

use self::macos_keycode::MacOSKeycode;
use self::macos_tap::MacOSTap;
use self::macos_typer::MacOSTyper;

impl IDispatcher for MacOSTap {
    fn dispatch(&mut self) -> Result<Option<InputEvent>> {
        let data = &mut self.data.lock().unwrap();
        if data.events.len() == 0 {
            return Ok(None);
        }

        return Ok(Some(data.events.remove(0).clone()));
    }

    fn set_listening(&mut self, listening: bool) {
        self.data.lock().unwrap().should_log = listening;
    }
}

pub type OsDispatcher = MacOSTap;
pub type OsTyper = MacOSTyper;
pub type OsKeycode = MacOSKeycode;
