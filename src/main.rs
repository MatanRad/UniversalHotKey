use std::collections::HashSet;

use anyhow::Result;
use config::UHKConfig;
use uhk_input::events::InputEvent;
use uhk_input::input::{IDispatcher, InputManager};
use uhk_input::modifiers::Modifiers;
use uhk_input::typer::InputTyper;
use uhk_scripting::parsing::parse;
use uhk_scripting::script::Script;

// mod test_script;
mod config;

fn load_scripts<'a>(conf: &UHKConfig, typer: &'a InputTyper) -> Vec<Script<'a>> {
    let mut scripts = vec![];

    for path in conf.scripts.iter() {
        let source = match std::fs::read_to_string("script.uhk") {
            Err(e) => {
                eprintln!(
                    "[SCRIPT LOADER] Couldn't find script ({}). Err: {}",
                    path, e
                );
                continue;
            }
            Ok(path) => path,
        };

        println!("[Script Loader] Parsing Script \"{}\"!", path);
        match parse(source.as_str()) {
            Err(e) => {
                println!(
                    "[Script Loader] Parsing Error: \"{}\" for script \"{}\"",
                    e, path
                );
                continue;
            }
            Ok(s) => {
                scripts.push(s.build(typer));
            }
        };
    }

    println!("[Script Loader] Done!");
    scripts
}

fn pretty_print_event(event: Option<InputEvent>, pressed: HashSet<Modifiers>) {
    let (desc, keycode) = match event {
        Some(InputEvent::KeyboardDownEvent(keycode)) => ("KeyDown", keycode),
        Some(InputEvent::KeyboardUpEvent(keycode)) => ("KeyUp", keycode),
        Some(InputEvent::KeyboardHeldEvent(keycode)) => ("KeyHeld", keycode),
        _ => {
            return;
        }
    };

    if keycode.is_modifier() {
        return;
    }

    let mut keys = String::new();

    for m in pressed.iter() {
        keys += format!("{:?} + ", m).as_str();
    }

    println!("{}: {}{}!", desc, keys, keycode);
}

fn inner_main() -> Result<()> {
    // parse the config
    let config = match UHKConfig::default() {
        Err(e) => return Err(anyhow::anyhow!("[Main] Error Loading Config: {}", e)),
        Ok(conf) => conf,
    };

    let mut manager = InputManager::new()?;
    let typer = InputTyper::new()?;

    let mut scripts = load_scripts(&config, &typer);

    if scripts.len() == 0 {
        return Err(anyhow::anyhow!(
            "No Scripts Loaded! Check stderr for errors! Exiting!"
        ));
    }

    loop {
        let event = manager.dispatch().unwrap();

        // TODO: Allow scripts to exit?
        for script in scripts.iter_mut() {
            let event_res = script.dispatch(&event, manager.modifiers());
            match event_res {
                Ok(opt) => opt,
                Err(_) => {
                    // Ret value is ignored, We don't care about any events that may have popped up.
                    // Let the script handle it :)
                    continue;
                }
            };
        }

        pretty_print_event(event, manager.modifiers().get_pressed());
    }
}

fn main() {
    match inner_main() {
        Err(e) => {
            eprintln!("[Main] Exiting - Error: \n\t{}", e);
            std::process::exit(1);
        }
        Ok(_) => std::process::exit(0),
    };
}
