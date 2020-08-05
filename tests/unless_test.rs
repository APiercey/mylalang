mod common;
use common::assert_eq;
use myla::core::types::Types::{Float, Integer, String as Str};

#[test]
fn test_true() {
    assert_eq("(unless true 1 2)", Integer(2));
}

#[test]
fn test_false() {
    assert_eq("(unless false 1 2)", Integer(1));
}

#[test]
fn test_predicate() {
    assert_eq(r#"(unless (> 2 1) "a" "b")"#, Str("b".to_string()));
}

#[test]
fn test_left_expression() {
    assert_eq(r#"(unless true (+ 1 2) (+ 3 4))"#, Integer(7));
}

#[test]
fn test_right_expression() {
    assert_eq(r#"(unless false (+ 1 2) (+ 3 4))"#, Integer(3));
}
