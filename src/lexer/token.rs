use std::error::Error;
use std::fmt::{self, Write};
use std::str;

tokens! {
    Token, TokenType {
        OpenParen,
        CloseParen,
        Ident(String),
        Int(i64),
        Float(f64),
        String(String),
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct VoidError;

impl<T: Error> From<T> for VoidError {
    fn from(_: T) -> Self {
        VoidError
    }
}

impl TokenType {
    pub fn parse(&self, bytes: &[u8]) -> Result<Token, VoidError> {
        let str = unsafe { str::from_utf8_unchecked(bytes) };
        match self {
            &TokenType::OpenParen => if str.is_empty() {
                Ok(Token::OpenParen)
            } else {
                Err(VoidError)
            },
            &TokenType::CloseParen => if str.is_empty() {
                Ok(Token::CloseParen)
            } else {
                Err(VoidError)
            },
            &TokenType::Ident => Ok(Token::Ident(str.to_owned())),
            &TokenType::Int => Ok(Token::Int(str.parse()?)),
            &TokenType::Float => Ok(Token::Float(str.parse()?)),
            &TokenType::String => Ok(Token::String(str[1..].to_owned())),
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Token::OpenParen => f.write_char('('),
            &Token::CloseParen => f.write_char(')'),
            &Token::Ident(ref s) => f.write_str(s),
            &Token::Int(ref n) => write!(f, "{}", n),
            &Token::Float(ref n) => write!(f, "{}", n),
            &Token::String(ref s) => write!(f, "{:?}", s),
        }
    }
}
