mod interpreter;
mod parser;
mod tokenizer;
mod types;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut tokens = tokenizer::tokenize(contents.as_str());
    let ast = parser::parse(&mut tokens);

    println!("{:?}", ast);
    // interpreter::execute_tree(ast);
}
