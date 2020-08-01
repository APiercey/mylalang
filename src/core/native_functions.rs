use crate::core::types::{vec_to_vector, Types};

pub fn inspect(t: Types) -> Types {
    t.inspect();

    return t;
}

pub fn add(first: Types, second: Types) -> Types {
    match (first, second) {
        (Types::Integer(a), Types::Integer(b)) => Types::Integer(a + b),
        (Types::Float(a), Types::Float(b)) => Types::Float(a + b),
        (Types::Float(a), Types::Integer(b)) => Types::Float(a + (b as f64)),
        (Types::Integer(a), Types::Float(b)) => Types::Float((a as f64) + b),
        (Types::String(a), Types::String(b)) => Types::String(format!("{}{}", a, b)),
        (Types::Vector(a), Types::Vector(b)) => vec_to_vector([&a[..], &b[..]].concat()),
        (a, b) => panic!("cannot add {:?} from {:?}", a, b),
    }
}

pub fn subtract(first: Types, second: Types) -> Types {
    match (first, second) {
        (Types::Integer(a), Types::Integer(b)) => Types::Integer(a - b),
        (Types::Float(a), Types::Integer(b)) => Types::Float(a - (b as f64)),
        (Types::Integer(a), Types::Float(b)) => Types::Float((a as f64) - b),
        (Types::Float(a), Types::Float(b)) => Types::Float(a - b),
        (Types::Vector(_), Types::Vector(_)) => panic!("vector subtraction is not implemented :("),
        (a, b) => panic!("cannot subtract {:?} from {:?}", a, b),
    }
}

pub fn divide(first: Types, second: Types) -> Types {
    match (first, second) {
        (Types::Integer(a), Types::Integer(b)) if a == 0 || b == 0 => panic!("Division by zero"),
        (Types::Float(a), Types::Float(b)) => Types::Float(a / b),
        (Types::Integer(a), Types::Integer(b)) => Types::Float((a as f64) / (b as f64)),
        (Types::Integer(a), Types::Float(b)) => Types::Float((a as f64) / b),
        (Types::Float(a), Types::Integer(b)) => Types::Float(a / (b as f64)),
        (a, b) => panic!("cannot divide {:?} from {:?}", a, b),
    }
}

pub fn multiply(first: Types, second: Types) -> Types {
    match (first, second) {
        (Types::Integer(a), Types::Integer(b)) => Types::Integer(a * b),
        (Types::Integer(a), Types::Float(b)) => Types::Float((a as f64) * b),
        (Types::Float(a), Types::Integer(b)) => Types::Float(a * (b as f64)),
        (Types::Float(a), Types::Float(b)) => Types::Float(a * b),
        (a, b) => panic!("cannot multiply {:?} from {:?}", a, b),
    }
}
