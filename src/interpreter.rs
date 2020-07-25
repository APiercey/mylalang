use super::parser;

#[derive(Debug)]
enum Types {
    Integer(isize),
    Float(f64),
    String(String),
}

fn add(first: Types, second: Types) -> Types {
    match (first, second) {
        (Types::Integer(a), Types::Integer(b)) => Types::Integer(a + b),
        (Types::Float(a), Types::Float(b)) => Types::Float(a + b),
        (Types::Float(a), Types::Integer(b)) => Types::Float(a + (b as f64)),
        (Types::Integer(a), Types::Float(b)) => Types::Float((a as f64) + b),
        (Types::String(a), Types::String(b)) => Types::String(format!("{}{}", a, b)),
        (_, _) => panic!("Oh boy, I'm in danger!"),
    }
}

fn subtract(first: Types, second: Types) -> Types {
    match (first, second) {
        (Types::Integer(a), Types::Integer(b)) => Types::Integer(a - b),
        (Types::Float(a), Types::Integer(b)) => Types::Float(a - (b as f64)),
        (Types::Integer(a), Types::Float(b)) => Types::Float((a as f64) - b),
        (_, _) => panic!("Oh boy, I'm in danger!"),
    }
}

fn divide(first: Types, second: Types) -> Types {
    match (first, second) {
        (Types::Integer(a), Types::Integer(b)) if a == 0 || b == 0 => panic!("Division by zero"),
        (Types::Float(a), Types::Float(b)) => Types::Float(a / b),
        (Types::Integer(a), Types::Integer(b)) => Types::Float((a as f64) / (b as f64)),
        (Types::Integer(a), Types::Float(b)) => Types::Float((a as f64) / b),
        (Types::Float(a), Types::Integer(b)) => Types::Float(a / (b as f64)),
        (_, _) => panic!("Oh boy, I'm in danger!"),
    }
}

fn multiply(first: Types, second: Types) -> Types {
    match (first, second) {
        (Types::Integer(a), Types::Integer(b)) => Types::Integer(a * b),
        (Types::Integer(a), Types::Float(b)) => Types::Float((a as f64) * b),
        (Types::Float(a), Types::Integer(b)) => Types::Float(a * (b as f64)),
        (Types::Float(a), Types::Float(b)) => Types::Float(a * b),
        (_, _) => panic!("Oh boy, I'm in danger!"),
    }
}

fn execute(program: &parser::Node) -> Types {
    match program {
        parser::Node::CallExpression(node) => match node.name.as_str() {
            "add" => add(execute(&node.params[0]), execute(&node.params[1])),
            "subtract" => subtract(execute(&node.params[0]), execute(&node.params[1])),
            "divide" => divide(execute(&node.params[0]), execute(&node.params[1])),
            "multiply" => multiply(execute(&node.params[0]), execute(&node.params[1])),
            _ => panic!("big oof"),
        },
        parser::Node::NumberLiteral(node) => Types::Integer(node.value),
        parser::Node::StringLiteral(node) => Types::String(node.value.clone()),
    }
}

pub fn execute_tree(ast: parser::AbstractSyntaxTree) {
    let mut programs = ast.body.iter();

    while let Some(program) = programs.next() {
        match execute(program) {
            Types::Integer(n) => println!("{}", n),
            Types::Float(n) => println!("{}", n),
            Types::String(n) => println!("{}", n),
        };
    }
}
