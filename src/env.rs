use crate::types::{TypeError, TypeResult, Types};
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

pub fn env_bind(env: &Env, bindings: Types, values: Vec<Types>) -> Env {
    let new_env = new_env(Some(env.clone()));

    match bindings {
        Types::Vector(v) => {
            for (i, b) in v.iter().enumerate() {
                match b {
                    Types::Word(w) => set_env(&env, &w, values[i].clone()),
                    _ => panic!("Currently unexpected"),
                }
            }
        }
        _ => panic!("Not expected"),
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
                .ok_or(TypeError::ErrMessage("No definition found".to_string()))?
                .clone()),
            _ => panic!("inner chill"),
        },
        _ => panic!("yo chill"),
    }
}
