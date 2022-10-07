use crate::events::InputEvent;

pub struct InputManager;

impl InputManager {
    pub fn dispatch(self) -> InputEvent {
        InputEvent::KeyboardDownEvent(0)
    }
}