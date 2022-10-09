mod linux_event;
mod linux_typer;

use anyhow::Result;

use crate::events::InputEvent;
use crate::input::os::linux_event::KeyEventValue;
use crate::input::IDispatcher;
use byteorder::{LittleEndian, ReadBytesExt};
use linux_event::LinuxInputEvent;
use linux_typer::LinuxTyper;
use std::fs;
use std::io::Read; // 1.2.7

pub struct LinuxDispatcher {
    device: fs::File,
    kb_buf: Vec<u8>,
}

pub type OsDispatcher = LinuxDispatcher;
pub type OsTyper = LinuxTyper;

impl IDispatcher for LinuxDispatcher {
    fn dispatch(&mut self) -> Result<Option<InputEvent>> {
        self.dispatch_keyboard()
    }
}

impl LinuxDispatcher {
    pub fn new() -> Result<Self> {
        // TODO: Change this
        let path =
            "/dev/input/by-id/usb-Razer_Razer_BlackWidow_Chroma_V2-if01-event-kbd".to_string();
        Self::new_with_device_path(&path)
    }

    pub fn new_with_device_path(path: &String) -> Result<Self> {
        let file = fs::File::open(&path)?;
        Ok(Self {
            device: file,
            kb_buf: Vec::new(),
        })
    }

    fn dispatch_keyboard(&mut self) -> Result<Option<InputEvent>> {
        let mut event: LinuxInputEvent;
        loop {
            while self.kb_buf.len() < std::mem::size_of::<InputEvent>() {
                let mut buf: [u8; 128] = [0; 128];
                let num_read = self.device.read(&mut buf)?;

                for i in 0..num_read {
                    self.kb_buf.push(buf[i]);
                }
            }

            let event_type_num = (&self.kb_buf[16..18]).read_u16::<LittleEndian>()?;
            let event_type = linux_event::EventType::from_u16(event_type_num)?;

            let event_code = (&self.kb_buf[18..20]).read_u16::<LittleEndian>()?;
            let event_value = (&self.kb_buf[20..24]).read_i32::<LittleEndian>()?;

            for _ in 0..std::mem::size_of::<LinuxInputEvent>() {
                self.kb_buf.remove(0);
            }
            event = LinuxInputEvent {
                timeval: [0; 16],
                event_type: event_type,
                code: event_code,
                value: event_value,
            };

            if event.event_type == linux_event::EventType::KEY {
                break;
            }
        }

        let event_code = linux_event::KeyEventValue::from_i32(event.value)?;

        let final_event = match event_code {
            KeyEventValue::DOWN => InputEvent::KeyboardDownEvent(event.code.into()),
            KeyEventValue::UP => InputEvent::KeyboardUpEvent(event.code.into()),
            KeyEventValue::HELD => InputEvent::KeyboardHeldEvent(event.code.into()),
        };

        Ok(Some(final_event))
    }
}
