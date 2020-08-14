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
    Lambda {
        eval: fn(env: Env, ast: Types) -> Types,
        env: Env,
        params: Rc<Types>,
        body: Rc<Types>,
    },
    VariadicFunc {
        lambdas: Vec<Types>, // Is it possible to reference only Lambdas? e.g. Vec<Types::Lambda>
    },
}

pub type ErrMessage = String;

impl Types {
    pub fn apply(&self, args: VArgs) -> Types {
        return match *self {
            Types::Func(f) => f(args),
            Types::Lambda {
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

    pub fn arity(&self) -> usize {
        match *self {
            Types::Lambda { ref params, .. } => match &**params {
                Types::Vector(vector) => {
                    let mut iter = vector.iter();
                    let mut arity = 0;

                    while let Some(p) = iter.next() {
                        match p {
                            Types::Word(ref w) => {
                                if w.as_str() != "&" {
                                    arity += 1
                                }
                            }
                            _ => panic!("Currently expected a varialbe assignment"),
                        }
                    }
                    arity
                }
                _ => panic!("Expected a list of parameters"),
            },
            _ => panic!("No apply"),
        }
    }

    pub fn is_variadic(&self) -> bool {
        match *self {
            Types::Lambda { ref params, .. } => match &**params {
                Types::Vector(vector) => {
                    let mut iter = vector.iter();
                    let mut is_variadic = false;

                    while let Some(p) = iter.next() {
                        match p {
                            Types::Word(ref w) => {
                                if w.as_str() == "&" {
                                    is_variadic = true;
                                }
                            }
                            _ => panic!("can not compare values"),
                        }
                    }
                    is_variadic
                }
                _ => panic!("Expected a list of parameters"),
            },
            _ => panic!("No apply"),
        }
    }

    pub fn is_lambda(&self) -> bool {
        match *self {
            Types::Lambda { .. } => true,
            _ => false,
        }
    }

    // pub fn add_lambda(&self) {
    //     match *self {
    //         Types::VariadicFunc { ref mut lambdas, .. } => {
    //             lambdas.push
    //         }
    //     }

    // }

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
            Types::Lambda { ref params, .. } => format!("<#anonfunc {:?}>", params),
            Types::VariadicFunc { ref lambdas, .. } => format!("<#varfunc {:?}>", lambdas.len()),
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
