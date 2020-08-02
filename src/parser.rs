use crate::core::types::{vec_to_list, vec_to_vector, Types};
use crate::tokenizer;

fn parse_number(token: &tokenizer::Token) -> Types {
    let value_as_string = token.value.iter().cloned().collect::<String>();

    return match token.value.contains(&'.') {
        true => Types::Float(value_as_string.parse::<f64>().unwrap()),
        false => Types::Integer(value_as_string.parse::<isize>().unwrap()),
    };
}

fn parse_word(token: &tokenizer::Token) -> Types {
    return Types::Word(token.value.iter().cloned().collect::<String>());
}

fn parse_string(token: &tokenizer::Token) -> Types {
    let string_slice = &token.value[1..token.value.len() - 1];

    return Types::String(string_slice.iter().cloned().collect::<String>());
}

fn parse_expression(input: &mut std::slice::Iter<tokenizer::Token>) -> Types {
    match input.next() {
        None => panic!("Early termination"),
        Some(token) => {
            let mut list = vec![parse_token(input, &token)];

            while let Some(next) = input.next() {
                match next.kind {
                    tokenizer::Kinds::ClosingParam => break,
                    _ => list.push(parse_token(input, &next)),
                }
            }
            return vec_to_list(list);
        }
    }
}

fn parse_vector(input: &mut std::slice::Iter<tokenizer::Token>) -> Types {
    let mut list = vec![];

    while let Some(next) = input.next() {
        match next.kind {
            tokenizer::Kinds::ClosingVectorBracket => break,
            _ => list.push(parse_token(input, &next)),
        }
    }

    return vec_to_vector(list);
}

fn parse_token(input: &mut std::slice::Iter<tokenizer::Token>, token: &tokenizer::Token) -> Types {
    match token.kind {
        tokenizer::Kinds::Number => parse_number(&token),
        tokenizer::Kinds::Str => parse_string(&token),
        tokenizer::Kinds::Word => parse_word(&token),
        tokenizer::Kinds::OpeningParam => parse_expression(input),
        tokenizer::Kinds::OpeningVectorBracket => parse_vector(input),
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
