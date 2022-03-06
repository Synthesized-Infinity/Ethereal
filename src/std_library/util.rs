use std::collections::HashMap;

use crate::evaluation::object::Object;

use super::Res;

pub fn add_globals() -> Res {
    let mut globals = HashMap::new();
    globals.insert(String::from("length"), Object::Inbuilt(length));
    return Res { globals, raw: None }
}

/// Function to get the length of an array or string.
/// # Arguments
/// * `args` - The array or string to get the length of.
/// # Returns
/// `Object` - The length of the array or string.
pub fn length(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        return Object::Error(format!(
            "Wrong number of arguments. Got {}. Expected 1.",
            args.len()
            
        ));
    }
    match &args[0] {
        Object::String(s) => Object::Int(s.len() as i32),
        Object::Array(a) => Object::Int(a.len() as i32),
        o => Object::Error(format!("Argument must be a string or array. Got {}", o)),
    }
}