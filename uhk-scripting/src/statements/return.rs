use crate::{parsing::Rule, script::Script};
use pest::iterators::Pair;

use crate::{
    execution::{ExecResult, IExecutable},
    parsing::IParseable,
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
    fn exec(&self, _script: &Script) -> ExecResult {
        ExecResult::SuccessReturn
    }
}

impl IParseable for ReturnStatement {
    fn parse(info: StatementCallInfo, pair: Pair<Rule>) -> anyhow::Result<Box<Self>> {
        if !matches!(pair.as_rule(), Rule::return_statement) {
            return Err(anyhow::anyhow!(
                "Expected rule 'return_statement' but found '{:?}'",
                pair.as_rule()
            ));
        }

        Ok(Box::new(ReturnStatement { info }))
    }
}

impl ReturnStatement {
    pub fn new(info: StatementCallInfo) -> Self {
        ReturnStatement { info: info }
    }
}
