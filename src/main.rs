use std::collections::HashSet;
use uhk_input::events::InputEvent;
use uhk_input::input::{IDispatcher, InputManager};
use uhk_input::keycode::KeyCode;
use uhk_input::modifiers::Modifiers;
use uhk_input::typer::InputTyper;

const MEME: bool = false;

fn main() {
    let mut manager = InputManager::new().unwrap();
    let mut typer = InputTyper::new().unwrap();

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

        if desc != "KeyUp" {
            continue;
        }

        if keycode == KeyCode::H && MEME {
            typer
                .type_key(
                    &KeyCode::T,
                    Some(&HashSet::from([Modifiers::Winkey])),
                    Some(&pressed),
                )
                .unwrap();

            std::thread::sleep(std::time::Duration::from_millis(1500));
            typer
                .type_str(
                    "open \"https://www.youtube.com/watch?v=dQw4w9WgXcQ\"\n",
                    Some(&pressed),
                )
                .unwrap();
        }

        if keycode == KeyCode::M && pressed == HashSet::from([Modifiers::Winkey, Modifiers::LCtrl])
        {
            typer
                .type_str("matan@radomski.co.il", Some(&pressed))
                .unwrap();
        }
    }
}
