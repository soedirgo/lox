use std::fmt;

use crate::token_type::TokenType;

#[derive(Debug)]
pub enum Literal {
    None,
    Str(String),
    Num(f64),
}

pub struct Token {
    type_: TokenType,
    lexeme: String,
    literal: Literal,
    line: usize,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?} {:?} {:?} {:?}",
            self.type_, self.lexeme, self.literal, self.line
        )
    }
}

impl Token {
    pub fn new(type_: TokenType, lexeme: String, literal: Literal, line: usize) -> Self {
        Token {
            type_,
            lexeme,
            literal,
            line,
        }
    }
}
