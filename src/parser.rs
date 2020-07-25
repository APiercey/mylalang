use super::tokenizer;

#[derive(Debug)]
pub struct AbstractSyntaxTree {
    pub body: Vec<Node>,
}

#[derive(Debug)]
pub struct NumberLiteral {
    pub value: isize,
}

#[derive(Debug)]
pub struct StringLiteral {
    pub value: String,
}

#[derive(Debug)]
pub struct CallExpression {
    pub name: String,
    pub params: Vec<Node>,
}

#[derive(Debug)]
pub enum Node {
    NumberLiteral(NumberLiteral),
    StringLiteral(StringLiteral),
    CallExpression(CallExpression),
}

fn parse_number(token: &tokenizer::Token) -> Node {
    return Node::NumberLiteral(NumberLiteral {
        value: token
            .value
            .iter()
            .cloned()
            .collect::<String>()
            .parse::<isize>()
            .unwrap(),
    });
}

fn parse_string(token: &tokenizer::Token) -> Node {
    let string_slice = &token.value[1..token.value.len() - 1];

    return Node::StringLiteral(StringLiteral {
        value: string_slice.iter().cloned().collect::<String>(),
    });
}

fn parse_expression(
    input: &mut std::slice::Iter<tokenizer::Token>,
    token: &tokenizer::Token,
) -> Node {
    let mut call_expression = CallExpression {
        name: token.value.iter().cloned().collect::<String>(),
        params: vec![],
    };

    while let Some(next) = input.next() {
        match next.kind {
            tokenizer::Kinds::ClosingParam => break,
            _ => call_expression.params.push(parse_token(input, &next)),
        }
    }

    return Node::CallExpression(call_expression);
}

fn parse_token(input: &mut std::slice::Iter<tokenizer::Token>, token: &tokenizer::Token) -> Node {
    match token.kind {
        tokenizer::Kinds::Number => parse_number(&token),
        tokenizer::Kinds::Word => parse_string(&token),
        tokenizer::Kinds::OpeningParam => match input.next() {
            None => panic!("Early termination"),
            Some(expression_token) => parse_expression(input, &expression_token),
        },
        _ => panic!("Unknown token"),
    }
}

pub fn parse(tokens: &mut std::vec::Vec<tokenizer::Token>) -> AbstractSyntaxTree {
    let mut ast = AbstractSyntaxTree { body: vec![] };
    let mut t_iterable = tokens.iter();

    while let Some(next) = t_iterable.next() {
        ast.body.push(parse_token(&mut t_iterable, &next))
    }

    return ast;
}
