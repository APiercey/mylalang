use crate::env;
use crate::types::{define_function, Types, VArgs};

fn inspect(t: Types) -> Types {
    t.inspect();

    return t;
}

fn add(first: Types, second: Types) -> Types {
    match (first, second) {
        (Types::Integer(a), Types::Integer(b)) => Types::Integer(a + b),
        (Types::Float(a), Types::Float(b)) => Types::Float(a + b),
        (Types::Float(a), Types::Integer(b)) => Types::Float(a + (b as f64)),
        (Types::Integer(a), Types::Float(b)) => Types::Float((a as f64) + b),
        (Types::String(a), Types::String(b)) => Types::String(format!("{}{}", a, b)),
        (a, b) => panic!("cannot add {:?} from {:?}", a, b),
    }
}

fn subtract(first: Types, second: Types) -> Types {
    match (first, second) {
        (Types::Integer(a), Types::Integer(b)) => Types::Integer(a - b),
        (Types::Float(a), Types::Integer(b)) => Types::Float(a - (b as f64)),
        (Types::Integer(a), Types::Float(b)) => Types::Float((a as f64) - b),
        (Types::Float(a), Types::Float(b)) => Types::Float(a - b),
        (a, b) => panic!("cannot subtract {:?} from {:?}", a, b),
    }
}

fn divide(first: Types, second: Types) -> Types {
    match (first, second) {
        (Types::Integer(a), Types::Integer(b)) if a == 0 || b == 0 => panic!("Division by zero"),
        (Types::Float(a), Types::Float(b)) => Types::Float(a / b),
        (Types::Integer(a), Types::Integer(b)) => Types::Float((a as f64) / (b as f64)),
        (Types::Integer(a), Types::Float(b)) => Types::Float((a as f64) / b),
        (Types::Float(a), Types::Integer(b)) => Types::Float(a / (b as f64)),
        (a, b) => panic!("cannot divide {:?} from {:?}", a, b),
    }
}

fn multiply(first: Types, second: Types) -> Types {
    match (first, second) {
        (Types::Integer(a), Types::Integer(b)) => Types::Integer(a * b),
        (Types::Integer(a), Types::Float(b)) => Types::Float((a as f64) * b),
        (Types::Float(a), Types::Integer(b)) => Types::Float(a * (b as f64)),
        (Types::Float(a), Types::Float(b)) => Types::Float(a * b),
        (a, b) => panic!("cannot multiply {:?} from {:?}", a, b),
    }
}

pub fn set_core_functions(env: &env::Env) {
    env::set_env(
        &env,
        "add",
        define_function(|args: VArgs| add(args[0].clone(), args[1].clone())),
    );

    env::set_env(
        &env,
        "subtract",
        define_function(|args: VArgs| subtract(args[0].clone(), args[1].clone())),
    );

    env::set_env(
        &env,
        "divide",
        define_function(|args: VArgs| divide(args[0].clone(), args[1].clone())),
    );

    env::set_env(
        &env,
        "multiply",
        define_function(|args: VArgs| multiply(args[0].clone(), args[1].clone())),
    );

    env::set_env(
        &env,
        "inspect",
        define_function(|args: VArgs| inspect(args[0].clone())),
    );
}
