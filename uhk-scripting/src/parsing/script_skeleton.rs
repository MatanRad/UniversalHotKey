use std::collections::HashMap;

use uhk_input::typer::InputTyper;

use crate::{
    func::{CallingMethod, Function},
    script::Script,
};

pub struct ScriptSkeleton {
    pub(crate) funcs: HashMap<CallingMethod, Function>,
}

impl ScriptSkeleton {
    pub fn build<'a>(self, typer: &'a InputTyper) -> Script<'a> {
        Script::new(self.funcs, typer)
    }
}
