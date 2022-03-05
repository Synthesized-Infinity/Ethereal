use super::object::*;
use std::collections::HashMap;

pub fn new_globals() -> HashMap<String, Object> {
    let mut globals = HashMap::new();
    globals.insert(String::from("log"), Object::Inbuilt(log));
    globals
}

fn log(args: Vec<Object>) -> Object {
    if args.len() < 1 {
        return Object::Error(String::from("Wrong number of arguments"));
    } else {
        for arg in args {
            print!("{} ", arg);
        }
        println!();
    }
    Object::Null
}