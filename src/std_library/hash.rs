use std::{collections::HashMap};
use crypto::md5::Md5;
use crypto::sha1::Sha1;
use crypto::sha2;
use crypto::sha3::Sha3;
use crypto::whirlpool::Whirlpool;
use crypto::digest::Digest;

use crate::evaluation::object::Object;

use super::Res;

/// Adds the standard library to the global environment.
pub fn add_globals() -> Res {
    let mut globals = HashMap::new();
    globals.insert(String::from("hasher"), Object::Inbuilt(hasher));
    Res {
        globals,
        raw: None,
    }
}

pub fn hasher(args: Vec<Object>) -> Object {

    if args.len() != 2 {
        return Object::Error(format!(
            "Wrong number of arguments. Got {}. Expected mode and string",
            args.len()
        ));
    }

    match &args[0] {
        Object::String(s) => {
            let algo: &str = &s;
            match algo {
                "md5" => {
                    let mut hasher = Md5::new();
                    hasher.input_str(&args[1].to_string());
                    let result = hasher.result_str();
                    return Object::String(result);
                },
                "sha1" => {
                    let mut hasher = Sha1::new();
                    hasher.input_str(&args[1].to_string());
                    let result = hasher.result_str();
                    return Object::String(result);
                },
                "sha256" => {
                    let mut hasher = sha2::Sha256::new();
                    hasher.input_str(&args[1].to_string());
                    let result = hasher.result_str();
                    return Object::String(result);
                },
                "sha512" => {
                    let mut hasher = sha2::Sha512::new();
                    hasher.input_str(&args[1].to_string());
                    let result = hasher.result_str();
                    return Object::String(result);
                },
                "sha3_256" => {
                    let mut hasher = Sha3::sha3_256();
                    hasher.input_str(&args[1].to_string());
                    let result = hasher.result_str();
                    return Object::String(result);
                },
                "sha3_512" => {
                    let mut hasher = Sha3::sha3_512();
                    hasher.input_str(&args[1].to_string());
                    let result = hasher.result_str();
                    return Object::String(result);
                },
                "whirlpool" => {
                    let mut hasher = Whirlpool::new();
                    hasher.input_str(&args[1].to_string());
                    let result = hasher.result_str();
                    return Object::String(result);
                },
                _ => Object::Error("Algorithm not supported".to_string())
            }
        }
        o => Object::Error(format!("First argument must be a string. Got {}", o)),
    }
}