use crate::execution::IExecutable;

pub struct StatementCallInfo {
    context_level: usize,
    block_index: usize,
}

impl StatementCallInfo {
    pub fn context_level(&self) -> usize {
        self.context_level
    }

    pub fn block_index(&self) -> usize {
        self.block_index
    }

    pub fn new(level: usize, block_index: usize) -> Self {
        StatementCallInfo {
            context_level: level,
            block_index: block_index,
        }
    }
}

pub trait IStatement
where
    Self: IExecutable,
{
    fn call_info(&self) -> &StatementCallInfo;
    fn name() -> String
    where
        Self: Sized;
}
