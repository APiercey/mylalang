use crate::core::env::{get_env, set_env, Env};
use crate::core::types::Types;
use std::rc::Rc;

fn evaluate_ast(env: Env, ast: Types) -> Types {
    match ast {
        Types::Integer(_) => ast,
        Types::Word(_) => match get_env(&env, &ast) {
            Ok(t) => t,
            Err(_) => panic!("Error, definition does not exist"),
        },
        _ => panic!("Unsure what to do here"),
    }
}

pub fn evaluate(env: Env, ast: Types) -> Types {
    match ast {
        Types::List(ref l) => {
            let action = &l[0].clone();

            match action {
                Types::List(ref _fl) => {
                    let result = evaluate(env, action.clone());

                    let params = l[1..].to_vec();

                    return result.apply(params);
                }
                Types::Word(word) => match word.as_str() {
                    "def" => {
                        let def_name = l[1].clone();
                        let def_value = l[2].clone();

                        match def_name {
                            Types::Word(ref def_name_value) => set_env(
                                &env,
                                &def_name_value,
                                evaluate(env.clone(), def_value.clone()),
                            ),
                            _ => panic!("Unexpected input"),
                        };

                        return def_name;
                    }
                    "fn" => {
                        let params = l[1].clone();
                        let body = l[2].clone();

                        let f = Types::DefFunc {
                            eval: evaluate,
                            env: env.clone(),
                            params: Rc::new(params),
                            body: Rc::new(body),
                        };

                        return f;
                    }
                    "if" => {
                        let predicate = l[1].clone();
                        let if_true_expression = l[2].clone();
                        let if_false_expression = l[3].clone();

                        let bool_result = match evaluate(env.clone(), predicate.clone()) {
                            Types::Bool(b) => b,
                            Types::Integer(i) => i > 0,
                            Types::String(s) => s.len() > 0,
                            Types::Vector(v) => v.len() > 0,
                            _ => false,
                        };

                        return if bool_result {
                            evaluate(env, if_true_expression)
                        } else {
                            evaluate(env, if_false_expression)
                        };
                    }
                    _ => {
                        let t = evaluate_ast(env.clone(), action.clone());

                        match t {
                            Types::Func(_) | Types::DefFunc { .. } => {
                                let mut args = vec![];
                                let mut iter = l[1..].iter();

                                while let Some(next) = iter.next() {
                                    let result = evaluate(env.clone(), next.clone());
                                    args.push(result);
                                }

                                t.apply(args)
                            }
                            _ => panic!("Expected function"),
                        }
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
