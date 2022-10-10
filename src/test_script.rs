use std::collections::HashMap;

use uhk_scripting::{
    block::Block,
    func::{CallingMethod, Function, IFunction},
    script::Script,
    statement::StatementCallInfo,
    statements::{log::LogStatement, r#return::ReturnStatement},
};

fn build_block() -> Block {
    let statement1 = LogStatement::new(StatementCallInfo::new(0, 0), "I have ran!".to_string());
    let statement2 = LogStatement::new(StatementCallInfo::new(0, 0), "I have ran2!".to_string());
    let return_statement = ReturnStatement::new(StatementCallInfo::new(0, 0));

    Block::new(
        0,
        vec![
            Box::new(statement1),
            Box::new(statement2),
            Box::new(return_statement),
        ],
    )
}

fn build_func() -> Function {
    let block = build_block();

    let method = CallingMethod::Manual("test_func".to_string());

    Function::new(method, vec![block])
}

pub fn get_script() -> Script {
    let test_func = build_func();
    let funcs = HashMap::from([((*test_func.calling_method()).clone(), test_func)]);

    Script::new(funcs)
}
