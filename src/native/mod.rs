use crate::core::env::Env;
use crate::evaluate as root_evaluate;

const ARITHMETIC_OPERATORS: &str = include_str!("arithmetic_operators.my");
const COMPARISON_OPERATORS: &str = include_str!("comparison_operators.my");
const CONSTRUCTOR_OPERATORS: &str = include_str!("constructor_operators.my");
const FILE_FUNCTIONS: &str = include_str!("file_functions.my");
const HIGHER_ORDER_FUNCTIONS: &str = include_str!("higher_order_functions.my");
const MATH_FUNCTIONS: &str = include_str!("math_functions.my");
const SEARCH_FUNCTIONS: &str = include_str!("search_functions.my");

pub fn load(env: &Env) {
    root_evaluate(&env, ARITHMETIC_OPERATORS);
    root_evaluate(&env, COMPARISON_OPERATORS);
    root_evaluate(&env, CONSTRUCTOR_OPERATORS);
    root_evaluate(&env, FILE_FUNCTIONS);
    root_evaluate(&env, HIGHER_ORDER_FUNCTIONS);
    root_evaluate(&env, MATH_FUNCTIONS);
    root_evaluate(&env, SEARCH_FUNCTIONS);
}
