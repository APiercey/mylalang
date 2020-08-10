use crate::core::types::Types;
use std::fs;

pub fn read_from_file(file_name: &Types) -> Types {
    match file_name {
        Types::String(import_path) => {
            let contents = fs::read_to_string(import_path.clone())
                .expect(format!("Could not import {:?}", import_path.as_str()).as_str());

            Types::String(contents.to_string())
        }
        _ => panic!("Expected a path name for a string"),
    }
}
