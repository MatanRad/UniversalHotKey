use anyhow::Result;
pub use ast::parse;
pub use ast::Rule;
use pest::iterators::Pair;

use crate::statement::StatementCallInfo;

pub(crate) mod ast;
pub(crate) mod script_skeleton;

pub trait IParseable {
    fn parse(info: StatementCallInfo, pair: Pair<Rule>) -> Result<Box<Self>>;
}
