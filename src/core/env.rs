use crate::core::types::{vec_to_list, TypeResult, Types};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
pub struct EnvStruct {
    data: RefCell<HashMap<String, Types>>,
    outer: Option<Env>,
}

pub type Env = Rc<EnvStruct>;

pub fn new_env(outer: Option<Env>) -> Env {
    Rc::new(EnvStruct {
        data: RefCell::new(HashMap::new()),
        outer: outer,
    })
}

pub fn set_env(env: &Env, key: &str, value: Types) {
    env.data.borrow_mut().insert(key.to_string(), value);
}

pub fn env_find(env: &Env, key: &str) -> Option<Env> {
    match (env.data.borrow().contains_key(key), env.outer.clone()) {
        (true, _) => Some(env.clone()),
        (false, Some(cloned_outer)) => env_find(&cloned_outer, key),
        _ => None,
    }
}

pub fn env_is_defined(env: &Env, key: &str) -> bool {
    match env_find(&env, &key) {
        Some(_) => true,
        None => false,
    }
}

pub fn env_bind(env: &Env, bindings: Types, values: Vec<Types>) -> Env {
    let new_env = new_env(Some(env.clone()));

    match bindings {
        Types::Vector(v) => {
            let mut iter = v.iter().enumerate();

            while let Some((i, b)) = iter.next() {
                match b {
                    Types::Word(w) => match w.as_str() {
                        "&" => {
                            match iter.next() {
                                Some((_, Types::Word(rest))) => {
                                    set_env(&new_env, &rest, vec_to_list(values[i..].to_vec()))
                                }
                                _ => panic!("hmm"),
                            }
                            break;
                        }
                        _ => set_env(&new_env, &w, values[i].clone()),
                    },
                    _ => panic!("Currently unexpected"),
                }
            }
        }
        _ => panic!("Bindings are expected to be wrapped in a vector"),
    }

    return new_env;
}

pub fn get_env(env: &Env, key: &Types) -> TypeResult {
    match key {
        Types::Word(ref w) => match env_find(env, w) {
            Some(e) => Ok(e
                .data
                .borrow()
                .get(w)
                .ok_or("No definition found".to_string())?
                .clone()),
            None => panic!("{:?} does not exist within this scope", w),
        },
        _ => panic!("yo chill"),
    }
}
