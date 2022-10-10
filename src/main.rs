use std::collections::HashSet;
use uhk_input::events::InputEvent;
use uhk_input::input::{IDispatcher, InputManager};
use uhk_input::keycode::KeyCode;
use uhk_input::modifiers::Modifiers;
use uhk_input::typer::InputTyper;
use uhk_scripting::func::CallingMethod;
use uhk_scripting::parsing::parse;

// mod test_script;

fn main() {
    let mut manager = InputManager::new().unwrap();
    let typer = InputTyper::new().unwrap();
    // let script = test_script::get_script();

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

        if keycode == KeyCode::R && pressed == HashSet::from([Modifiers::Winkey, Modifiers::LCtrl])
        {
            typer
                .type_key(&KeyCode::T, Some(&HashSet::from([Modifiers::Winkey])), true)
                .unwrap();

            std::thread::sleep(std::time::Duration::from_millis(1500));
            typer
                .type_str(
                    "open \"https://www.youtube.com/watch?v=dQw4w9WgXcQ\"\n",
                    true,
                )
                .unwrap();
        }

        if keycode == KeyCode::M && pressed == HashSet::from([Modifiers::Winkey, Modifiers::LCtrl])
        {
            typer.type_str("matan@radomski.co.il", true).unwrap();
        }

        if keycode == KeyCode::S && pressed == HashSet::from([Modifiers::Winkey, Modifiers::LCtrl])
        {
            let source = std::fs::read_to_string("script.uhk").unwrap();

            println!("PARSING!");
            let script = match parse(source.as_str()) {
                Err(e) => {
                    println!("PARSING ERROR: {}", e);
                    continue;
                }
                Ok(s) => s,
            };

            println!("Executing!");
            match script.exec_func(&CallingMethod::Manual("main".to_string())) {
                Err(e) => {
                    println!("RUNTIME ERROR: {}", e);
                    continue;
                }
                Ok(_) => {
                    println!("DONE");
                }
            }
        }
    }
}
