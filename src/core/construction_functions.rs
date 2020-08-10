use crate::core::types::{vec_to_vector, Types};

pub fn cons(head: &Types, tail: &Types) -> Types {
    return match (head, tail) {
        (Types::String(a), Types::String(b)) => Types::String(format!("{}{}", a, b)),
        (a, Types::Vector(ref b)) => {
            let mut new_construct = (**b).clone();
            new_construct.insert(0, a.clone());
            vec_to_vector(new_construct)
        }
        (a, b) => panic!("Cannot cons {:?} into {:?}", a, b),
    };
}

pub fn head(item: &Types) -> Types {
    return match item {
        Types::String(s) => {
            if s.as_str().len() >= 1 {
                return Types::String(s.as_str()[..1].to_string());
            } else {
                return Types::String("".to_string());
            }
        }
        Types::Vector(s) => {
            if s.len() >= 1 {
                s[0].clone()
            } else {
                vec_to_vector(vec![])
            }
        }
        a => panic!("Cannot get the head of {:?}", a),
    };
}

pub fn tail(item: &Types) -> Types {
    return match item {
        Types::String(s) => {
            if s.as_str().len() >= 1 {
                Types::String(s.as_str()[1..].to_string())
            } else {
                Types::String("".to_string())
            }
        }
        Types::Vector(s) => {
            if s.len() >= 1 {
                vec_to_vector(s[1..].to_vec())
            } else {
                vec_to_vector(vec![])
            }
        }
        a => panic!("Cannot get the tail of {:?}", a),
    };
}
