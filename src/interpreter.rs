use crate::env::{get_env, set_env, Env};
use crate::types::Types;

fn evaluate_ast(env: &Env, ast: Types) -> Types {
    match ast {
        Types::Word(_) => match get_env(&env, &ast) {
            Ok(t) => t,
            Err(_) => panic!("Error, definition does not exist"),
        },
        _ => panic!("Unsure what to do here"),
    }
}

pub fn execute(env: &Env, ast: Types) -> Types {
    match ast {
        Types::List(ref l) => {
            let action = &l[0].clone();

            match action {
                Types::Word(word) => match word.as_str() {
                    "def" => {
                        let def_name = l[1].clone();
                        let def_value = l[2].clone();

                        match def_name {
                            Types::Word(ref def_name_value) => {
                                set_env(&env, &def_name_value, def_value)
                            }
                            _ => panic!("Unexecpted input"),
                        };

                        return def_name;
                    }
                    _ => {
                        let t = evaluate_ast(&env, action.clone());

                        let mut args = vec![];
                        let mut iter = l[1..].iter();

                        while let Some(next) = iter.next() {
                            let result = execute(&env, next.clone());
                            args.push(result);
                        }

                        return t.apply(args);
                    }
                },
                _ => panic!("Needed list"),
            }
        }
        Types::Word(_) => match get_env(&env, &ast) {
            Ok(t) => t,
            Err(_) => panic!("Error, definition does not exist"),
        },
        t => t,
    }
}
