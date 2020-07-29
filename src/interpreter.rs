use crate::env::{get_env, Env};
use crate::types::Types;

pub fn execute(env: &Env, ast: &Types) {
    let result = match &ast {
        Types::List(ref l) => match get_env(&env, &l[0]) {
            Ok(t) => t.apply(l[1..].to_vec()),
            Err(_) => panic!("error..."),
        },
        _ => panic!("Lets chill for a min"),
    };

    println!("{:?}", result)
}
