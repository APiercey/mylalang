use super::tokenizer;
use crate::types::{vec_to_list, Types};

fn parse_number(token: &tokenizer::Token) -> Types {
    return Types::Integer(
        token
            .value
            .iter()
            .cloned()
            .collect::<String>()
            .parse::<isize>()
            .unwrap(),
    );
}

fn parse_word(token: &tokenizer::Token) -> Types {
    return Types::Word(token.value.iter().cloned().collect::<String>());
}

fn parse_string(token: &tokenizer::Token) -> Types {
    let string_slice = &token.value[1..token.value.len() - 1];

    return Types::String(string_slice.iter().cloned().collect::<String>());
}

fn parse_expression(
    input: &mut std::slice::Iter<tokenizer::Token>,
    token: &tokenizer::Token,
) -> Types {
    let mut list = vec![parse_word(&token)];

    while let Some(next) = input.next() {
        match next.kind {
            tokenizer::Kinds::ClosingParam => break,
            _ => list.push(parse_token(input, &next)),
        }
    }

    return vec_to_list(list);
}

fn parse_token(input: &mut std::slice::Iter<tokenizer::Token>, token: &tokenizer::Token) -> Types {
    match token.kind {
        tokenizer::Kinds::Number => parse_number(&token),
        tokenizer::Kinds::Str => parse_string(&token),
        tokenizer::Kinds::Word => parse_word(&token),
        tokenizer::Kinds::OpeningParam => match input.next() {
            None => panic!("Early termination"),
            Some(expression_token) => parse_expression(input, &expression_token),
        },
        _ => panic!("Unknown token"),
    }
}

pub fn parse(tokens: &mut std::vec::Vec<tokenizer::Token>) -> Vec<Types> {
    let mut t_iterable = tokens.iter();
    let mut acc = vec![];

    while let Some(next) = t_iterable.next() {
        let result = parse_token(&mut t_iterable, &next);
        acc.push(result);
    }

    return acc;
}
