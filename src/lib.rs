pub mod core;
pub mod evaluator;
pub mod native;
pub mod parser;
pub mod tokenizer;

use crate::core::env::Env;
use crate::core::types::Types;

pub fn evaluate(env: &Env, program: &str) -> Types {
    let mut tokens = tokenizer::tokenize(program);
    let ast = parser::parse(&mut tokens);

    return ast.iter().fold(Types::Nil, |_state, t| {
        evaluator::evaluate(env.clone(), t.clone())
    });
}
