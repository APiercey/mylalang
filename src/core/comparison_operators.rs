use crate::core::types::Types;

pub fn eq(left: &Types, right: &Types) -> Types {
    // #TODO What if it's a Word?
    match (left, right) {
        (Types::Integer(a), Types::Integer(b)) => Types::Bool(a == b),
        (Types::String(a), Types::String(b)) => Types::Bool(a == b),
        (Types::Float(a), Types::Float(b)) => Types::Bool(a == b),
        (Types::Nil, Types::Nil) => Types::Bool(true),
        (Types::Vector(a), Types::Vector(b)) => {
            if a.len() == b.len() {
                let mut is_equal = Types::Bool(true);

                for i in 1..=a.len() {
                    match eq(&a[i - 1], &b[i - 1]) {
                        Types::Bool(false) => {
                            is_equal = Types::Bool(false);
                            break;
                        }
                        _ => continue,
                    }
                }

                is_equal
            } else {
                Types::Bool(false)
            }
        }
        (a, b) => panic!("Cannot compare {}", format!("{:?}{:?}", a, b)),
    }
}

pub fn gt(left: &Types, right: &Types) -> Types {
    match (left, right) {
        (Types::Integer(a), Types::Integer(b)) => Types::Bool(a > b),
        (Types::String(a), Types::String(b)) => Types::Bool(a > b),
        (Types::Float(a), Types::Float(b)) => Types::Bool(a > b),
        (Types::Nil, Types::Nil) => Types::Bool(true),
        (_, Types::Nil) => Types::Bool(true),
        (Types::Nil, _) => Types::Bool(false),
        (a, b) => panic!("Cannot compare {}", format!("{:?}{:?}", a, b)),
    }
}

pub fn lt(left: &Types, right: &Types) -> Types {
    match (left, right) {
        (Types::Integer(a), Types::Integer(b)) => Types::Bool(a < b),
        (Types::String(a), Types::String(b)) => Types::Bool(a < b),
        (Types::Float(a), Types::Float(b)) => Types::Bool(a < b),
        (Types::Nil, Types::Nil) => Types::Bool(true),
        (_, Types::Nil) => Types::Bool(false),
        (Types::Nil, _) => Types::Bool(true),
        (a, b) => panic!("Cannot compare {}", format!("{:?}{:?}", a, b)),
    }
}

pub fn lt_or_eq(left: &Types, right: &Types) -> Types {
    match (left, right) {
        (Types::Integer(a), Types::Integer(b)) => Types::Bool(a <= b),
        (Types::String(a), Types::String(b)) => Types::Bool(a <= b),
        (Types::Float(a), Types::Float(b)) => Types::Bool(a <= b),
        (Types::Nil, Types::Nil) => Types::Bool(true),
        (_, Types::Nil) => Types::Bool(false),
        (Types::Nil, _) => Types::Bool(true),
        (a, b) => panic!("Cannot compare {}", format!("{:?}{:?}", a, b)),
    }
}

pub fn gt_or_eq(left: &Types, right: &Types) -> Types {
    match (left, right) {
        (Types::Integer(a), Types::Integer(b)) => Types::Bool(a >= b),
        (Types::String(a), Types::String(b)) => Types::Bool(a >= b),
        (Types::Float(a), Types::Float(b)) => Types::Bool(a >= b),
        (Types::Nil, Types::Nil) => Types::Bool(true),
        (_, Types::Nil) => Types::Bool(true),
        (Types::Nil, _) => Types::Bool(false),
        (_, _) => panic!("Cannot compare"),
    }
}

pub fn empty(item: &Types) -> Types {
    return match item {
        Types::String(s) => Types::Bool(s.as_str().len() == 0),
        Types::Vector(v) => Types::Bool(v.len() == 0),
        Types::List(l) => Types::Bool(l.len() == 0),
        a => panic!("Cannot check if {:?} is empty", a),
    };
}
