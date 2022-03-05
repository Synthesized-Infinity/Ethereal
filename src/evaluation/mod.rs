pub mod object;
pub mod store;
pub mod lib;
pub mod globals;

use crate::ast::*;
use globals::new_globals;
use store::Store;
use object::Object;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use self::lib::{load_etrl};

pub struct Eval {
    pub store: Rc<RefCell<Store>>,
}

impl Eval {
    pub fn new(store: Rc<RefCell<Store>>) -> Self {
        Eval { store }
    }

    fn is_truthy(&mut self, object: Object) -> bool {
        !matches!(object, Object::Null | Object::Bool(false))
    }

    fn is_error(&mut self, object: &Object) -> bool {
        matches!(object, Object::Error(_))
    }

    pub fn eval(&mut self, program: Program) -> Option<Object> {
        let mut result = None;

        for statement in program.statements {
            match self.eval_statement(statement) {
                Some(Object::Error(val)) => return Some(Object::Error(val)),
                Some(Object::Return(val)) => return Some(*val),
                e => result = e,
            }
        }

        result
    }

    fn eval_statement(&mut self, statement: Statement) -> Option<Object> {
        match statement {
            Statement::Expression(e) => self.eval_expr(e),
            Statement::Return(e) => {
                let val = match self.eval_expr(e) {
                    Some(v) => v,
                    None => return None,
                };

                Some(Object::Return(Box::new(val)))
            }
            Statement::Set(i, v) => {
                let val = match self.eval_expr(v) {
                    Some(value) => value,
                    None => return None,
                };
                if self.is_error(&val) {
                    Some(val)
                } else {
                    let Ident(name) = i;
                    self.store.borrow_mut().set(name, val);
                    None
                }
            }
            Statement::Update(i, v) => {
                let Ident(name) = i;
                let val = match self.eval_expr(v) {
                    Some(value) => value,
                    None => return None,
                };
                if self.is_error(&val) {
                    Some(val)
                } else {
                    let mut store = self.store.borrow_mut();
                    match store.get(&name) {
                        Some(_) => {
                            store.update(name, val);
                            None
                        }
                        None => Some(Object::Error(format!("identifier not found: {}", name))),
                    }
                }
            }
            Statement::Include(i) => {
                let lib = i;
                self.extend_global_store(lib)
            }
        }
    }

    fn eval_block_statement(&mut self, statements: BlockStatement) -> Option<Object> {
        let mut result = None;

        for statement in statements {
            match self.eval_statement(statement) {
                Some(Object::Return(e)) => return Some(Object::Return(e)),
                Some(Object::Error(e)) => return Some(Object::Error(e)),
                e => result = e,
            }
        }

        result
    }

    fn eval_expr(&mut self, expr: Expr) -> Option<Object> {
        match expr {
            Expr::Ident(ident) => Some(self.eval_ident(ident)),
            Expr::Literal(lit) => Some(self.eval_literal(lit)),
            Expr::Prefix(prefix, right) => self
                .eval_expr(*right)
                .map(|expr| self.eval_prefix_expr(prefix, expr)),
            Expr::Infix(infix, left, right) => {
                let left_expr = self.eval_expr(*left);
                let right_expr = self.eval_expr(*right);
                match left_expr.clone() {
                    Some(l) => {
                        if self.is_error(&l) {
                            return left_expr;
                        }
                        if self.is_error(&right_expr.clone().unwrap()) {
                            return right_expr;
                        }
                        right_expr.map(|r| self.eval_infix_expr(infix, left_expr.unwrap(), r))
                    }
                    _ => None,
                }
            }
            Expr::If {
                cond: condition,
                then: consequence,
                else_: alternative,
            } => {
                let cond_expr = match self.eval_expr(*condition) {
                    Some(e) => e,
                    None => return None,
                };

                if self.is_truthy(cond_expr) {
                    self.eval_block_statement(*consequence)
                } else if let Some(a) = alternative {
                    self.eval_block_statement(a)
                } else {
                    None
                }
            }
            Expr::Fun { params, body } => Some(Object::Fn(params, body, self.store.clone())),
            Expr::Call { function, args } => Some(self.eval_call_expr(*function, args)),
            Expr::Index { array, index } => {
                let arr = self.eval_expr(*array);
                let i = self.eval_expr(*index);
                if arr.is_some() && i.is_some() {
                    Some(self.eval_index_expr(arr.unwrap(), i.unwrap()))
                } else {
                    None
                }
            }
        }
    }

    fn eval_prefix_expr(&mut self, prefix: Prefix, expr: Object) -> Object {
        if self.is_error(&expr) {
            return expr;
        }
        match prefix {
            Prefix::Exclamation => self.eval_not_prefix_expr(expr),
            Prefix::Minus => self.eval_minus_prefix_expr(expr),
            Prefix::Plus => self.eval_plus_prefix_expr(expr),
        }
    }

    fn eval_not_prefix_expr(&mut self, expr: Object) -> Object {
        match expr {
            Object::Bool(true) => Object::Bool(false),
            Object::Bool(false) => Object::Bool(true),
            Object::Null => Object::Bool(true),
            _ => Object::Bool(false),
        }
    }

    fn eval_minus_prefix_expr(&mut self, expr: Object) -> Object {
        match expr {
            Object::Int(i) => Object::Int(-i),
            _ => Object::Error(format!("unknown operator: -{}", expr)),
        }
    }

    fn eval_plus_prefix_expr(&mut self, expr: Object) -> Object {
        match expr {
            Object::Int(i) => Object::Int(i),
            _ => Object::Error(format!("unknown operator: {}", expr)),
        }
    }

    fn eval_infix_expr(&mut self, infix: Infix, left: Object, right: Object) -> Object {
        match left {
            Object::Int(left_expr) => {
                if let Object::Int(right_expr) = right {
                    self.eval_int_infix_expr(infix, left_expr, right_expr)
                } else {
                    Object::Error(format!("type mismatch: {} {} {}", left, infix, right))
                }
            }
            Object::String(left_expr) => {
                if let Object::String(right_expr) = right {
                    self.eval_string_infix_expr(infix, left_expr, right_expr)
                } else {
                    Object::Error(format!("type mismatch: {} {} {}", left_expr, infix, right))
                }
            }
            _ => Object::Error(format!("unknown operator: {} {} {}", left, infix, right)),
        }
    }

    fn eval_string_infix_expr(&mut self, infix: Infix, left: String, right: String) -> Object {
        match infix {
            Infix::Plus => Object::String(format!("{}{}", left, right)),
            _ => Object::Error(format!("unknown operator: {} {} {}", left, infix, right)),
        }
    }

    fn eval_int_infix_expr(&mut self, infix: Infix, left: i32, right: i32) -> Object {
        match infix {
            Infix::Plus => Object::Int(left + right),
            Infix::Minus => Object::Int(left - right),
            Infix::Times => Object::Int(left * right),
            Infix::Divide => Object::Int(left / right),
            Infix::Modulo => Object::Int(left % right),
            Infix::LessThan => Object::Bool(left < right),
            Infix::LessThanEqual => Object::Bool(left <= right),
            Infix::GreaterThan => Object::Bool(left > right),
            Infix::GreaterThanEqual => Object::Bool(left >= right),
            Infix::Equals => Object::Bool(left == right),
            Infix::NotEquals => Object::Bool(left != right),
        }
    }

    fn eval_call_expr(&mut self, function: Expr, args: Vec<Expr>) -> Object {
        let args = args
            .iter()
            .map(|a| self.eval_expr(a.clone()).unwrap_or(Object::Null))
            .collect::<Vec<_>>();

        let x= self.apply_function(function, args);
        x
    }

    fn eval_index_expr(&mut self, left: Object, index: Object) -> Object {
        match left {
            Object::Array(ref arr) => {
                if let Object::Int(i) = index {
                    self.eval_array_index_expr(arr.clone(), i)
                } else {
                    Object::Error(format!("index operator not supported: {}", left))
                }
            }
            Object::Object(ref hash) => match index {
                Object::Int(_) | Object::Bool(_) | Object::String(_) => match hash.get(&index) {
                    Some(o) => o.clone(),
                    None => Object::Null,
                },
                Object::Error(_) => index,
                _ => Object::Error(format!("unsable as hash key: {}", index)),
            },
            _ => Object::Error(format!("unknown operator: {} {}", left, index)),
        }
    }

    fn eval_array_index_expr(&mut self, array: Vec<Object>, index: i32) -> Object {
        let max = array.len() as i32;
        if index > max {
            return Object::Null;
        }

        if index < 0 {
            match array.get((array.len() as i32 + index) as usize) {
                Some(o) => return o.clone(),
                None => return Object::Null,
            }
        }
        match array.get(index as usize) {
            Some(o) => o.clone(),
            None => Object::Null,
        }
    }

    fn apply_function(&mut self, function: Expr, args: Vec<Object>) -> Object {
        let (params, body, store) = match self.eval_expr(function) {
            Some(Object::Fn(params, body, store)) => (params, body, store),
            Some(Object::Inbuilt(func)) => return func(args),
            Some(o) => return Object::Error(format!("function not found: {}", o)),
            None => return Object::Null,
        };

        if params.len() != args.len() {
            return Object::Error(format!(
                "expected arguments: {}\ngiven arguments: {}",
                params.len(),
                args.len()
            ));
        };

        let current_store = Rc::clone(&self.store);
        let extended_store = self.extended_function_store(params, store, args);
        self.store = Rc::new(RefCell::new(extended_store));
        let evaluated = self.eval_block_statement(body);
        self.store = current_store;
        //println!("{:#?}", self.store);
        self.unwrap_return_value(evaluated)
    }

    fn extended_function_store(
        &mut self,
        params: Vec<Ident>,
        store: Rc<RefCell<Store>>,
        args: Vec<Object>,
    ) -> Store {
        let mut scope_store = Store::new_enclosed(store);

        for (ident, arg) in params.iter().zip(args.iter()) {
            let Ident(name) = ident.clone();
            scope_store.set(name, arg.to_owned());
        }

        scope_store
    }

    fn extend_global_store(&mut self, lib: String) -> Option<Object> {
        let lib_store = match load_etrl(lib.clone()) {
            Some(e) => e,
            None => return Some(Object::Error(format!("Could not load lib: {}", lib))),
        };
        let mut new_store = Store::new_enclosed(self.store.clone());
        for (k, v) in lib_store {
            new_store.set(k, v);
        }
        self.store = Rc::new(RefCell::new(new_store));
        None
    }

    fn unwrap_return_value(&mut self, obj: Option<Object>) -> Object {
        match obj {
            Some(Object::Return(o)) => *o,
            Some(o) => o,
            None => Object::Null,
        }
    }

    fn eval_ident(&mut self, ident: Ident) -> Object {
        let Ident(i) = ident;
        let builtins =  new_globals();
        if builtins.contains_key(&i) {
            return builtins.get(&i).unwrap().clone();
        };
        match self.store.borrow_mut().get(&i) {
            Some(i) => i,
            None => Object::Error(format!("identifier not found: {}", i)),
        }
    }

    fn eval_literal(&mut self, lit: Literal) -> Object {
        match lit {
            Literal::String(s) => Object::String(s),
            Literal::Int(i) => Object::Int(i),
            Literal::Boolean(b) => Object::Bool(b),
            Literal::Array(a) => Object::Array(
                a.iter()
                    .map(|e| self.eval_expr(e.clone()).unwrap_or(Object::Null))
                    .collect::<Vec<_>>(),
            ),
            Literal::Object(h) => self.eval_object_literal(h),
        }
    }

    fn eval_object_literal(&mut self, h: Vec<(Expr, Expr)>) -> Object {
        let mut hash = HashMap::new();

        for (k, v) in h {
            let key = self.eval_expr(k).unwrap_or(Object::Null);
            if self.is_error(&key) {
                return key;
            }

            let val = self.eval_expr(v).unwrap_or(Object::Null);
            if self.is_error(&val) {
                return val;
            }

            hash.insert(key, val);
        }
        Object::Object(hash)
    }
}