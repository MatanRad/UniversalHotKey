use crate::{parsing::Rule, script::Script};
use pest::iterators::Pair;

use crate::{
    execution::{ExecResult, IExecutable},
    parsing::IParseable,
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
    fn exec(&self, _script: &Script) -> ExecResult {
        println!("{}", self.text);
        ExecResult::SuccessNext
    }
}

impl IParseable for LogStatement {
    fn parse(info: StatementCallInfo, pair: Pair<Rule>) -> anyhow::Result<Box<Self>> {
        if !matches!(pair.as_rule(), Rule::log_statement) {
            return Err(anyhow::anyhow!(
                "[log_statement] Expected rule 'log_statement' but found '{:?}'",
                pair.as_rule()
            ));
        }

        let string_pair = match pair.into_inner().next() {
            None => {
                return Err(anyhow::anyhow!(
                    "[log_statement] couldn't find str_content!"
                ))
            }
            Some(p) => p,
        };

        if !matches!(string_pair.as_rule(), Rule::str_content) {
            return Err(anyhow::anyhow!(
                "[log_statement] Expected rule 'str_content' but found '{:?}'",
                string_pair.as_rule()
            ));
        }

        Ok(Box::new(LogStatement {
            info: info,
            text: string_pair.as_str().to_string(),
        }))
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
