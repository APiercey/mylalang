mod arithmetic_operators;
pub mod comparison_operators;
pub mod env;
mod output_functions;
pub mod types;
use crate::core::arithmetic_operators::{add, divide, multiply, subtract};
use crate::core::comparison_operators::{eq, gt, gt_or_eq, lt, lt_or_eq};
use crate::core::env::{set_env, Env};
use crate::core::output_functions::inspect;
use crate::core::types::{define_function, Types, VArgs};

fn set_types(env: &Env) {
    set_env(&env, "true", Types::Bool(true));
    set_env(&env, "false", Types::Bool(false));
    set_env(&env, "nil", Types::Nil);
}

pub fn setup_core_environment(env: &Env) {
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
}
