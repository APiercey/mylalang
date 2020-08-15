#[derive(Debug)]
pub enum Kinds {
    Null,
    OpeningParam,
    ClosingParam,
    OpeningVectorBracket,
    ClosingVectorBracket,
    Number,
    Word,
    Str,
}

#[derive(Debug)]
pub struct Token {
    pub consumes: usize,
    pub kind: Kinds,
    pub value: Vec<char>,
}

fn tokenize_comment(input: &str) -> Token {
    let mut acc = vec![];
    let mut chars = input.chars();

    return tokenize_comment_r(&mut acc, &mut chars);
}

fn tokenize_comment_r(acc: &mut Vec<char>, input: &mut impl Iterator<Item = char>) -> Token {
    return match input.next() {
        None => Token {
            consumes: 0,
            kind: Kinds::Null,
            value: vec![],
        },
        Some(x) => match x {
            ';' if { acc.len() == 0 } => {
                acc.push(x);
                tokenize_comment_r(acc, input)
            }
            '\n' if { acc.len() > 0 } => Token {
                consumes: acc.len(),
                kind: Kinds::Null,
                value: vec![],
            },
            _ if { acc.len() > 0 } => {
                acc.push(x);
                tokenize_comment_r(acc, input)
            }
            _ => Token {
                consumes: 0,
                kind: Kinds::Null,
                value: vec![],
            },
        },
    };
}

fn tokenize_character(kind: Kinds, character: char, input: &str) -> Token {
    return match input.chars().next() {
        Some(x) if x == character => Token {
            consumes: 1,
            kind: kind,
            value: vec![x],
        },
        _ => Token {
            consumes: 0,
            kind: Kinds::Null,
            value: vec![],
        },
    };
}

// fn tokenize_operator(input: &str) -> Token {
//     let mut acc = vec![];
//     let mut chars = input.chars();

//     return tokenize_operator_r(&mut acc, &mut chars);
// }

// fn tokenize_operator_r(acc: &mut Vec<char>, input: &mut impl Iterator<Item = char>) -> Token {
//     return match input.next() {
//         None => Token {
//             consumes: 0,
//             kind: Kinds::Null,
//             value: vec![],
//         },
//         Some(x) => match x {
//             '=' if { acc.len() > 0 } => {
//                 acc.push(x);

//                 return Token {
//                     consumes: acc.len(),
//                     kind: Kinds::Word,
//                     value: acc.to_vec(),
//                 };
//             }
//             '>' | '<' => {
//                 acc.push(x);
//                 tokenize_operator_r(acc, input)
//             }
//             '+' | '-' | '*' | '/' | '=' | ':' => Token {
//                 consumes: 1,
//                 kind: Kinds::Word,
//                 value: vec![x],
//             },
//             _ if { acc.len() > 0 } => Token {
//                 consumes: acc.len(),
//                 kind: Kinds::Word,
//                 value: acc.to_vec(),
//             },
//             _ => Token {
//                 consumes: 0,
//                 kind: Kinds::Null,
//                 value: vec![],
//             },
//         },
//     };
// }

fn tokenize_whitespace(input: &str) -> Token {
    return tokenize_character(Kinds::Null, ' ', input);
}

fn tokenize_new_line(input: &str) -> Token {
    return tokenize_character(Kinds::Null, '\n', input);
}

fn tokenize_opening_param(input: &str) -> Token {
    return tokenize_character(Kinds::OpeningParam, '(', input);
}

fn tokenize_closing_param(input: &str) -> Token {
    return tokenize_character(Kinds::ClosingParam, ')', input);
}

fn tokenize_opening_vector_bracket(input: &str) -> Token {
    return tokenize_character(Kinds::OpeningVectorBracket, '[', input);
}

fn tokenize_closing_vector_bracket(input: &str) -> Token {
    return tokenize_character(Kinds::ClosingVectorBracket, ']', input);
}

fn tokenize_number(input: &str) -> Token {
    let mut acc = vec![];
    let mut chars = input.chars();

    return tokenize_number_r(&mut acc, &mut chars);
}

fn tokenize_number_r(acc: &mut Vec<char>, input: &mut impl Iterator<Item = char>) -> Token {
    return match input.next() {
        None => Token {
            consumes: 0,
            kind: Kinds::Null,
            value: vec![],
        },
        Some(x) => match x {
            '-' => {
                acc.push(x);
                tokenize_number_r(acc, input)
            }
            '0'..='9' | '.' => {
                acc.push(x);
                tokenize_number_r(acc, input)
            }
            '_' if { acc.len() > 0 } => {
                acc.push(x);
                tokenize_number_r(acc, input)
            }
            _ if { acc.len() > 0 } => match (acc[0], acc.len()) {
                ('-', 1) => Token {
                    consumes: 0,
                    kind: Kinds::Null,
                    value: vec![],
                },
                (_, _) => Token {
                    consumes: acc.len(),
                    kind: Kinds::Number,
                    value: acc.to_vec(),
                },
            },
            _ => Token {
                consumes: 0,
                kind: Kinds::Null,
                value: vec![],
            },
        },
    };
}

fn tokenize_string(input: &str) -> Token {
    let mut acc = vec![];
    let mut chars = input.chars();

    return tokenize_string_r(&mut acc, &mut chars);
}

fn tokenize_string_r(acc: &mut Vec<char>, input: &mut impl Iterator<Item = char>) -> Token {
    return match input.next() {
        None if { acc.len() > 0 } => {
            panic!("Unterminated String! ...I'll be back...");
        }
        None => Token {
            consumes: 0,
            kind: Kinds::Null,
            value: vec![],
        },
        Some('"') if { acc.len() > 0 } => {
            acc.push('"');
            Token {
                consumes: acc.len(),
                kind: Kinds::Str,
                value: acc.to_vec(),
            }
        }
        Some('"') => {
            acc.push('"');
            tokenize_string_r(acc, input)
        }
        Some(x) if { acc.len() > 0 } => {
            acc.push(x);
            tokenize_string_r(acc, input)
        }
        _ => Token {
            consumes: 0,
            kind: Kinds::Null,
            value: vec![],
        },
    };
}

fn tokenize_word(input: &str) -> Token {
    let mut acc = vec![];
    let mut chars = input.chars();

    return tokenize_word_r(&mut acc, &mut chars);
}

fn tokenize_word_r(acc: &mut Vec<char>, input: &mut impl Iterator<Item = char>) -> Token {
    return match input.next() {
        None => Token {
            consumes: 0,
            kind: Kinds::Null,
            value: vec![],
        },
        Some(x) => match x {
            'a'..='z' | 'A'..='Z' => {
                acc.push(x);
                tokenize_word_r(acc, input)
            }
            '_' | '0'..='9' | '?' | '!' if { acc.len() > 0 } => {
                acc.push(x);
                tokenize_word_r(acc, input)
            }
            '+' | '-' | '*' | '/' | '=' | ':' | '>' | '<' => {
                acc.push(x);
                tokenize_word_r(acc, input)
            }
            '&' => {
                acc.push('&');
                Token {
                    consumes: acc.len(),
                    kind: Kinds::Word,
                    value: acc.to_vec(),
                }
            }
            _ if { acc.len() > 0 } => Token {
                consumes: acc.len(),
                kind: Kinds::Word,
                value: acc.to_vec(),
            },
            _ => Token {
                consumes: 0,
                kind: Kinds::Null,
                value: vec![],
            },
        },
    };
}

pub fn tokenize(full_prg: &str) -> Vec<Token> {
    let mut chars = full_prg.chars();
    let mut acc = vec![];
    let functions: Vec<&dyn Fn(&str) -> Token> = vec![
        &tokenize_comment,
        &tokenize_whitespace,
        &tokenize_new_line,
        &tokenize_opening_param,
        &tokenize_closing_param,
        &tokenize_opening_vector_bracket,
        &tokenize_closing_vector_bracket,
        &tokenize_string,
        &tokenize_number,
        &tokenize_word,
    ];

    // TODO: How to know when there are no chars left ... ?
    for _a in 0..full_prg.len() {
        for f in &functions {
            let token = f(chars.as_str());
            for _x in 0..token.consumes {
                chars.next();
            }

            match token.kind {
                Kinds::Null => continue,
                _ => acc.push(token),
            }
        }
    }

    return acc;
}
