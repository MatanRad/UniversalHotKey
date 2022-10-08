use uhk_input::events::InputEvent;
use uhk_input::input::{IDispatcher, InputManager};

fn main() {
    let mut manager = InputManager::new().unwrap();

    loop {
        let event = manager.dispatch().unwrap();

        let (desc, keycode) = match event {
            Some(InputEvent::KeyboardDownEvent(keycode)) => ("KeyDown", keycode),
            Some(InputEvent::KeyboardUpEvent(keycode)) => ("KeyUp", keycode),
            Some(InputEvent::KeyboardHeldEvent(keycode)) => ("KeyHeld", keycode),
            _ => {
                continue;
            }
        };

        if keycode.is_modifier() {
            continue;
        }

        let pressed = manager.modifiers().get_pressed();
        let mut keys = String::new();

        for m in pressed.iter() {
            keys += format!("{:?} + ", m).as_str();
        }

        println!("{}: {}{}!", desc, keys, keycode);
    }
}
