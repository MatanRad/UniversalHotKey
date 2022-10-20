use anyhow::Result;

use crate::events::InputEvent;
use crate::keycode::KeyCode;
use std::{
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
};

use core_graphics::event::{
    CGEvent, CGEventFlags, CGEventTap, CGEventTapLocation, CGEventTapOptions, CGEventTapPlacement,
    CGEventType,
};

const CG_FIELD_KEYBOARD_EVENT_KEYCODE: u32 = 9;

pub struct MacOSTap {
    pub events: Arc<Mutex<Vec<InputEvent>>>,
    pub handle: JoinHandle<()>,
}

fn event_by_flag_bit(flags: &CGEventFlags, bit: CGEventFlags, keycode: KeyCode) -> InputEvent {
    if flags.contains(bit) {
        return InputEvent::KeyboardDownEvent(KeyCode::from(keycode));
    }
    return InputEvent::KeyboardUpEvent(KeyCode::from(keycode));
}

fn cgevent_to_events(ev: &CGEvent) -> Result<Vec<InputEvent>> {
    let kc = ev.get_integer_value_field(CG_FIELD_KEYBOARD_EVENT_KEYCODE);
    let mut events = vec![];

    let main_ev = match ev.get_type() {
        CGEventType::KeyDown => InputEvent::KeyboardDownEvent(KeyCode::from(kc)),
        CGEventType::KeyUp => InputEvent::KeyboardUpEvent(KeyCode::from(kc)),
        _ => return Err(anyhow::anyhow!("Cannot convert event to InputEvent!")),
    };

    let flags = ev.get_flags();
    events.push(event_by_flag_bit(
        &flags,
        CGEventFlags::CGEventFlagCommand,
        KeyCode::LEFTMETA,
    ));
    events.push(event_by_flag_bit(
        &flags,
        CGEventFlags::CGEventFlagControl,
        KeyCode::LEFTCTRL,
    ));
    events.push(event_by_flag_bit(
        &flags,
        CGEventFlags::CGEventFlagAlternate,
        KeyCode::LEFTALT,
    ));
    events.push(event_by_flag_bit(
        &flags,
        CGEventFlags::CGEventFlagShift,
        KeyCode::LEFTSHIFT,
    ));

    events.push(main_ev);

    Ok(events)
}

impl TryInto<InputEvent> for CGEvent {
    type Error = anyhow::Error;
    fn try_into(self) -> Result<InputEvent, Self::Error> {
        let kc = self.get_integer_value_field(CG_FIELD_KEYBOARD_EVENT_KEYCODE);
        match self.get_type() {
            CGEventType::KeyDown => {
                // TODO: KC
                Ok(InputEvent::KeyboardDownEvent(KeyCode::from(kc)))
            }
            CGEventType::KeyUp => {
                // TODO: KC
                Ok(InputEvent::KeyboardUpEvent(KeyCode::from(kc)))
            }
            _ => Err(anyhow::anyhow!("Cannot convert event to InputEvent!")),
        }
    }
}

impl MacOSTap {
    pub fn new() -> Result<Self> {
        let data = Arc::new(Mutex::new(vec![]));
        let cloned = data.clone();
        let handle = thread::spawn(move || {
            let tap = CGEventTap::new(
                CGEventTapLocation::Session,
                CGEventTapPlacement::HeadInsertEventTap,
                CGEventTapOptions::ListenOnly, // TODO We can be active ;)
                vec![CGEventType::KeyDown, CGEventType::KeyUp],
                |_, _, c| {
                    let found_events = cgevent_to_events(c);

                    if found_events.is_ok() {
                        let mut sdata = cloned.lock().unwrap();
                        (*sdata).append(&mut (found_events.unwrap()));
                    }

                    return Some(c.clone());
                },
            )
            .unwrap();

            unsafe {
                let loop_source: core_foundation::runloop::CFRunLoopSource = tap
                    .mach_port
                    .create_runloop_source(0)
                    .expect("Failed creating loop source");
                let current = core_foundation::runloop::CFRunLoop::get_current();
                current.add_source(
                    &loop_source,
                    core_foundation::runloop::kCFRunLoopCommonModes,
                );
                tap.enable();
                core_foundation::runloop::CFRunLoop::run_current();
            }
        });

        Ok(Self {
            handle: handle,
            events: data,
        })
    }
}
