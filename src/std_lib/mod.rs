use std::collections::HashMap;

use crate::evaluation::object::Object;

pub mod util;
pub mod array;

pub fn get_std_lib(lib: String) -> Option<HashMap<String, Object>> {
    match lib.as_str() {
        "std:util" => Some(util::add_globals()),
        "std:array" => Some(array::add_globals()),
        _ => None,
    }
}