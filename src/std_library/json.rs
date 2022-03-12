use std::{collections::HashMap};

use serde_json::Value;

use crate::evaluation::object::Object;

use super::Res;

/// Adds the standard library to the global environment.
pub fn add_globals() -> Res {
    let mut globals = HashMap::new();
    globals.insert(String::from("parse_json"), Object::Inbuilt(parse_json));
    Res {
        globals,
        raw:None
    }
}

pub fn parse_json(args: Vec<Object>) -> Object {
    match &args[0] {
        Object::String(s) => {
            let json_str = s.as_str();
            let json: Result<Value, serde_json::Error> = serde_json::from_str(json_str);
            match json {
                Ok(json_obj) => {
                    match json_obj {
                        Value::Object(obj) => {
                            let mut hash = HashMap::new();
                            for (key, value) in obj {
                                let key = Object::String(key.to_string());
                                let value = match value {
                                    Value::Null => Object::Null,
                                    Value::Bool(_) => Object::Bool(value.as_bool().unwrap()),
                                    Value::Number(_) => Object::Number(value.as_f64().unwrap()),
                                    Value::String(_) => Object::String(value.as_str().unwrap().to_string()),
                                    Value::Array(_) => {
                                        let mut array = Vec::new();
                                        for value in value.as_array().unwrap() {
                                            array.push(parse_json(vec![Object::String(value.to_string())]))
                                        }
                                        Object::Array(array)
                                    }
                                    Value::Object(_) => parse_json(vec![Object::String(value.to_string())]),
                                };
                                hash.insert(key, value);
                            }
                            return Object::Object(hash);
                        }
                        Value::Array(arr) => {
                            return Object::Array(arr.iter().map(|value| {
                                match value {
                                    Value::Null => Object::Null,
                                    Value::Bool(_) => Object::Bool(value.as_bool().unwrap()),
                                    Value::Number(_) => Object::Number(value.as_f64().unwrap()),
                                    Value::String(_) => Object::String(value.as_str().unwrap().to_string()),
                                    Value::Array(_) => {
                                        let mut array = Vec::new();
                                        for value in value.as_array().unwrap() {
                                            array.push(parse_json(vec![Object::String(value.to_string())]))
                                        }
                                        Object::Array(array)
                                    }
                                    Value::Object(_) => parse_json(vec![Object::String(value.to_string())]),
                                }
                            }).collect());
                        }
                        _ => {
                            return Object::Error(format!("Expected an object. Got {}", json_obj));
                        }
                    }
                }
                Err(e) => {
                    return Object::Error(format!("{}", e));
                }
            }
        }
        _ => Object::Error(format!("Expected a string. Got {}", args[0]))
    }
}
