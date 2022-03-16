extern crate ethereal_lang;

use ethereal_lang::ast::Program;
use ethereal_lang::evaluation::globals::new_globals;
use ethereal_lang::evaluation::store::Store;
use ethereal_lang::evaluation::object::Object;
use ethereal_lang::evaluation::Eval;
use ethereal_lang::lexer::Lexer;
use ethereal_lang::parser::Parser;
use std::cell::RefCell;
use std::ffi::{CStr, CString};
use std::mem;
use std::os::raw::{c_char, c_void};
use std::rc::Rc;

fn main() {}

extern "C" {
    fn print(input_ptr: *mut c_char);
}

pub fn internal_print(msg: &str) {
    unsafe {
        print(string_to_ptr(msg.to_string()));
    }
}

fn string_to_ptr(s: String) -> *mut c_char {
    CString::new(s).unwrap().into_raw()
}

fn parse(input: &str) -> Result<Program, String> {
    let mut parser = Parser::new(Lexer::new(input.to_string()));
    let program = parser.parse_program();
    let errors = parser.errors;

    if !errors.is_empty() {
        let msg = errors
            .into_iter()
            .map(|e| format!("{}\n", e))
            .collect::<String>();

        return Err(msg);
    }

    Ok(program)
}

#[no_mangle]
pub fn alloc(size: usize) -> *mut c_void {
    let mut buf = Vec::with_capacity(size);
    let ptr = buf.as_mut_ptr();
    mem::forget(buf);
    ptr as *mut c_void
}

/// # Safety
/// 
/// This is unsafe because it is the caller's responsibility to ensure that the
/// pointer is valid for the duration of the call.
#[no_mangle]
pub unsafe fn dealloc(ptr: *mut c_void, size: usize) {
    let _buf = Vec::from_raw_parts(ptr, 0, size);
}

/// # Safety
///
/// This function is unsafe because it is the caller's responsibility to ensure 
/// that the pointer is valid.
#[no_mangle]
pub unsafe fn eval(input_ptr: *mut c_char) -> *mut c_char {
    let input = CStr::from_ptr(input_ptr).to_string_lossy().into_owned();
    let program = match parse(&input) {
        Ok(program) => program,
        Err(msg) => return string_to_ptr(msg),
    };

    let env = Store::from(new_globals());

    let mut evaluation = Eval::new(Rc::new(RefCell::new(env)));
    let evaluated = evaluation.eval(program).unwrap_or(Object::Null);
    let output = format!("{}", evaluated);

    string_to_ptr(output)
}
