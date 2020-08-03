mod common;
use common::assert_eq;
use myla::core::types::Types::{Bool, Float, Integer, String as Str};

#[test]
fn test_defining_integers() {
    assert_eq("(def one 1)(inspect one)", Integer(1));
}

#[test]
fn test_defining_strings() {
    assert_eq(
        r#"(def str "Hello, World.")(inspect str)"#,
        Str("Hello, World.".to_string()),
    );
}

#[test]
fn test_defining_floats() {
    assert_eq(r#"(def flt 1.45)(inspect flt)"#, Float(1.45));
}

#[test]
fn test_defining_bools() {
    assert_eq("(def chruw true)(inspect chruw)", Bool(true));
    assert_eq("(def phalc false)(inspect phalc)", Bool(false));
}

// TODO: implement Eq matching for complex structures
// #[test]
// fn test_defining_vectors() {
//     assert_eq(
//         "(def coll [1 2 3])(inspect coll)",
//         vec_to_list(vec![Integer(1), Integer(2), Integer(3)]),
//     );
// }
