mod arithmetic_operators;
pub mod comparison_operators;
pub mod env;
mod output_functions;
pub mod types;
use crate::core::arithmetic_operators::{add, divide, multiply, subtract};
use crate::core::comparison_operators::{eq, gt, gt_or_eq, lt, lt_or_eq};
use crate::core::env::{set_env, Env};
use crate::core::output_functions::inspect;
use crate::core::types::{define_function, vec_to_vector, Types, VArgs};

fn set_types(env: &Env) {
    set_env(&env, "true", Types::Bool(true));
    set_env(&env, "false", Types::Bool(false));
    set_env(&env, "nil", Types::Nil);
}

fn cons(head: &Types, tail: &Types) -> Types {
    return match (head, tail) {
        (Types::String(a), Types::String(b)) => Types::String(format!("{}{}", a, b)),
        (a, Types::Vector(ref b)) => {
            let mut new_construct = (**b).clone();
            new_construct.insert(0, a.clone());
            vec_to_vector(new_construct)
        }
        (a, b) => panic!("Cannot cons {:?} into {:?}", a, b),
    };
}

fn head(item: &Types) -> Types {
    return match item {
        Types::String(s) => {
            if s.as_str().len() >= 1 {
                return Types::String(s.as_str()[..1].to_string());
            } else {
                return Types::String("".to_string());
            }
        }
        Types::Vector(s) => {
            if s.len() >= 1 {
                s[0].clone()
            } else {
                vec_to_vector(vec![])
            }
        }
        a => panic!("Cannot get the head of {:?}", a),
    };
}

fn tail(item: &Types) -> Types {
    return match item {
        Types::String(s) => {
            if s.as_str().len() >= 1 {
                Types::String(s.as_str()[1..].to_string())
            } else {
                Types::String("".to_string())
            }
        }
        Types::Vector(s) => {
            if s.len() >= 1 {
                vec_to_vector(s[1..].to_vec())
            } else {
                vec_to_vector(vec![])
            }
        }
        a => panic!("Cannot get the tail of {:?}", a),
    };
}

fn empty(item: &Types) -> Types {
    return match item {
        Types::String(s) => Types::Bool(s.as_str().len() == 0),
        Types::Vector(s) => Types::Bool(s.len() == 0),
        a => panic!("Cannot get the tail of {:?}", a),
    };
}

pub fn load(env: &Env) {
    set_types(&env);

    set_env(
        &env,
        "+",
        define_function(|args: VArgs| add(&args[0], &args[1])),
    );

    set_env(
        &env,
        "-",
        define_function(|args: VArgs| subtract(&args[0], &args[1])),
    );

    set_env(
        &env,
        "/",
        define_function(|args: VArgs| divide(&args[0], &args[1])),
    );

    set_env(
        &env,
        "*",
        define_function(|args: VArgs| multiply(&args[0], &args[1])),
    );

    set_env(
        &env,
        "inspect",
        define_function(|args: VArgs| inspect(&args[0])),
    );

    set_env(
        &env,
        "=",
        define_function(|args: VArgs| eq(&args[0], &args[1])),
    );

    set_env(
        &env,
        ">",
        define_function(|args: VArgs| gt(&args[0], &args[1])),
    );

    set_env(
        &env,
        "<",
        define_function(|args: VArgs| lt(&args[0], &args[1])),
    );

    set_env(
        &env,
        "<=",
        define_function(|args: VArgs| lt_or_eq(&args[0], &args[1])),
    );

    set_env(
        &env,
        ">=",
        define_function(|args: VArgs| gt_or_eq(&args[0], &args[1])),
    );

    set_env(
        &env,
        ":",
        define_function(|args: VArgs| cons(&args[0], &args[1])),
    );

    set_env(&env, "head", define_function(|args: VArgs| head(&args[0])));
    set_env(&env, "tail", define_function(|args: VArgs| tail(&args[0])));
    set_env(
        &env,
        "empty?",
        define_function(|args: VArgs| empty(&args[0])),
    );
}
