use std::{collections::HashMap};
use rand::{Rng};

use crate::evaluation::object::Object;

use super::Res;

/// Adds the standard library to the global environment.
pub fn add_globals() -> Res {
    let mut globals = HashMap::new();
    globals.insert(String::from("random"), Object::Inbuilt(random));
    globals.insert(String::from("round"), Object::Inbuilt(round));
    globals.insert(String::from("ceil"), Object::Inbuilt(ceil));
    globals.insert(String::from("floor"), Object::Inbuilt(floor));
    globals.insert(String::from("abs"), Object::Inbuilt(abs));
    globals.insert(String::from("sqrt"), Object::Inbuilt(sqrt));
    globals.insert(String::from("MAX_INT"), Object::Number(std::f64::MAX));
    globals.insert(String::from("MIN_INT"), Object::Number(std::f64::MIN));
    Res {
        globals,
        raw: None,
    }
}

pub fn random(args: Vec<Object>) -> Object {
    let min = match &args[0] {
        Object::Number(n) => *n,
        _ => 0.0,
    };

    let max = match &args[1] {
        Object::Number(n) => *n,
        _ => 0.0,
    };

    let mut rng = rand::thread_rng();
    let random_number = rng.gen_range(min..max);
    Object::Number(random_number)
}

pub fn round(args: Vec<Object>) -> Object {
    match &args[0] {
        Object::Number(n) => Object::Number(n.round()),
        _ => Object::Error(format!("Argument must be a number. Got {}", args[0])),
    }
}

pub fn floor(args: Vec<Object>) -> Object {
    match &args[0] {
        Object::Number(n) => Object::Number(n.floor()),
        _ => Object::Error(format!("Argument must be a number. Got {}", args[0])),
    }
}

pub fn ceil(args: Vec<Object>) -> Object {
    match &args[0] {
        Object::Number(n) => Object::Number(n.ceil()),
        _ => Object::Error(format!("Argument must be a number. Got {}", args[0])),
    }
}

pub fn abs(args: Vec<Object>) -> Object {
    match &args[0] {
        Object::Number(n) => Object::Number(n.abs()),
        _ => Object::Error(format!("Argument must be a number. Got {}", args[0])),
    }
}

pub fn sqrt(args: Vec<Object>) -> Object {
    match &args[0] {
        Object::Number(n) => Object::Number(n.sqrt()),
        _ => Object::Error(format!("Argument must be a number. Got {}", args[0])),
    }
}
