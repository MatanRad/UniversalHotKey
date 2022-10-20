use crate::block::Block;
use crate::execution::{ExecResult, IExecutable};
use crate::script::Script;
use core::panic;
use uhk_input::input::InputManager;
use uhk_input::keycode::KeyCode;
use uhk_input::modifiers::Modifiers;
use uhk_input::utils::HashableHashSet;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum CallingMethod {
    Manual(String),
    Hotkey(HashableHashSet<KeyCode>, HashableHashSet<Modifiers>),
}

pub trait IFunction
where
    Self: IExecutable,
{
    fn calling_method(&self) -> &CallingMethod;
    fn blocks(&self) -> &Vec<Block>;
}

pub struct Function {
    calling_method: CallingMethod,
    blocks: Vec<Block>,
}

impl IFunction for Function {
    fn calling_method(&self) -> &CallingMethod {
        &self.calling_method
    }

    fn blocks(&self) -> &Vec<Block> {
        &self.blocks
    }
}

impl IExecutable for Function {
    fn exec(&self, script: &Script, manager: &mut InputManager) -> ExecResult {
        let mut curr_block: usize = 0;
        loop {
            let block = &self.blocks()[curr_block];
            for s in block.statements().iter() {
                let res = s.exec(script, manager);

                match res {
                    ExecResult::SuccessNext => {}
                    ExecResult::SuccessReturn => {
                        return ExecResult::SuccessReturn;
                    }
                    ExecResult::FailNext(reason) => {
                        // TODO: Support names here.
                        println!(
                            "Failed executing statement: ({}) reason: ({})",
                            s.call_info().context_level(),
                            reason
                        )
                    }
                    ExecResult::FailCrash(reason) => {
                        // TODO: Support names here.
                        panic!(
                            "Failed executing statement ({})!\nCrash Reason: '{}'",
                            s.call_info().context_level(),
                            reason
                        );
                    }
                    ExecResult::FailProgram(reason) => {
                        // TODO: Support names here.
                        return ExecResult::FailProgram(format!(
                            "Failed executing statement ({})!\nFail Reason: '{}'",
                            s.call_info().context_level(),
                            reason
                        ));
                    }
                    ExecResult::SuccessBreak => {
                        // TODO: is this needed? can be accomplished with jump
                        panic!("SuccessBreak NOT IMPLEMENTED");
                    }
                    ExecResult::SuccessJump(_) => {
                        panic!("SuccessJump NOT IMPLEMENTED");
                        // This would probably need to do continue btw.
                    }
                }
            }

            curr_block += 1;

            if curr_block >= self.blocks().len() {
                // TODO: which block?
                // TODO: is this what we want? maybe fallthrough to "next" function?
                return ExecResult::FailProgram("Block didn't return!".to_string());
            }
        }
    }
}

impl Function {
    pub fn new(calling_method: CallingMethod, blocks: Vec<Block>) -> Self {
        Self {
            calling_method: calling_method,
            blocks: blocks,
        }
    }
}
