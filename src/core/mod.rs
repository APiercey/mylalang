pub mod env;
mod native_functions;
pub mod types;
use crate::core::env::{set_env, Env};
use crate::core::native_functions::{add, divide, inspect, multiply, subtract};
use crate::core::types::{define_function, Types, VArgs};

pub fn set_core_functions(env: &Env) {
    set_env(
        &env,
        "add",
        define_function(|args: VArgs| add(args[0].clone(), args[1].clone())),
    );

    set_env(
        &env,
        "subtract",
        define_function(|args: VArgs| subtract(args[0].clone(), args[1].clone())),
    );

    set_env(
        &env,
        "divide",
        define_function(|args: VArgs| divide(args[0].clone(), args[1].clone())),
    );

    set_env(
        &env,
        "multiply",
        define_function(|args: VArgs| multiply(args[0].clone(), args[1].clone())),
    );

    set_env(
        &env,
        "inspect",
        define_function(|args: VArgs| inspect(args[0].clone())),
    );

    set_env(&env, "true", Types::Bool(true));
    set_env(&env, "false", Types::Bool(false));
}
