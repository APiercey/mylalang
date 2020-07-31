use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum Types {
    Integer(isize),
    Word(String),
    List(Rc<Vec<Types>>),
    Vector(Rc<Vec<Types>>),
    String(String),
    Float(f64),
    Func(fn(VArgs) -> Types),
}

#[derive(Debug)]
pub enum TypeError {
    ErrMessage(String),
    Type(Types),
}

impl Types {
    pub fn apply(&self, args: VArgs) -> Types {
        return match *self {
            Types::Func(f) => f(args),
            _ => panic!("No apply"),
        };
    }

    pub fn inspect(&self) {
        return match *self {
            Types::Func(f) => println!("<#func {:?}>", f),
            Types::Integer(i) => println!("{}", i),
            Types::String(ref s) => println!("{}", s),
            Types::Float(f) => println!("{}", f),
            _ => panic!("No apply"),
        };
    }
}

pub type TypeResult = Result<Types, TypeError>;

pub type VArgs = Vec<Types>;

pub fn vec_to_list(list: Vec<Types>) -> Types {
    return Types::List(Rc::new(list));
}

pub fn vec_to_vector(vector: Vec<Types>) -> Types {
    return Types::Vector(Rc::new(vector));
}

pub fn define_function(fun: fn(VArgs) -> Types) -> Types {
    return Types::Func(fun);
}
