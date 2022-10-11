use std::collections::HashSet;
use std::{collections::HashMap, vec};

use crate::block::Block;
use crate::func::{CallingMethod, Function, IFunction};
use crate::script::Script;
use crate::statement::{IStatement, StatementCallInfo};
use crate::statements;

use anyhow::Result;
use pest::iterators::{Pair, Pairs};
use pest::Parser;
use uhk_input::keycode::KeyCode;
use uhk_input::modifiers::Modifiers;
use uhk_input::utils::HashableHashSet;

use super::IParseable;

#[derive(Parser)]
#[grammar = "parsing/program.pest"] // relative to src
pub struct UHKParser;

pub fn parse_statements(
    pairs: Pairs<Rule>,
    level: usize,
    block_index: usize,
) -> Result<Vec<Box<dyn IStatement>>> {
    let mut statements = vec![];

    for p in pairs {
        let info = StatementCallInfo::new(level, block_index);
        let statement: Box<dyn IStatement> = match p.as_rule() {
            Rule::log_statement => statements::LogStatement::parse(info, p)?,
            Rule::return_statement => statements::ReturnStatement::parse(info, p)?,
            Rule::call_statement => statements::CallStatement::parse(info, p)?,
            Rule::send_statement => statements::SendStatement::parse(info, p)?,
            Rule::send_raw_statement => statements::SendStatement::parse(info, p)?,
            _ => {
                return Err(anyhow::anyhow!(
                    "Invalid statement encountered '{:?}'!",
                    p.as_rule()
                ));
            }
        };

        statements.push(statement);
    }

    Ok(statements)
}

pub fn parse_func_hotkeys(pair: Pair<Rule>) -> Result<HashableHashSet<KeyCode>> {
    let mut keys = HashSet::new();
    if !matches!(pair.as_rule(), Rule::func_name)
        && !matches!(pair.as_rule(), Rule::chars_singleline)
    {
        return Err(anyhow::anyhow!(
            "[PARSE HOTKEYS] Tried building keycode list out of non-key rule {:?}",
            pair.as_rule()
        ));
    }

    let name = pair.as_str();

    for c in name.chars() {
        let keycode = KeyCode::from(c);

        if !keys.insert(keycode) {
            return Err(anyhow::anyhow!("Key '{}' typed twice!", c));
        }
    }

    Ok(HashableHashSet::new(keys))
}

pub fn parse_func_modifiers(pair: Pair<Rule>) -> Result<HashableHashSet<Modifiers>> {
    let mut mods = HashSet::new();
    if !matches!(pair.as_rule(), Rule::func_modifiers)
        && !matches!(pair.as_rule(), Rule::func_modifiers_required)
    {
        return Err(anyhow::anyhow!(
            "[PARSE MODIFIERS] Tried building mods out of non-mods rule {:?} ({})",
            pair.as_rule(),
            pair
        ));
    }

    let inner = pair.into_inner();

    for p in inner {
        let modifier = match p.as_rule() {
            Rule::mod_winkey => Modifiers::Winkey,
            Rule::mod_alt => Modifiers::LAlt,
            Rule::mod_ctrl => Modifiers::LCtrl,
            Rule::mod_shift => Modifiers::LShift,
            Rule::mod_concat => return Err(anyhow::anyhow!("Mod mod_concat not supported yet!")),
            Rule::mod_altgr => return Err(anyhow::anyhow!("Mod mod_altgr not supported yet!")),
            Rule::mod_wildcard => {
                return Err(anyhow::anyhow!("Mod mod_wildcard not supported yet!"))
            }
            Rule::mod_block => return Err(anyhow::anyhow!("Mod mod_block not supported yet!")),
            Rule::mod_force_hook => {
                return Err(anyhow::anyhow!("Mod mod_force_hook not supported yet!"))
            }
            Rule::mod_up => return Err(anyhow::anyhow!("Mod mod_up not supported yet!")),
            _ => return Err(anyhow::anyhow!("Expected mod but found {:?}!", p.as_rule())),
        };

        if !mods.insert(modifier) {
            return Err(anyhow::anyhow!("Mod {:?} typed twice!", p.as_rule()));
        }
    }

    Ok(HashableHashSet::new(mods))
}

pub fn parse_hotkey(pair: Pair<Rule>) -> Result<CallingMethod> {
    if !matches!(pair.as_rule(), Rule::hotkey) {
        return Err(anyhow::anyhow!(
            "[PARSE HOTKEY] Tried building func out of non-hotkey rule {:?}",
            pair.as_rule()
        ));
    }

    let mut subpairs = pair.into_inner();
    let mods_pair = match subpairs.next() {
        Some(p) => p,
        None => return Err(anyhow::anyhow!("[PARSE FUNC] Didn't find func_modifiers!")),
    };

    if !matches!(mods_pair.as_rule(), Rule::func_modifiers) {
        return Err(anyhow::anyhow!(
            "[PARSE HOTKEY] Expected hotkey modifiers but found: {:?}!",
            mods_pair.as_rule()
        ));
    }

    let mods = parse_func_modifiers(mods_pair)?;

    let keys_pair = match subpairs.next() {
        Some(p) => p,
        None => return Err(anyhow::anyhow!("[PARSE HOTKEY] Didn't find func_name!")),
    };

    // TODO: leave it as func_name? change it to hotkeys?
    if !matches!(keys_pair.as_rule(), Rule::func_name) {
        return Err(anyhow::anyhow!(
            "[PARSE HOTKEY] Expected hotkey 'func_name' but found: {:?}!",
            keys_pair.as_rule()
        ));
    }

    let keys = parse_func_hotkeys(keys_pair)?;

    Ok(CallingMethod::Hotkey(keys, mods))
}

pub fn parse_func(pair: Pair<Rule>) -> Result<CallingMethod> {
    if !matches!(pair.as_rule(), Rule::func) {
        return Err(anyhow::anyhow!(
            "[PARSE FUNC] Tried building func out of non-func rule {}",
            pair
        ));
    }

    let mut subpairs = pair.into_inner();
    let name_pair = match subpairs.next() {
        Some(p) => p,
        None => return Err(anyhow::anyhow!("[PARSE FUNC] Didn't find func_name!")),
    };

    if !matches!(name_pair.as_rule(), Rule::func_name) {
        return Err(anyhow::anyhow!(
            "[PARSE FUNC] Expected func name but found: {:?}!",
            name_pair.as_rule()
        ));
    }

    let name = name_pair.as_str().to_string();
    // println!("[DEBUG PARSE FUNC] Found function: {}", name);
    Ok(CallingMethod::Manual(name))
}

pub fn build_generic_func(pair: Pair<Rule>) -> Result<Function> {
    // println!("[DEBUG] BUILDING FUNC");
    // TODO: only one block for now. No If's, for's or but's.
    let calling_method = match pair.as_rule() {
        Rule::func => parse_func(pair.clone())?,
        Rule::hotkey => parse_hotkey(pair.clone())?,
        _ => {
            return Err(anyhow::anyhow!(
                "[PARSE GENERIC] Tried building func out of non-func rule {}",
                pair
            ));
        }
    };

    let mut func_pairs = pair.into_inner();
    let some_statements = func_pairs.find(|p| matches!(p.as_rule(), Rule::some_statements));

    if some_statements.is_none() {
        // TODO: which func
        return Err(anyhow::anyhow!(
            "[PARSE GENERIC] Didn't find statements in func!"
        ));
    }

    let statements_pair = some_statements.unwrap().into_inner();

    // TODO: Only one block supported for now. The index is 0
    let block = Block::new(0, parse_statements(statements_pair, 0, 0)?);
    Ok(Function::new(calling_method, vec![block]))
}

pub fn parse(source: &str) -> Result<Script> {
    let mut funcs = HashMap::new();

    let mut root_pairs = UHKParser::parse(Rule::program, source)?;
    let program_pair = match root_pairs.next() {
        None => {
            return Err(anyhow::anyhow!("[PARSE SCRIPT] Empty Rule Tree!"));
        }
        Some(p) => p,
    };

    if !matches!(program_pair.as_rule(), Rule::program) {
        return Err(anyhow::anyhow!(
            "[PARSE SCRIPT] Main rule should be 'program', found '{:?}'!",
            program_pair.as_rule()
        ));
    }

    for pair in program_pair.into_inner() {
        let func = match pair.as_rule() {
            Rule::func => build_generic_func(pair)?,
            Rule::hotkey => build_generic_func(pair)?,
            Rule::EOI => continue,
            _ => {
                println!("[DEBUG] unknown rule: {:?}", pair.as_rule());
                continue;
            }
        };

        if funcs.contains_key(func.calling_method()) {
            // TODO: Which func?
            return Err(anyhow::anyhow!(
                "[PARSE GENERIC] Func with calling method already exists!"
            ));
        }
        funcs.insert(func.calling_method().clone(), func);
    }

    Ok(Script::new(funcs)?)
}
