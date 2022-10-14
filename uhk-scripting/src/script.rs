use uhk_input::events::InputEvent;
use uhk_input::input::InputManager;
use uhk_input::typer::InputTyper;

use crate::execution::ExecResult;
use crate::execution::IExecutable;
use crate::func::CallingMethod;
use crate::func::Function;
use std::collections::HashMap;

pub(crate) trait IScript {
    fn functions(&self) -> &HashMap<CallingMethod, Function>;
    fn call_func(&self, call_method: &CallingMethod) -> ExecResult;
    fn manager(&self) -> &InputManager;
    fn typer(&self) -> &InputTyper;
}

pub struct Script<'a> {
    funcs: HashMap<CallingMethod, Function>,
    typer: &'a InputTyper,
}

impl<'a> IScript for Script<'a> {
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

impl<'a> Script<'a> {
    pub fn new(
        funcs: HashMap<CallingMethod, Function>,
        manager: &'a InputManager,
        typer: &'a InputTyper,
    ) -> Self {
        Self {
            funcs: funcs,
            manager: manager,
            typer: typer,
        }
    }

    pub fn dispatch(&mut self, event: &Option<InputEvent>) -> anyhow::Result<()> {
        let event = match event {
            None => {
                return Ok(());
            }
            Some(e) => e,
        };

        let keycode_up = match event {
            InputEvent::KeyboardUpEvent(keycode) => keycode,
            _ => {
                return Ok(());
            }
        };
        if keycode_up.is_modifier() {
            return Ok(());
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
                return Ok(());
            }
        }

        Ok(())
    }
}
