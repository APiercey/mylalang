use myla::core;
use myla::core::env;
use myla::core::types::Types;
use myla::core::types::Types::{Float, Integer};
use myla::evaluator;
use myla::parser;
use myla::tokenizer;

fn evaluate(program: &str) -> Types {
    let env = env::new_env(None);

    core::setup_core_environment(&env);

    let mut tokens = tokenizer::tokenize(program);

    let ast = parser::parse(&mut tokens);

    return ast.iter().fold(Types::Nil, |_state, t| {
        evaluator::evaluate(env.clone(), t.clone())
    });
}

fn assert_eq(assertion: &str, expected: Types) {
    match (evaluate(assertion), expected) {
        (Integer(a), Integer(b)) => assert_eq!(a, b),
        (Float(a), Float(b)) => assert_eq!(a, b),
        _ => panic!("Incompatible types"),
    }
}

#[test]
fn test_add_integers() {
    assert_eq("(+ 1 2)", Types::Integer(3));
    assert_eq("(+ -5 5)", Types::Integer(0));
    assert_eq("(+ 4 -45)", Types::Integer(-41));
    assert_eq("(+ -10 -3)", Types::Integer(-13));
}

#[test]
fn test_add_floats() {
    assert_eq("(+ 1.0 1.0)", Types::Float(2.0));
    assert_eq("(+ -5.25 3.25)", Types::Float(-2.0));
    assert_eq("(+ -10.1 -3.1)", Types::Float(-13.2));
    // Remember. Say no to floats, kids!
    assert_eq("(+ 4.50 -5.35)", Types::Float(-0.8499999999999996));
}
