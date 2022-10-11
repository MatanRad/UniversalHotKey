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

    // TODO: JESUS better naming
    pub fn exec_func(&self, call_method: &CallingMethod) -> anyhow::Result<ExecResult> {
        let res = self.call_func(call_method);

        return match res {
            ExecResult::SuccessNext => Ok(res),
            ExecResult::SuccessReturn => Ok(res),
            ExecResult::SuccessBreak => Ok(res),
            ExecResult::SuccessJump(_) => Ok(res),
            ExecResult::FailNext => Err(anyhow::anyhow!("Quiet Failure!")),
            ExecResult::FailCrash(reason) => {
                Err(anyhow::anyhow!("Would have crashed! Reason: '{}'", reason))
            }
            ExecResult::FailProgram(reason) => Err(anyhow::anyhow!("Loud Failure: '{}'", reason)),
        };
    }
}
