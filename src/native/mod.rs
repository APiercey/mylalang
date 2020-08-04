use crate::core::env::Env;
use crate::evaluate as root_evaluate;

const FUNCTIONS: &str = include_str!("functions.my");

pub fn load(env: &Env) {
    root_evaluate(&env, FUNCTIONS);
}
