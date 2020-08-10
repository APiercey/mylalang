mod arithmetic_operators;
pub mod comparison_operators;
mod construction_functions;
pub mod env;
mod file_functions;
mod output_functions;
pub mod types;
use crate::core::arithmetic_operators::{add, divide, multiply, subtract};
use crate::core::comparison_operators::{empty, eq, gt, gt_or_eq, lt, lt_or_eq};
use crate::core::construction_functions::{cons, head, tail};
use crate::core::env::{set_env, Env};
use crate::core::file_functions::read_from_file;
use crate::core::output_functions::inspect;
use crate::core::types::{define_function, Types, VArgs};

pub fn load(env: &Env) {
    // Set runtime types
    set_env(&env, "true", Types::Bool(true));
    set_env(&env, "false", Types::Bool(false));
    set_env(&env, "nil", Types::Nil);

    // Set runtime functions and operators
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

    set_env(
        &env,
        "readfile",
        define_function(|args: VArgs| read_from_file(&args[0])),
    );
}
