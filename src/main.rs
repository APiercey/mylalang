mod env;
mod interpreter;
mod parser;
mod tokenizer;
mod types;
use crate::types::{define_function, Types, VArgs};
use std::env as envargs;
use std::fs;

fn add(first: Types, second: Types) -> Types {
    match (first, second) {
        (Types::Integer(a), Types::Integer(b)) => Types::Integer(a + b),
        (Types::Float(a), Types::Float(b)) => Types::Float(a + b),
        (Types::Float(a), Types::Integer(b)) => Types::Float(a + (b as f64)),
        (Types::Integer(a), Types::Float(b)) => Types::Float((a as f64) + b),
        (Types::String(a), Types::String(b)) => Types::String(format!("{}{}", a, b)),
        (_, _) => panic!("Oh boy, I'm in danger!"),
    }
}

fn subtract(first: Types, second: Types) -> Types {
    match (first, second) {
        (Types::Integer(a), Types::Integer(b)) => Types::Integer(a - b),
        (Types::Float(a), Types::Integer(b)) => Types::Float(a - (b as f64)),
        (Types::Integer(a), Types::Float(b)) => Types::Float((a as f64) - b),
        (_, _) => panic!("Oh boy, I'm in danger!"),
    }
}

fn divide(first: Types, second: Types) -> Types {
    match (first, second) {
        (Types::Integer(a), Types::Integer(b)) if a == 0 || b == 0 => panic!("Division by zero"),
        (Types::Float(a), Types::Float(b)) => Types::Float(a / b),
        (Types::Integer(a), Types::Integer(b)) => Types::Float((a as f64) / (b as f64)),
        (Types::Integer(a), Types::Float(b)) => Types::Float((a as f64) / b),
        (Types::Float(a), Types::Integer(b)) => Types::Float(a / (b as f64)),
        (_, _) => panic!("Oh boy, I'm in danger!"),
    }
}

fn multiply(first: Types, second: Types) -> Types {
    match (first, second) {
        (Types::Integer(a), Types::Integer(b)) => Types::Integer(a * b),
        (Types::Integer(a), Types::Float(b)) => Types::Float((a as f64) * b),
        (Types::Float(a), Types::Integer(b)) => Types::Float(a * (b as f64)),
        (Types::Float(a), Types::Float(b)) => Types::Float(a * b),
        (_, _) => panic!("Oh boy, I'm in danger!"),
    }
}

fn main() {
    let args: Vec<String> = envargs::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut tokens = tokenizer::tokenize(contents.as_str());
    let ast = parser::parse(&mut tokens);

    let env = env::new_env(None);

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

    let result = interpreter::execute(&env, ast);

    println!("{:?}", result)
}
