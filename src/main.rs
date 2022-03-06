#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

pub mod ast;
pub mod evaluation;
pub mod lexer;
pub mod parser;
pub mod std_lib;

use evaluation::{store::Store, object::Object, Eval};
use lexer::Lexer;
use parser::Parser;
use std::{cell::RefCell, env, fs, rc::Rc};


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 1 && args[1].as_str() == "run" {
        let filename = &args[2].split('.').collect::<Vec<_>>();
        if filename[filename.len() - 1] != "etrl" {
            println!("File must be .etrl");
            return;
        }
        let content = fs::read_to_string(&args[2]).expect("Could not read file.");

        let store = Store::new();
        let mut evaluator = Eval {
            store: Rc::new(RefCell::new(store)),
        };
        let lexer = Lexer::new(content);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        if !parser.errors.is_empty() {
            for e in parser.errors.iter() {
                println!("\t{}", e);
            }
            return;
        }
        let res = evaluator.eval(program);

        if let Some(o) = res {
            match o {
                Object::Null => (),
                _ => println!("{}", o),
            }
        }
        return;
    }

}