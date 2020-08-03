mod common;
use common::assert_eq;
use myla::core::types::Types::{Float, Integer};

#[test]
fn test_add_integers() {
    assert_eq("(+ 1 2)", Integer(3));
    assert_eq("(+ -5 5)", Integer(0));
    assert_eq("(+ 4 -45)", Integer(-41));
    assert_eq("(+ -10 -3)", Integer(-13));
}

#[test]
fn test_add_floats() {
    assert_eq("(+ 1.0 1.0)", Float(2.0));
    assert_eq("(+ -5.25 3.25)", Float(-2.0));
    assert_eq("(+ -10.1 -3.1)", Float(-13.2));
    // Remember. Say no to floats, kids!
    assert_eq("(+ 4.50 -5.35)", Float(-0.8499999999999996));
}
