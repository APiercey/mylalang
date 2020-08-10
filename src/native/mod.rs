use crate::core::env::Env;
use crate::evaluate as root_evaluate;

const FUNCTIONS: &str = include_str!("functions.my");
const FILES: &str = include_str!("files.my");

pub fn load(env: &Env) {
    root_evaluate(&env, FUNCTIONS);
    root_evaluate(&env, FILES);
}
