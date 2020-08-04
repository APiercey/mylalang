use myla::core;
use myla::core::env;
use myla::native;
use std::env as envargs;
use std::fs;

fn main() {
    let args: Vec<String> = envargs::args().collect();
    let entry_file = &args[1];

    let env = env::new_env(None);
    core::load(&env);
    native::load(&env);

    let contents = fs::read_to_string(entry_file).expect("Something went wrong reading the file");
    let result = myla::evaluate(&env, contents.as_str());
    println!("=> {}", result.inspect());
}
