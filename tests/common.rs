use myla::core;
use myla::core::env;
use myla::core::types::Types;
use myla::core::types::Types::{Float, Integer, Nil};
use myla::evaluator;
use myla::parser;
use myla::tokenizer;

pub fn evaluate(program: &str) -> Types {
    let env = env::new_env(None);

    core::setup_core_environment(&env);

    let mut tokens = tokenizer::tokenize(program);

    let ast = parser::parse(&mut tokens);

    return ast
        .iter()
        .fold(Nil, |_state, t| evaluator::evaluate(env.clone(), t.clone()));
}

pub fn assert_eq(assertion: &str, expected: Types) {
    match (evaluate(assertion), expected) {
        (Integer(a), Integer(b)) => assert_eq!(a, b),
        (Float(a), Float(b)) => assert_eq!(a, b),
        _ => panic!("Incompatible types"),
    }
}
