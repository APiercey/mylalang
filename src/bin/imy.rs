use myla::core;
use myla::core::env;
use myla::evaluator;
use myla::parser;
use myla::tokenizer;
use rustyline::error::ReadlineError;
use rustyline::Editor;

fn main() {
    let mut rl = Editor::<()>::new();
    if rl.load_history("/tmp/mylalang_history.txt").is_err() {
        // do nothing
    }

    let env = env::new_env(None);
    core::setup_core_environment(&env);

    loop {
        let readline = rl.readline("imy> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                let mut tokens = tokenizer::tokenize(line.as_str());
                let ast = parser::parse(&mut tokens);

                let mut iter = ast.iter();
                while let Some(next) = iter.next() {
                    let inspection = evaluator::evaluate(env.clone(), next.clone()).inspect();
                    println!("=> {}", inspection);
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    rl.save_history("/tmp/mylalang_history.txt").unwrap();
}
