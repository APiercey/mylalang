use myla::core;
use myla::core::types::Types;
use myla::core::types::Types::{Bool, Float, Integer, Nil, String as Str};
use myla::native;

fn evaluate(assertion: &str) -> Types {
    let env = core::env::new_env(None);

    core::load(&env);
    native::load(&env);

    return myla::evaluate(&env, assertion);
}

pub fn assert_eq(assertion: &str, expected: Types) {
    match (evaluate(assertion), expected) {
        (Integer(a), Integer(b)) => assert_eq!(a, b),
        (Float(a), Float(b)) => assert_eq!(a, b),
        (Str(a), Str(b)) => assert_eq!(a, b),
        (Nil, Nil) => assert!(true),
        (Bool(a), Bool(b)) => assert_eq!(a, b),
        // (Vector(a), Vector(b)) => assert_eq!(a, b),
        _ => panic!("Incompatible types"),
    }
}
