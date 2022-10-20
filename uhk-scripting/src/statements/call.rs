use crate::{
    func::CallingMethod,
    parsing::Rule,
    script::{IScript, Script},
};
use pest::iterators::Pair;
use uhk_input::input::InputManager;

use crate::{
    execution::{ExecResult, IExecutable},
    parsing::IParseable,
    statement::{IStatement, StatementCallInfo},
};

pub struct CallStatement {
    info: StatementCallInfo,
    calling_method: CallingMethod,
}

impl IStatement for CallStatement {
    fn call_info(&self) -> &StatementCallInfo {
        &self.info
    }

    fn name() -> String
    where
        Self: Sized,
    {
        // TODO: This is not how it's done in AHK, need to support expressions first.
        "Call".to_string()
    }
}

impl IExecutable for CallStatement {
    fn exec(&self, script: &Script, manager: &mut InputManager) -> ExecResult {
        let res = script.call_func(&self.calling_method, manager);

        if res == ExecResult::SuccessReturn {
            return ExecResult::SuccessNext;
        }

        res
    }
}

impl IParseable for CallStatement {
    fn parse(info: StatementCallInfo, pair: Pair<Rule>) -> anyhow::Result<Box<Self>> {
        if !matches!(pair.as_rule(), Rule::call_statement) {
            return Err(anyhow::anyhow!(
                "[call_statement] Expected rule 'call_statement' but found '{:?}'",
                pair.as_rule()
            ));
        }

        let name_pair = match pair.into_inner().next() {
            None => return Err(anyhow::anyhow!("[call_statement] couldn't find func_name!")),
            Some(p) => p,
        };

        if !matches!(name_pair.as_rule(), Rule::func_name) {
            return Err(anyhow::anyhow!(
                "[log_statement] Expected rule 'func_name' but found '{:?}'",
                name_pair.as_rule()
            ));
        }

        Ok(Box::new(CallStatement {
            info: info,
            calling_method: CallingMethod::Manual(name_pair.as_str().to_string()),
        }))
    }
}

impl CallStatement {
    pub fn new(info: StatementCallInfo, func_name: String) -> Self {
        CallStatement {
            info: info,
            calling_method: CallingMethod::Manual(func_name),
        }
    }
}
