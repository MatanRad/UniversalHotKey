use crate::{
    execution::{ExecResult, IExecutable},
    statement::{IStatement, StatementCallInfo},
};

pub struct ReturnStatement {
    info: StatementCallInfo,
}

impl IStatement for ReturnStatement {
    fn call_info(&self) -> &StatementCallInfo {
        &self.info
    }

    fn name() -> String
    where
        Self: Sized,
    {
        "Return".to_string()
    }
}

impl IExecutable for ReturnStatement {
    fn exec(&self) -> ExecResult {
        ExecResult::SuccessReturn
    }
}

impl ReturnStatement {
    pub fn new(info: StatementCallInfo) -> Self {
        ReturnStatement { info: info }
    }
}
