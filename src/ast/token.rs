use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Eof,
    Illegal,

    // Literals
    Ident(String),
    Int(i32),
    String(String),
    Boolean(bool),

    // Operators
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    Percent,
    Anew,

    // Comparison
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    Equals,
    NotEquals,

    // Delimiters
    Comma,
    Colon,
    Semicolon,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,

    // Keywords
    Set,
    Func,
    If,
    Else,
    Return,
    Include

}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

