use std::fmt::{self, Write};
use std::str;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum TokenTemplate<T: TokenPayload> {
    OpenParen(()),
    CloseParen(()),
    Ident(T::String),
    Int(T::Int),
    Float(T::Float),
    String(T::String),
}

pub trait TokenPayload {
    type String;
    type Int;
    type Float;
}

#[derive(Clone, PartialEq, Debug)]
pub struct Concrete;

impl TokenPayload for Concrete {
    type String = String;
    type Int = i64;
    type Float = f64;
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Tag;

impl TokenPayload for Tag {
    type String = ();
    type Int = ();
    type Float = ();
}

pub type Token = TokenTemplate<Concrete>;
pub type TokenType = TokenTemplate<Tag>;

impl TokenTemplate<Tag> {
    pub fn parse(&self, bytes: &[u8]) -> Result<TokenTemplate<Concrete>, ()> {
        let str = unsafe { str::from_utf8_unchecked(bytes) };
        match self {
            &TokenTemplate::OpenParen(_) => if str.is_empty() {
                Ok(TokenTemplate::OpenParen(()))
            } else {
                Err(())
            },
            &TokenTemplate::CloseParen(_) => if str.is_empty() {
                Ok(TokenTemplate::CloseParen(()))
            } else {
                Err(())
            },
            &TokenTemplate::Ident(_) => Ok(TokenTemplate::Ident(str.to_owned())),
            &TokenTemplate::Int(_) => Ok(TokenTemplate::Int(str.parse().map_err(|_| ())?)),
            &TokenTemplate::Float(_) => Ok(TokenTemplate::Float(str.parse().map_err(|_| ())?)),
            &TokenTemplate::String(_) => Ok(TokenTemplate::String(str[1..].to_owned())),
        }
    }
}

impl fmt::Display for TokenTemplate<Concrete> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &TokenTemplate::OpenParen(_) => f.write_char('('),
            &TokenTemplate::CloseParen(_) => f.write_char(')'),
            &TokenTemplate::Ident(ref s) => f.write_str(s),
            &TokenTemplate::Int(ref n) => write!(f, "{}", n),
            &TokenTemplate::Float(ref n) => write!(f, "{}", n),
            &TokenTemplate::String(ref s) => write!(f, "\"{}\"", s),
        }
    }
}
