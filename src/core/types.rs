use crate::core::env::{env_bind, Env};
use std::fmt;
use std::rc::Rc;

#[derive(Clone)]
pub enum Types {
    Integer(isize),
    Word(String),
    List(Rc<Vec<Types>>),
    Vector(Rc<Vec<Types>>),
    String(String),
    Float(f64),
    Bool(bool),
    Nil,
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

    pub fn inspect(&self) -> String {
        return match *self {
            Types::Func(f) => format!("<#func {:?}>", f),
            Types::Integer(i) => format!("{}", i),
            Types::String(ref s) => format!("{}", s),
            Types::Float(f) => format!("{}", f),
            Types::Vector(ref v) => format!("[{:?}]", v),
            Types::List(ref l) => format!("({:?})", l),
            Types::Word(ref w) => format!("<#def {:?}>", w),
            Types::Bool(b) => format!("{}", b),
            Types::Nil => format!("nil"),
            Types::DefFunc { ref params, .. } => format!("<#anonfunc {:?}>", params),
        };
    }
}

impl fmt::Debug for Types {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.inspect())
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
