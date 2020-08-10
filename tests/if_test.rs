mod common;
use common::assert_eq;
use myla::core::types::Types::{Integer, String as Str};

#[test]
fn test_true() {
    assert_eq("(if true 1 2)", Integer(1));
}

#[test]
fn test_false() {
    assert_eq("(if false 1 2)", Integer(2));
}

#[test]
fn test_predicate() {
    assert_eq(r#"(if (> 2 1) "a" "b")"#, Str("a".to_string()));
}

#[test]
fn test_left_expression() {
    assert_eq(r#"(if true (+ 1 2) (+ 3 4))"#, Integer(3));
}

#[test]
fn test_right_expression() {
    assert_eq(r#"(if false (+ 1 2) (+ 3 4))"#, Integer(7));
}
