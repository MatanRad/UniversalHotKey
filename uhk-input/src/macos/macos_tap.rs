use anyhow::Result;
use core_foundation::mach_port::CFMachPort;

use crate::events::InputEvent;
use crate::keycode::KeyCode;

use core_graphics::event::{
    CGEvent, CGEventMask, CGEventTap, CGEventTapLocation, CGEventTapOptions, CGEventTapPlacement,
    CGEventType,
};

const CG_FIELD_KEYBOARD_EVENT_KEYCODE: u32 = 9;

pub struct MacOSTap<'a> {
    tap: CGEventTap<'a>,
    pub events: Vec<InputEvent>,
}

impl TryInto<InputEvent> for CGEvent {
    type Error = anyhow::Error;
    fn try_into(self) -> Result<InputEvent, Self::Error> {
        let kc = self.get_integer_value_field(CG_FIELD_KEYBOARD_EVENT_KEYCODE);
        match self.get_type() {
            CGEventType::KeyDown => {
                // TODO: KC
                Ok(InputEvent::KeyboardDownEvent(KeyCode::UNKNOWN))
            }
            CGEventType::KeyUp => {
                // TODO: KC
                Ok(InputEvent::KeyboardUpEvent(KeyCode::UNKNOWN))
            }
            _ => Err(anyhow::anyhow!("Cannot convert event to InputEvent!")),
        }
    }
}

impl<'a> MacOSTap<'a> {
    pub fn new() -> Result<Self> {
        let events = vec![];

        let tap = match CGEventTap::new(
            CGEventTapLocation::Session,
            CGEventTapPlacement::HeadInsertEventTap,
            CGEventTapOptions::ListenOnly, // TODO We can be active ;)
            vec![CGEventType::KeyDown, CGEventType::KeyUp.into()],
            |a, b, c| {
                match c.clone().try_into() {
                    Ok(i) => events.push(i),
                    _ => {}
                }

                return Some(c.clone());
            },
        ) {
            Ok(t) => t,
            Err(_) => return Err(anyhow::anyhow!("Couldn't Create Event Tap!")),
        };

        Ok(Self {
            tap: tap,
            events: events,
        })
    }
}
