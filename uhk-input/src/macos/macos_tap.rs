use anyhow::Result;
use core_foundation::runloop::CFRunLoopMode;

use crate::events::InputEvent;
use crate::keycode::KeyCode;
use std::{
    marker::PhantomData,
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
};

use core_graphics::event::{
    CGEvent, CGEventTap, CGEventTapLocation, CGEventTapOptions, CGEventTapPlacement, CGEventType,
};

const CG_FIELD_KEYBOARD_EVENT_KEYCODE: u32 = 9;

pub struct MacOSTap<'a> {
    pub events: Arc<Mutex<Vec<InputEvent>>>,
    pub handle: JoinHandle<()>,
    _phony: PhantomData<&'a ()>,
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

impl<'a> MacOSTap<'a> {
    pub fn new() -> Result<Self> {
        let data = Arc::new(Mutex::new(vec![]));
        let cloned = data.clone();
        let handle = thread::spawn(move || {
            let tap = CGEventTap::new(
                CGEventTapLocation::Session,
                CGEventTapPlacement::HeadInsertEventTap,
                CGEventTapOptions::ListenOnly, // TODO We can be active ;)
                vec![CGEventType::KeyDown, CGEventType::KeyUp],
                |a, b, c| {
                    match c.clone().try_into() {
                        Ok(i) => {
                            let mut sdata = cloned.lock().unwrap();
                            (*sdata).push(i);
                        }
                        _ => {}
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
            _phony: PhantomData,
        })
    }
}
