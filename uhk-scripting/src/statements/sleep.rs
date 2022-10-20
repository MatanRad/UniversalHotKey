use crate::{parsing::Rule, script::Script};
use pest::iterators::Pair;
use uhk_input::input::InputManager;

use crate::{
    execution::{ExecResult, IExecutable},
    parsing::IParseable,
    statement::{IStatement, StatementCallInfo},
};

pub struct SleepStatement {
    info: StatementCallInfo,
    millis: u64,
}

impl IStatement for SleepStatement {
    fn call_info(&self) -> &StatementCallInfo {
        &self.info
    }

    fn name() -> String
    where
        Self: Sized,
    {
        "Sleep".to_string()
    }
}

impl IExecutable for SleepStatement {
    fn exec(&self, _script: &Script, _: &mut InputManager) -> ExecResult {
        std::thread::sleep(std::time::Duration::from_millis(self.millis));
        ExecResult::SuccessNext
    }
}

impl IParseable for SleepStatement {
    fn parse(info: StatementCallInfo, pair: Pair<Rule>) -> anyhow::Result<Box<Self>> {
        if !matches!(pair.as_rule(), Rule::sleep_statement) {
            return Err(anyhow::anyhow!(
                "[sleep_statement] Expected rule 'sleep_statement' but found '{:?}'",
                pair.as_rule()
            ));
        }

        let int_pair = match pair.into_inner().next() {
            None => return Err(anyhow::anyhow!("[sleep_statement] couldn't find int!")),
            Some(p) => p,
        };

        if !matches!(int_pair.as_rule(), Rule::int) {
            return Err(anyhow::anyhow!(
                "[sleep_statement] Expected rule 'int' but found '{:?}'",
                int_pair.as_rule()
            ));
        }

        match int_pair.as_str().parse::<u64>() {
            Err(e) => {
                return Err(anyhow::anyhow!("[SLEEP ERROR] {}", e));
            }
            Ok(millis) => Ok(Box::new(SleepStatement {
                info: info,
                millis: millis,
            })),
        }
    }
}

impl SleepStatement {
    pub fn new(info: StatementCallInfo, millis: u64) -> Self {
        Self {
            info: info,
            millis: millis,
        }
    }
}
