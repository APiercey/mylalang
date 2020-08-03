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
fn test_defined_functions() {
    let program = "
        (def add (fn [a b] (+ a b)))
        (add 1 2)";

    assert_eq(program, Types::Integer(3));
}

#[test]
fn test_anonymous_functions() {
    let program = "((fn [a b] (* a b)) 4 5)";

    assert_eq(program, Types::Integer(20));
}

#[test]
fn test_functions_with_multiexpression_bodies() {
    {
        let program = "
            (def multi_body (fn [i]
              (let [a i b 2]
                (+ a b)
                a)))

            (multi_body 1)";

        assert_eq(program, Types::Integer(1));
    }
    {
        let program = "((fn [i] (let [x 1] (+ x x) i)))";

        assert_eq(program, Types::Integer(1));
    }
}

#[test]
fn test_functions_using_let() {
    {
        let program = "
            (def shifter (fn [i]
              (let [x (* i i)
                    y (+ i i)
                    z (- x y)]
                z)))

            (shifter 6)";

        assert_eq(program, Types::Integer(24));
    }
    {
        let program = "((fn [i] (let [x (* i i) y (+ i i) z (- x y)] z)) 6)";

        assert_eq(program, Types::Integer(24));
    }
}

#[test]
fn test_functions_without_params() {
    {
        let program = "
            (def function_without_params (fn [] 1))

            (function_without_params)";

        assert_eq(program, Types::Integer(1));
    }
    {
        let program = "((fn [] 1))";

        assert_eq(program, Types::Integer(1));
    }
}
