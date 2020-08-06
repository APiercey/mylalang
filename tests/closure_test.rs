mod common;
use common::assert_eq;
use myla::core::types::Types::{Float, Integer, String as Str};

#[test]
fn test_closure_over_params() {
    assert_eq(
        r#"
        (def adder (fn [a] (fn [b] (+ a b))))
        (def addFive (adder 5))
        (addFive 5)
        "#,
        Integer(10),
    );
}

#[test]
fn test_closure_over_params_and_let() {
    assert_eq(
        r#"
        (def addTen (fn [a]
          (let [b 10]
            (fn [c] (+ a (+ b c))))))

        (def addThirteen (addTen 3))
        (addThirteen 10)
        "#,
        Integer(23),
    );
}
