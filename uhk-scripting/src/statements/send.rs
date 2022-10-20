use crate::{
    parsing::{
        ast::{parse_func_hotkeys, parse_func_modifiers},
        Rule,
    },
    script::{IScript, Script},
};
use pest::iterators::Pair;
use uhk_input::{
    input::{IDispatcher, InputManager},
    keycode::KeyCode,
    modifiers::Modifiers,
    utils::HashableHashSet,
};

use crate::{
    execution::{ExecResult, IExecutable},
    parsing::IParseable,
    statement::{IStatement, StatementCallInfo},
};

pub enum SendMethod {
    Hotkey(HashableHashSet<Modifiers>, HashableHashSet<KeyCode>),
    Text(String),
}

pub struct SendStatement {
    info: StatementCallInfo,
    method: SendMethod,
    is_raw: bool,
}

impl IStatement for SendStatement {
    fn call_info(&self) -> &StatementCallInfo {
        &self.info
    }

    fn name() -> String
    where
        Self: Sized,
    {
        // TODO: This is not how it's done in AHK, need to support expressions first.
        "Send".to_string()
    }
}

impl IExecutable for SendStatement {
    fn exec(&self, script: &Script, manager: &mut InputManager) -> ExecResult {
        manager.set_listening(false);

        let typer = script.typer();
        let res = match &self.method {
            SendMethod::Hotkey(modifiers, keys) => {
                typer.type_keys(keys.hashset(), Some(modifiers.hashset()), true)
            }
            SendMethod::Text(text) => {
                if self.is_raw {
                    typer.type_str(text.as_str(), true)
                } else {
                    let text = text.replace("{ENTER}", "\n").replace("{TAB}", "\t");
                    typer.type_str(text.as_str(), true)
                }
            }
        };

        manager.set_listening(true);

        match res {
            Err(e) => ExecResult::FailProgram(format!("[SEND FAIL] {}", e)),
            Ok(_) => ExecResult::SuccessNext,
        }
    }
}

impl IParseable for SendStatement {
    fn parse(info: StatementCallInfo, pair: Pair<Rule>) -> anyhow::Result<Box<Self>> {
        let is_raw;
        match pair.as_rule() {
            Rule::send_statement => {
                is_raw = false;
            }
            Rule::send_raw_statement => {
                is_raw = true;
            }
            _ => {
                return Err(anyhow::anyhow!(
                    "[send_statement] Expected rule 'send_statement' but found '{:?}'",
                    pair.as_rule()
                ));
            }
        }

        let mut pair = pair.into_inner();
        let mods_pair = match pair.next() {
            None => {
                return Err(anyhow::anyhow!(
                    "[send_statement] couldn't find enough arugments!"
                ))
            }
            Some(p) => p,
        };

        match mods_pair.as_rule() {
            Rule::chars_singleline => {
                return Ok(Box::new(Self {
                    info: info,
                    method: SendMethod::Text(mods_pair.as_str().to_string()),
                    is_raw: is_raw,
                }));
            }
            Rule::func_modifiers_required => {
                let mods = parse_func_modifiers(mods_pair.clone())?;
                let keycodes = match pair.next() {
                    None => return Err(anyhow::anyhow!("[send_statement] Missing keys!")),
                    Some(p) => parse_func_hotkeys(p)?,
                };

                return Ok(Box::new(Self {
                    info: info,
                    method: SendMethod::Hotkey(mods, keycodes),
                    is_raw: is_raw,
                }));
            }
            _ => {
                return Err(anyhow::anyhow!(
                    "[send_statement] Expected to find text or modifiers+hotkeys!"
                ))
            }
        }
    }
}

impl SendStatement {
    pub fn new(info: StatementCallInfo, method: SendMethod, is_raw: bool) -> Self {
        Self {
            info: info,
            method: method,
            is_raw: is_raw,
        }
    }
}
