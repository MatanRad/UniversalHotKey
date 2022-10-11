use crate::script::Script;

#[derive(PartialEq)]
pub enum ExecResult {
    SuccessNext,
    SuccessReturn,
    SuccessBreak,
    SuccessJump(usize),
    FailNext(String),
    FailCrash(String), // TODO: Maybe instead of strings we want errors here? Might be good
    FailProgram(String),
}

impl ExecResult {
    pub fn to_result(&self) -> Result<&ExecResult, anyhow::Error> {
        return match self {
            ExecResult::SuccessNext => Ok(self),
            ExecResult::SuccessReturn => Ok(self),
            ExecResult::SuccessBreak => Ok(self),
            ExecResult::SuccessJump(_) => Ok(self),
            ExecResult::FailNext(reason) => Err(anyhow::anyhow!(
                "[SCRIPT] Soft Runtime Error: '{}'!",
                reason
            )),
            ExecResult::FailCrash(reason) => Err(anyhow::anyhow!(
                "[SCRIPT] Should have crashed! Reason: '{}'",
                reason
            )),
            ExecResult::FailProgram(reason) => {
                Err(anyhow::anyhow!("[SCRIPT] Hard Failure: '{}'", reason))
            }
        };
    }
}

pub trait IExecutable {
    fn exec(&self, script: &Script) -> ExecResult;
}
