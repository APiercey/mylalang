pub mod env;
mod native_functions;
pub mod types;
use crate::core::env::{set_env, Env};
use crate::core::native_functions::{add, divide, inspect, multiply, subtract};
use crate::core::types::{define_function, Types, VArgs};

pub fn set_core_functions(env: &Env) {
    set_env(
        &env,
        "+",
        define_function(|args: VArgs| add(args[0].clone(), args[1].clone())),
    );

    set_env(
        &env,
        "-",
        define_function(|args: VArgs| subtract(args[0].clone(), args[1].clone())),
    );

    set_env(
        &env,
        "/",
        define_function(|args: VArgs| divide(args[0].clone(), args[1].clone())),
    );

    set_env(
        &env,
        "*",
        define_function(|args: VArgs| multiply(args[0].clone(), args[1].clone())),
    );

    set_env(
        &env,
        "inspect",
        define_function(|args: VArgs| inspect(args[0].clone())),
    );

    set_env(
        &env,
        "=",
        define_function(|args: VArgs| args[0].eq(&args[1])),
    );

    set_env(
        &env,
        ">",
        define_function(|args: VArgs| args[0].gt(&args[1])),
    );

    set_env(
        &env,
        "<",
        define_function(|args: VArgs| args[0].lt(&args[1])),
    );

    set_env(
        &env,
        "<=",
        define_function(|args: VArgs| args[0].lt_or_eq(&args[1])),
    );

    set_env(
        &env,
        ">=",
        define_function(|args: VArgs| args[0].gt_or_eq(&args[1])),
    );

    set_env(&env, "true", Types::Bool(true));
    set_env(&env, "false", Types::Bool(false));
    set_env(&env, "nil", Types::Nil);
}
