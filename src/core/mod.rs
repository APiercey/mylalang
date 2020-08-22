mod arithmetic_operators;
pub mod comparison_operators;
mod construction_functions;
pub mod env;
mod file_functions;
mod output_functions;
pub mod types;
use crate::core::arithmetic_operators::{add, divide, multiply, subtract};
use crate::core::comparison_operators::{empty, eq, gt, lt};
use crate::core::construction_functions::{assoc, cons, head, into, list, tail};
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
        "add",
        define_function(|args: VArgs| add(&args[0], &args[1])),
    );

    set_env(
        &env,
        "subtract",
        define_function(|args: VArgs| subtract(&args[0], &args[1])),
    );

    set_env(
        &env,
        "divide",
        define_function(|args: VArgs| divide(&args[0], &args[1])),
    );

    set_env(
        &env,
        "multiply",
        define_function(|args: VArgs| multiply(&args[0], &args[1])),
    );

    set_env(
        &env,
        "inspect",
        define_function(|args: VArgs| inspect(&args[0])),
    );

    set_env(
        &env,
        "eq?",
        define_function(|args: VArgs| eq(&args[0], &args[1])),
    );

    set_env(
        &env,
        "gt?",
        define_function(|args: VArgs| gt(&args[0], &args[1])),
    );

    set_env(
        &env,
        "lt?",
        define_function(|args: VArgs| lt(&args[0], &args[1])),
    );

    set_env(
        &env,
        "cons",
        define_function(|args: VArgs| cons(&args[0], &args[1])),
    );

    set_env(&env, "into", define_function(|args: VArgs| into(&args)));
    set_env(&env, "assoc", define_function(|args: VArgs| assoc(&args)));
    set_env(&env, "list", define_function(|args: VArgs| list(&args)));

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
