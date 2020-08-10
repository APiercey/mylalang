mod common;
use common::assert_eq;
use myla::core::types::Types::Integer;

#[test]
fn test_defined_functions() {
    let program = "
        (def add (fn [a b] (+ a b)))
        (add 1 2)";

    assert_eq(program, Integer(3));
}

#[test]
fn test_anonymous_functions() {
    let program = "((fn [a b] (* a b)) 4 5)";

    assert_eq(program, Integer(20));
}

#[test]
fn test_functions_with_multiexpression_bodies() {
    {
        let program = "
            (def multi_body (fn [i]
              (let [a i b 2]
                (+ a b)
                a)))

            (multi_body 1)";

        assert_eq(program, Integer(1));
    }
    {
        let program = "
            ((fn [i] (let [a i] (+ i i) a)) 1)";

        assert_eq(program, Integer(1));
    }
}

#[test]
fn test_functions_using_let() {
    {
        let program = "
            (def shifter (fn [i]
              (let [x (* i i)
                    y (+ i i)
                    z (- x y)]
                z)))

            (shifter 6)";

        assert_eq(program, Integer(24));
    }
    {
        let program = "((fn [i] (let [x (* i i) y (+ i i) z (- x y)] z)) 6)";

        assert_eq(program, Integer(24));
    }
}

#[test]
fn test_functions_without_params() {
    {
        let program = "
            (def function_without_params (fn [] 1))

            (function_without_params)";

        assert_eq(program, Integer(1));
    }
    {
        let program = "((fn [] 1))";

        assert_eq(program, Integer(1));
    }
}

#[test]
fn test_variadic_functions() {
    {
        let program = "
            (def variadic_f (fn [a & rest] rest))

            (variadic_f 1 2)";

        assert_eq(program, Integer(1));
    }
}
