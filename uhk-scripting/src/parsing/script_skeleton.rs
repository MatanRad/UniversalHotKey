use std::collections::HashMap;

use uhk_input::{input::InputManager, typer::InputTyper};

use crate::{
    func::{CallingMethod, Function},
    script::Script,
};

pub struct ScriptSkeleton {
    pub(crate) funcs: HashMap<CallingMethod, Function>,
}

impl ScriptSkeleton {
    pub fn build<'a>(self, manager: &'a InputManager, typer: &'a InputTyper) -> Script<'a> {
        Script::new(self.funcs, manager, typer)
    }
}
