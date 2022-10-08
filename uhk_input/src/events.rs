use crate::keycode::KeyCode;

pub enum InputEvent {
    KeyboardDownEvent(KeyCode),
    KeyboardUpEvent(KeyCode),
    KeyboardHeldEvent(KeyCode),
    MouseDownEvent,
    MouseUpEvent,
    MouseMovedEvent
}