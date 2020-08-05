mod common;
use common::assert_eq;
use myla::core::types::Types::{Bool, Float, Integer, Nil, String as Str};

#[test]
fn test_simple_expressions() {
    assert_eq("(do 1 2 3)", Integer(3));
}

#[test]
fn test_complex_expressions() {
    assert_eq(r#" (do (if true (+ 1 1) nil) (> 1 2))"#, Bool(false));
}

#[test]
fn test_do_anon_functions() {
    assert_eq(
        r#"
        (do ((fn [i] (+ i i)) 1))"#,
        Integer(2),
    );
}

#[test]
fn test_empty_is_nil() {
    assert_eq("(do)", Nil);
}
