use super::{store::Store, object::*, Eval};
use crate::{lexer::Lexer, parser::Parser, std_library::*};
use std::{cell::RefCell, collections::HashMap, fs, rc::Rc};

/// Function to load an external file or a standard library onto the environment.\
/// The file is loaded as a string, and the string is parsed into an AST.
/// The AST is then evaluated.
/// The result is then added to the environment.
/// # Arguments
/// * `lib` - The name of the library to load.
/// # Returns
/// `HashMap<String, Object>` - The environment with the library loaded.
/// 
pub fn load_etrl(lib: String) -> Option<HashMap<String, Object>> {
    // Checks if the library is a standard library.
    if lib.starts_with("std:") {
        // Loads the standard library.
        // The standard library is a HashMap of names to objects.
        return get_std_lib(lib);
    }
    let filename =format!("./{}.etrl", lib);
    // File is read as a string.
    let file = fs::read_to_string(filename).expect("Lib not found.");
    let mut parser = Parser::new(Lexer::new(file));
    let program = parser.parse_program();
    if !parser.errors.is_empty() {
        for e in parser.errors.iter() {
            println!("\t{}", e);
        }
        return None;
    };
    // Evaluates the program.
    let mut eval = Eval::new(Rc::new(RefCell::new(Store::new())));
    eval.eval(program);
    let store = (&*eval.store.borrow()).to_owned().store;
    let mut final_env = HashMap::new();
    // Returns the environment with the library loaded.
    for (k, v) in store.iter() {
        final_env.insert(k.clone(), v.clone());
    }
    Some(final_env)
}