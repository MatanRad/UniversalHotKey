use crate::events::InputEvent;
use anyhow::Result;

#[cfg_attr(
    all(target_os = "linux", target_pointer_width = "64"),
    path = "linux/linux.rs"
)]
#[cfg_attr(target_os = "windows", path = "windows/windows.rs")]
#[cfg_attr(target_os = "macos", path = "macos/macos.rs")]
mod os;
use os::OsDispatcher;

pub trait IDispatcher {
    fn dispatch(&mut self) -> Result<Option<InputEvent>>;
}

pub struct InputManager {
    os_dispatcher: OsDispatcher,
}

impl IDispatcher for InputManager {
    fn dispatch(&mut self) -> Result<Option<InputEvent>> {
        self.os_dispatcher.dispatch()
    }
}

impl InputManager {
    pub fn new() -> Result<Self> {
        let dispatcher = OsDispatcher::new()?;
        return Ok(Self {
            os_dispatcher: dispatcher,
        });
    }
}
