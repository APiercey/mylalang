use crate::core::types::Types;

pub fn inspect(t: &Types) -> Types {
    let inspection = t.inspect();

    println!("{}", inspection);

    return t.clone();
}
