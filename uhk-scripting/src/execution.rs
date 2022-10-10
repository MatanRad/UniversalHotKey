use crate::script::Script;

#[derive(PartialEq)]
pub enum ExecResult {
    SuccessNext,
    SuccessReturn,
    SuccessBreak,
    SuccessJump(usize),
    FailNext,
    FailCrash(String),
    FailProgram(String),
}

pub trait IExecutable {
    fn exec(&self, script: &Script) -> ExecResult;
}
