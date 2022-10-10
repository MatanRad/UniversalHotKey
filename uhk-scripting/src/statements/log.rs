use crate::{
    execution::{ExecResult, IExecutable},
    statement::{IStatement, StatementCallInfo},
};

pub struct LogStatement {
    info: StatementCallInfo,
    text: String,
}

impl IStatement for LogStatement {
    fn call_info(&self) -> &StatementCallInfo {
        &self.info
    }

    fn name() -> String
    where
        Self: Sized,
    {
        "Log".to_string()
    }
}

impl IExecutable for LogStatement {
    fn exec(&self) -> ExecResult {
        println!("{}", self.text);
        ExecResult::SuccessNext
    }
}

impl LogStatement {
    pub fn new(info: StatementCallInfo, text: String) -> Self {
        LogStatement {
            info: info,
            text: text,
        }
    }
}
