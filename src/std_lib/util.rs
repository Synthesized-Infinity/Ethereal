use std::collections::HashMap;

use crate::evaluation::object::Object;

pub fn add_globals() -> HashMap<String, Object> {
    let mut globals = HashMap::new();
    globals.insert(String::from("length"), Object::Inbuilt(length));
    globals
}
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