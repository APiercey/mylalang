use crate::core::types::Types;

pub fn eq(left: &Types, right: &Types) -> Types {
    match (left, right) {
        (Types::Integer(a), Types::Integer(b)) => Types::Bool(a == b),
        (Types::String(a), Types::String(b)) => Types::Bool(a == b),
        (Types::Float(a), Types::Float(b)) => Types::Bool(a == b),
        (Types::Nil, Types::Nil) => Types::Bool(true),
        (_, _) => Types::Bool(false),
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
        (_, _) => Types::Bool(false),
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
        (_, _) => Types::Bool(false),
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
        (_, _) => Types::Bool(false),
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
        (_, _) => Types::Bool(false),
    }
}
