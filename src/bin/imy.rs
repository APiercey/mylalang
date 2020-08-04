use myla::core;
use myla::core::env;
use myla::native;
use rustyline::error::ReadlineError;
use rustyline::Editor;

fn main() {
    let mut rl = Editor::<()>::new();
    if rl.load_history("/tmp/mylalang_history.txt").is_err() {
        // do nothing
    }

    let env = env::new_env(None);
    core::setup_core_environment(&env);
    native::load(&env);

    loop {
        let readline = rl.readline("imy> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                let result = myla::evaluate(&env, line.as_str());
                println!("=> {}", result.inspect());
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
