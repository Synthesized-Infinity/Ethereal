#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

pub mod ast;
pub mod evaluation;
pub mod lexer;
pub mod parser;
pub mod std_library;

use std::env;
use std::fs;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 1 && args[1].as_str() == "run" {
        let filename = &args[2].split('.').collect::<Vec<_>>();
        if filename[filename.len() - 1] != "etrl" {
            println!("File must be .etrl");
            return;
        }
        let content = fs::read_to_string(&args[2]).expect("Could not read file.");

        ethereal::interpert(content.as_str());
    }

}