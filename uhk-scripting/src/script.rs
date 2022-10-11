use uhk_input::events::InputEvent;
use uhk_input::input::IDispatcher;
use uhk_input::input::InputManager;
use uhk_input::modifiers::Modifiers;
use uhk_input::typer::InputTyper;

use crate::execution::ExecResult;
use crate::execution::IExecutable;
use crate::func::CallingMethod;
use crate::func::Function;
use std::collections::HashMap;
use std::collections::HashSet;

pub(crate) trait IScript {
    fn functions(&self) -> &HashMap<CallingMethod, Function>;
    fn call_func(&self, call_method: &CallingMethod) -> ExecResult;
    fn manager(&self) -> &InputManager;
    fn typer(&self) -> &InputTyper;
}

pub struct Script {
    funcs: HashMap<CallingMethod, Function>,
    manager: InputManager,
    typer: InputTyper,
}

impl IScript for Script {
    fn functions(&self) -> &HashMap<CallingMethod, Function> {
        &self.funcs
    }

    fn call_func(&self, call_method: &CallingMethod) -> ExecResult {
        for (method, func) in self.funcs.iter() {
            if method == call_method {
                return func.exec(self);
            }
        }

        // TODO: which function?
        ExecResult::FailProgram("Function not found!".to_string())
    }

    fn typer(&self) -> &InputTyper {
        &self.typer
    }

    fn manager(&self) -> &InputManager {
        &self.manager
    }
}

impl IDispatcher for Script {
    fn dispatch(&mut self) -> anyhow::Result<Option<InputEvent>> {
        let event = match self.manager.dispatch()? {
            None => {
                return Ok(None);
            }
            Some(event) => event,
        };

        let keycode_up = match event {
            InputEvent::KeyboardUpEvent(keycode) => keycode,
            _ => {
                return Ok(Some(event));
            }
        };
        if keycode_up.is_modifier() {
            return Ok(Some(event));
        }

        for (call_method, func) in self.funcs.iter() {
            let (keycodes, modifiers) = match call_method {
                CallingMethod::Manual(_) => continue,
                CallingMethod::Hotkey(keys, mods) => (keys, mods),
            };

            // TODO: Support more than one non-modifier key at a time
            // This is disabled in parsing, better have this here anyway...
            if keycodes.hashset().len() > 1 {
                return Err(anyhow::anyhow!("[SCRIPT DISPATCH] How did you get here? Hotkeys with more than 1 non-modifier keys aren't supported. (keys: {:?}", keycodes.hashset()));
            }

            let pressed_mods = self.manager.modifiers().get_pressed();
            if keycodes.hashset().contains(&keycode_up) && pressed_mods == *modifiers.hashset() {
                // Running the hotkey func!
                // TODO: ignoring the result. Should be fine. Think about it.
                let _ = func.exec(&self);
                return Ok(Some(event));
            }
        }

        Ok(Some(event))
    }
}

impl Script {
    pub fn new(funcs: HashMap<CallingMethod, Function>) -> anyhow::Result<Self> {
        let manager = InputManager::new()?;
        let typer = InputTyper::new()?;
        Ok(Self {
            funcs: funcs,
            manager: manager,
            typer: typer,
        })
    }

    pub fn get_pressed_modifiers(&self) -> HashSet<Modifiers> {
        self.manager().modifiers().get_pressed()
    }
}
