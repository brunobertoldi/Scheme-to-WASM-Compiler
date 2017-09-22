use std::error::Error;
use std::fmt::{self, Write};
use std::str;

tokens! {
    Token, TokenType {
        OpenParen: b"",
        CloseParen: b"",
        Quote: b"quote",
        Lambda: b"lambda",
        If: b"if",
        Ident(String),
        Bool(bool),
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

pub trait TokenFromBytes: Sized {
    fn from_bytes(bytes: &[u8]) -> Result<Self, VoidError>;
}

impl TokenFromBytes for String {
    fn from_bytes(bytes: &[u8]) -> Result<Self, VoidError> {
        Ok(unsafe { String::from_utf8_unchecked(bytes.to_owned()) })
    }
}

impl TokenFromBytes for bool {
    fn from_bytes(bytes: &[u8]) -> Result<Self, VoidError> {
        if bytes == b"t" {
            Ok(true)
        } else if bytes == b"f" {
            Ok(false)
        } else {
            Err(VoidError)
        }
    }
}

impl TokenFromBytes for i64 {
    fn from_bytes(bytes: &[u8]) -> Result<Self, VoidError> {
        let s = unsafe { str::from_utf8_unchecked(bytes) };
        Ok(s.parse()?)
    }
}

impl TokenFromBytes for f64 {
    fn from_bytes(bytes: &[u8]) -> Result<Self, VoidError> {
        let s = unsafe { str::from_utf8_unchecked(bytes) };
        Ok(s.parse()?)
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Token::OpenParen => f.write_char('('),
            Token::CloseParen => f.write_char(')'),
            Token::Quote => f.write_char('\''),
            Token::Lambda => f.write_str("lambda"),
            Token::If => f.write_str("if"),
            Token::Ident(ref s) => f.write_str(s),
            Token::Bool(true) => f.write_str("#t"),
            Token::Bool(false) => f.write_str("#f"),
            Token::Int(ref n) => write!(f, "{}", n),
            Token::Float(ref n) => write!(f, "{}", n),
            Token::String(ref s) => write!(f, "{:?}", s),
        }
    }
}
