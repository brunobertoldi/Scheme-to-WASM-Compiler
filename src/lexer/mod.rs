mod error;
mod table;

pub use self::error::*;
use self::table::LEXER_TABLE;

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

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum LexerState {
    Ready,
    Comment,
    Ident,
    Sign,
    Int,
    Float,
    String,
    StringEscape,
}

#[derive(Clone, Debug)]
pub struct Lexer {
    line: u32,
    state: LexerState,
    current: Vec<u8>,
}

impl Lexer {
    pub fn new() -> Self {
        Lexer {
            line: 0,
            state: LexerState::Ready,
            current: Vec::new(),
        }
    }

    pub fn push_char(file_name: &str, c: u8) -> Result<Option<Token>> {
        Ok(None)
    }
}
