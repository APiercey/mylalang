mod common;
use common::assert_eq;
use myla::core::types::Types::{Bool, Float, Integer, Nil, String as Str};

#[test]
fn test_evaluating_values() {
    assert_eq(
        r#"(eval "(def a 1)")
        (inspect a)"#,
        Integer(1),
    );
}

#[test]
fn test_evaluating_functions() {
    assert_eq(
        r#"(eval "(def inc (fn [i] (+ i 1)))")
        (inc 1)"#,
        Integer(2),
    );
}
