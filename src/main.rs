mod parser;
mod tokenizer;

#[derive(Debug)]
enum Types {
    Integer(isize),
    String(String),
}

fn add(first: Types, second: Types) -> Types {
    match (first, second) {
        (Types::Integer(a), Types::Integer(b)) => Types::Integer(a + b),
        (_, _) => panic!("string oof"),
    }
}

fn subtract(a: isize, b: isize) -> isize {
    return a + b;
}

fn execute_tree(ast: parser::AbstractSyntaxTree) {
    let mut programs = ast.body.iter();
    while let Some(program) = programs.next() {
        match execute(program) {
            Types::Integer(n) => println!("{}", n),
            Types::String(n) => println!("{}", n),
        };
    }
}

fn execute(program: &parser::Node) -> Types {
    match program {
        parser::Node::CallExpression(node) => match node.name.as_str() {
            "add" => add(execute(&node.params[0]), execute(&node.params[1])),
            _ => panic!("big oof"),
        },
        parser::Node::NumberLiteral(node) => Types::Integer(node.value),
        parser::Node::StringLiteral(node) => Types::String(node.value.clone()),
    }
}

fn main() {
    let program = "(add 123 456)";
    let mut tokens = tokenizer::tokenize(program);
    let ast = parser::parse(&mut tokens);

    execute_tree(ast);
}
