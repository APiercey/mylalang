mod common;
use common::assert_eq;
use myla::core::types::Types::{Bool, Float, Integer, Nil, String as Str};

#[test]
fn test_reading_a_file() {
    assert_eq(
        r#"(readfile "tests/helper_files/hello_world.txt")"#,
        Str("Hello, World!\n".to_string()),
    );
}

#[test]
fn test_def_content_of_file() {
    assert_eq(
        r#"
        (def contents (readfile "tests/helper_files/hello_world.txt"))
        (inspect contents)
        "#,
        Str("Hello, World!\n".to_string()),
    );
}
