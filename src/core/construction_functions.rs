use crate::core::types::{vec_to_hash, vec_to_list, vec_to_vector, Types, VArgs};
use std::collections::BTreeMap;

pub fn cons(head: &Types, tail: &Types) -> Types {
    return match (head, tail) {
        (Types::String(a), Types::String(b)) => Types::String(format!("{}{}", a, b)),
        (a, Types::Vector(ref b)) => {
            let mut new_construct = (**b).clone();
            new_construct.insert(0, a.clone());
            vec_to_vector(new_construct)
        }
        (a, Types::List(ref b)) => {
            let mut new_construct = (**b).clone();
            new_construct.insert(0, a.clone());
            vec_to_list(new_construct)
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
        Types::Vector(v) => {
            if v.len() >= 1 {
                v[0].clone()
            } else {
                vec_to_vector(vec![])
            }
        }
        Types::List(l) => {
            if l.len() >= 1 {
                l[0].clone()
            } else {
                vec_to_list(vec![])
            }
        }
        Types::Hash(h) => match h.iter().next() {
            Some((key, value)) => vec_to_vector(vec![Types::String(key.clone()), value.clone()]),
            None => vec_to_vector(vec![]),
        },
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
        Types::Vector(v) => {
            if v.len() >= 1 {
                vec_to_vector(v[1..].to_vec())
            } else {
                vec_to_vector(vec![])
            }
        }
        Types::List(l) => {
            if l.len() >= 1 {
                vec_to_list(l[1..].to_vec())
            } else {
                vec_to_list(vec![])
            }
        }
        Types::Hash(h) => match h.iter().next() {
            Some((skip_key, _)) => {
                let mut iter = h.iter();
                let mut matrix = vec![];

                while let Some((key, value)) = iter.next() {
                    if key != skip_key {
                        matrix.push(vec_to_vector(vec![
                            Types::String(key.clone()),
                            value.clone(),
                        ]));
                    }
                }

                vec_to_vector(matrix)
            }
            None => vec_to_vector(vec![]),
        },
        a => panic!("Cannot get the tail of {:?}", a),
    };
}

pub fn into(args: &VArgs) -> Types {
    match args[0] {
        Types::Hash(ref h) => match args[1].clone() {
            Types::Vector(ref matrices) => {
                let mut acc: Vec<Types> = vec![];

                for (key, value) in (**h).iter() {
                    acc.push(Types::String(key.to_string()));
                    acc.push(value.clone());
                }

                for pair in matrices.iter() {
                    match pair {
                        Types::Vector(ref v) => {
                            for t in v.iter() {
                                acc.push(t.clone())
                            }
                        }
                        _ => panic!("unexpected type"),
                    }
                }

                vec_to_hash(acc)
            }
            _ => panic!("can only use vectors for args"),
        },
        _ => panic!("Into only supports hashmaps for the moment"),
    }
}

pub fn assoc(args: &VArgs) -> Types {
    match args[0] {
        Types::Hash(ref h) => {
            let mut acc: Vec<Types> = vec![];

            for (key, value) in (**h).iter() {
                acc.push(Types::String(key.to_string()));
                acc.push(value.clone());
            }

            for t in args[1..].iter() {
                acc.push(t.clone())
            }

            vec_to_hash(acc)
        }
        _ => panic!("Into only supports hashmaps for the moment"),
    }
}

pub fn list(args: &VArgs) -> Types {
    vec_to_list(args.clone())
}
