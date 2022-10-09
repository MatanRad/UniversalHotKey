use uhk_input::events::InputEvent;
use uhk_input::input::{IDispatcher, ITyper, InputManager};
use uhk_input::keycode::KeyCode;
use uhk_input::modifiers::Modifiers;

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

        if keycode == KeyCode::H && desc == "KeyUp" {
            manager
                .os_typer
                .type_single(&KeyCode::T, &vec![Modifiers::Winkey])
                .unwrap();

            std::thread::sleep(std::time::Duration::from_millis(1500));
            manager
                .os_typer
                .type_str("open \"https://www.youtube.com/watch?v=dQw4w9WgXcQ\"\n")
                .unwrap();
        }
    }
}
