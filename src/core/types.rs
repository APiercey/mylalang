use crate::core::env::{env_bind, Env};
use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum Types {
    Integer(isize),
    Word(String),
    List(Rc<Vec<Types>>),
    Vector(Rc<Vec<Types>>),
    String(String),
    Float(f64),
    Bool(bool),
    Func(fn(VArgs) -> Types),
    DefFunc {
        eval: fn(env: Env, ast: Types) -> Types,
        env: Env,
        params: Rc<Types>,
        body: Rc<Types>,
    },
}

pub type ErrMessage = String;

impl Types {
    pub fn apply(&self, args: VArgs) -> Types {
        return match *self {
            Types::Func(f) => f(args),
            Types::DefFunc {
                ref params,
                ref env,
                ref body,
                eval,
                ..
            } => {
                let p = &**params;
                let b = &**body;
                let fun_env = env_bind(&env, p.clone(), args);

                return eval(fun_env, b.clone());
            }
            _ => panic!("No apply"),
        };
    }

    pub fn eq(&self, other: &Self) -> Types {
        match (self, other) {
            (Types::Integer(a), Types::Integer(b)) => Types::Bool(a == b),
            (Types::String(a), Types::String(b)) => Types::Bool(a == b),
            (Types::Float(a), Types::Float(b)) => Types::Bool(a == b),
            (_, _) => Types::Bool(false),
        }
    }

    pub fn gt(&self, other: &Self) -> Types {
        match (self, other) {
            (Types::Integer(a), Types::Integer(b)) => Types::Bool(a > b),
            (Types::String(a), Types::String(b)) => Types::Bool(a > b),
            (Types::Float(a), Types::Float(b)) => Types::Bool(a > b),
            (_, _) => Types::Bool(false),
        }
    }

    pub fn lt(&self, other: &Self) -> Types {
        match (self, other) {
            (Types::Integer(a), Types::Integer(b)) => Types::Bool(a < b),
            (Types::String(a), Types::String(b)) => Types::Bool(a < b),
            (Types::Float(a), Types::Float(b)) => Types::Bool(a < b),
            (_, _) => Types::Bool(false),
        }
    }

    pub fn lt_or_eq(&self, other: &Self) -> Types {
        match (self, other) {
            (Types::Integer(a), Types::Integer(b)) => Types::Bool(a <= b),
            (Types::String(a), Types::String(b)) => Types::Bool(a <= b),
            (Types::Float(a), Types::Float(b)) => Types::Bool(a <= b),
            (_, _) => Types::Bool(false),
        }
    }

    pub fn gt_or_eq(&self, other: &Self) -> Types {
        match (self, other) {
            (Types::Integer(a), Types::Integer(b)) => Types::Bool(a >= b),
            (Types::String(a), Types::String(b)) => Types::Bool(a >= b),
            (Types::Float(a), Types::Float(b)) => Types::Bool(a >= b),
            (_, _) => Types::Bool(false),
        }
    }

    pub fn inspect(&self) {
        return match *self {
            Types::Func(f) => println!("<#func {:?}>", f),
            Types::Integer(i) => println!("{}", i),
            Types::String(ref s) => println!("{}", s),
            Types::Float(f) => println!("{}", f),
            Types::Vector(ref v) => println!("<#vec {:?}", v),
            Types::List(ref l) => println!("<#list {:?}", l),
            Types::Word(ref w) => println!("<#def {:?}>", w),
            Types::Bool(b) => println!("{}", b),
            Types::DefFunc { ref params, .. } => println!("<#anonfunc {:?}>", params),
        };
    }
}

pub type TypeResult = Result<Types, ErrMessage>;

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
