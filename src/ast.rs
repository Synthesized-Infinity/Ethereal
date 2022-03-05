use std::fmt::{self, Display, Formatter};

#[derive(PartialEq, Clone, Debug)]
pub struct Ident(pub String);

#[derive(PartialEq, Clone, Debug)]
pub enum Prefix {
    Plus,
    Minus,
    Exclamation
}

#[derive(PartialEq, Clone, Debug)]
pub enum Infix {
    Plus,
    Minus,
    Times,
    Divide,
    Modulo,
    Equals,
    NotEquals,
    LessThan,
    GreaterThan,
    LessThanEqual,
    GreaterThanEqual
}

#[derive(PartialEq, Clone, Debug)]
pub enum Literal {
    Int(i32),
    String(String),
    Boolean(bool),
    Array(Vec<Expr>)
}

#[derive(PartialEq, Clone, Debug)]
pub enum Statement {
    Let(Ident, Expr),
    Return(Expr),
    Expression(Expr),
    Import(Ident),
}

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum Precedence {
    Lowest,
    Equals,
    LessGreater,
    Sum,
    Product,
    Prefix,
    Call,
    Index
}

pub type BlockStatement = Vec<Statement>;

#[derive(PartialEq, Clone, Debug)]
pub enum Expr {
    Literal(Literal),
    Ident(Ident),
    Prefix(Prefix, Box<Expr>),
    Infix(Infix, Box<Expr>, Box<Expr>),
    If {
        cond: Box<Expr>,
        then: Box<BlockStatement>,
        else_: Option<BlockStatement>
    },

    Fun {
        params: Vec<Ident>,
        body: BlockStatement
    },

    Call {
        function: Box<Expr>,
        args: Vec<Expr>
    },

    Index {
        array: Box<Expr>,
        index: Box<Expr>
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Program {
    statements: Vec<Statement>
}

// Implement the Display trait for all the types we have

impl Display for Prefix {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            Prefix::Plus => write!(f, "+"),
            Prefix::Minus => write!(f, "-"),
            Prefix::Exclamation => write!(f, "!")
        }
    }
}

impl Display for Infix {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            Infix::Plus => write!(f, "+"),
            Infix::Minus => write!(f, "-"),
            Infix::Times => write!(f, "*"),
            Infix::Divide => write!(f, "/"),
            Infix::Modulo => write!(f, "%"),
            Infix::Equals => write!(f, "=="),
            Infix::NotEquals => write!(f, "!="),
            Infix::LessThan => write!(f, "<"),
            Infix::GreaterThan => write!(f, ">"),
            Infix::LessThanEqual => write!(f, "<="),
            Infix::GreaterThanEqual => write!(f, ">=")
        }
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}




