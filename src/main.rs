mod core;
mod env;
mod interpreter;
mod parser;
mod tokenizer;
mod types;
use std::env as envargs;
use std::fs;

fn main() {
    let args: Vec<String> = envargs::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut tokens = tokenizer::tokenize(contents.as_str());
    let ast = parser::parse(&mut tokens);
    let env = env::new_env(None);

    core::set_core_functions(&env);

    let mut iter = ast.iter();
    while let Some(next) = iter.next() {
        interpreter::execute(env.clone(), next.clone());
    }
}
