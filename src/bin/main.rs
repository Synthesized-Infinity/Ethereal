
extern crate ethereal;

use std::env;
use std::fs;
use ethereal::repl;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 1 && args[1].as_str() == "run" {
        let filename = &args[2].split('.').collect::<Vec<_>>();
        if filename[filename.len() - 1] != "etrl" {
            println!("File must have the extention .etrl");
            return;
        }
        let content = fs::read_to_string(&args[2]).expect("Could not read file.");

        ethereal::interpret(content.as_str());
    } else {
    
        println!(
            "Welcome to the Ethereal REPL. Type in commands to get started.",
        );
        repl::start();
    }

}