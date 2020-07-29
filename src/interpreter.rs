use crate::env::{get_env, Env};
use crate::types::Types;

pub fn execute(env: &Env, ast: Types) -> Types {
    match ast {
        Types::List(ref l) => match get_env(&env, &l[0]) {
            Ok(t) => {
                let mut args = vec![];
                let mut iter = l[1..].iter();

                while let Some(next) = iter.next() {
                    args.push(execute(&env, next.clone()));
                }

                return t.apply(args);
            }
            Err(_) => panic!("error..."),
        },
        t => t,
    }
}
