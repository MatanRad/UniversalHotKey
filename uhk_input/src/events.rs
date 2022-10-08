use crate::keycode::KeyCode;

#[derive(Clone, Copy)]
pub enum InputEvent {
    KeyboardDownEvent(KeyCode),
    KeyboardUpEvent(KeyCode),
    KeyboardHeldEvent(KeyCode),
    MouseDownEvent,
    MouseUpEvent,
    MouseMovedEvent,
}
