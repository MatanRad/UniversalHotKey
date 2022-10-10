use anyhow::Result;
pub use ast::parse;
pub use ast::Rule;
use pest::iterators::Pair;

use crate::statement::StatementCallInfo;

mod ast;

pub trait IParseable {
    fn parse(info: StatementCallInfo, pair: Pair<Rule>) -> Result<Box<Self>>;
}
