use myla::core;
use myla::core::env;
use myla::evaluator;
use myla::parser;
use myla::tokenizer;
use std::env as envargs;
use std::fs;

fn main() {
    let args: Vec<String> = envargs::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let env = env::new_env(None);

    core::setup_core_environment(&env);

    let mut tokens = tokenizer::tokenize(contents.as_str());

    let ast = parser::parse(&mut tokens);

    let mut iter = ast.iter();
    while let Some(next) = iter.next() {
        evaluator::evaluate(env.clone(), next.clone());
    }
}
