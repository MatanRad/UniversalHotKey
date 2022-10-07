use crate::keycode::KeyCode;

pub enum InputEvent {
    KeyboardDownEvent(KeyCode),
    KeyboardUpEvent(KeyCode),
    MouseDownEvent,
    MouseUpEvent,
    MouseMovedEvent
}