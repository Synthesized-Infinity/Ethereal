use std::{format, collections::HashMap, str::FromStr};
use reqwest::{header::{HeaderMap, self}};

use crate::evaluation::object::Object;

use super::Res;

pub fn add_globals() -> Res {
    let mut globals = HashMap::new();
    globals.insert("request".to_string(), Object::Inbuilt(request));
    Res { globals, raw: None }
}

pub fn request(args: Vec<Object>) -> Object {
    if args.len() < 2 || args.len() > 3 {
        return Object::Error(format!(
            "Wrong number of arguments. Got {}. Expected 2 or 3.",
            args.len()
        ));
    }
    let method = match &args[0] {
        Object::String(s) => s,
        o => return Object::Error(format!("First argument must be a string. Got {}", o)),
    };
    let url = match &args[1] {
        Object::String(s) => s,
        o => return Object::Error(format!("Second argument must be a string. Got {}", o)),
    };
    let headers = match args.get(2) {
        Some(Object::Object(h)) =>{
            let mut headers = HeaderMap::new();
            for (k, v) in h.iter() {
                match (k, v) {
                    (Object::String(k), Object::String(v)) => {
                        let key = header::HeaderName::from_str(k).unwrap();
                        headers.insert(key, v.clone().parse().unwrap());
                    },
                    _ => {
                        return Object::Error(format!("Headers must be a map of strings. Got {}", args[2]));
                    }
                }
            }
            headers
        },
        Some(o) => return Object::Error(format!("Third argument must be an Object. Got {}", o)),
        None => HeaderMap::new(),
    };

    let client = reqwest::blocking::Client::new();

    let response = match method.as_str() {
        "GET" => client.get(url).headers(headers).send(),
        "POST" => client.post(url).headers(headers).send(),
        "PUT" => client.put(url).headers(headers).send(),
        "DELETE" => client.delete(url).headers(headers).send(),
        _ => return Object::Error(format!("Unsupported HTTP method {}", method)),
    };

    match response {
        Ok(res) => {
            let mut headers = HashMap::new();
            for (k, v) in res.headers().iter() {
                headers.insert(Object::String(k.as_str().to_string()), Object::String(v.to_str().unwrap().to_string()));
            }
            let status = res.status();
            let body = res.text().unwrap();
            let status_code = status.as_u16();
            let status_text = status.canonical_reason().unwrap_or("");
            let status_line = format!("{} {}", status_code, status_text);
            let mut result = HashMap::new();
            result.insert(Object::String("status".to_string()), Object::String(status_line));
            result.insert(Object::String("headers".to_string()), Object::Object(headers));
            result.insert(Object::String("body".to_string()), Object::String(body.to_string()));
            Object::Object(result)
        },
        Err(e) => Object::Error(format!("{}", e)),
    }


}