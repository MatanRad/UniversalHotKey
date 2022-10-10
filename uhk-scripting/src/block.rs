use crate::statement::IStatement;

pub struct Block {
    statements: Vec<Box<dyn IStatement>>,
    index: usize,
}

impl Block {
    pub fn statements(&self) -> &Vec<Box<dyn IStatement>> {
        &self.statements
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn new(index: usize, statements: Vec<Box<dyn IStatement>>) -> Self {
        Self {
            index: index,
            statements: statements,
        }
    }
}
