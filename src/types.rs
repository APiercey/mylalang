use std::rc::Rc;

#[derive(Debug)]
pub enum Types {
    Integer(isize),
    Float(f64),
    Word(String),
    List(Rc<Vec<Types>>),
    String(String),
    Func(fn(VArgs) -> Types),
}

pub type VArgs = Vec<Types>;

pub fn vec_to_list(list: Vec<Types>) -> Types {
    return Types::List(Rc::new(list));
}
