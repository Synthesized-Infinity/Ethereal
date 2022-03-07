use std::collections::HashMap;

use crate::evaluation::object::Object;

use super::Res;

/// Adds the standard library to the global environment.
pub fn add_globals() -> Res {
    let mut globals = HashMap::new();
    globals.insert(String::from("replace"), Object::Inbuilt(replace));
    Res {
        globals,
        raw: None,
    }
}

pub fn replace(args: Vec<Object>) -> Object {
    if args.len() != 3 {
        return Object::Error(format!(
            "Wrong number of arguments. Got {}. Expected 3.",
            args.len()
        ));
    }

    match &args[0] {
        Object::String(s) => {
            let mut s = s.clone();
            s = s.replace(&args[1].to_string(), &args[2].to_string());
            Object::String(s)
        }
        o => Object::Error(format!("First argument must be a string. Got {}", o)),
    }
}