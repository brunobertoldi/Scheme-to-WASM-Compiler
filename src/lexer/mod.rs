mod error;
#[macro_use] mod macros;
mod table;
mod token;

use std::str;

use iter::{StreamAdapter, StreamMap};

pub use self::error::*;
pub use self::token::*;
use self::table::{LEXER_TABLE, Consume, TableTrans};

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

#[derive(Debug)]
pub struct Lexer<'a> {
    file_name: &'a str,
    line: u32,
    state: LexerState,
    current: Vec<u8>,
}

impl<'a> Lexer<'a> {
    pub fn new(file_name: &'a str) -> Self {
        Lexer {
            file_name: file_name,
            line: 0,
            state: LexerState::Ready,
            current: Vec::new(),
        }
    }

    pub fn iter<I>(self, source: I) -> StreamAdapter<Self, I, Result<Token>>
        where I: Iterator<Item=u8> {
        StreamAdapter::new(self, source)
    }

    fn push_char(&mut self, c: u8, out: &mut Vec<Token>) -> Result<()> {
        if c == b'\n' {
            self.line += 1;
        }

        match LEXER_TABLE[&self.state][c as usize] {
            Ok(TableTrans {output, next_state, consume}) => {
                if consume == Consume::Append {
                    self.current.push(c);
                }

                if let Some(output_type) = output {
                    let token = output_type.parse(&self.current).expect("Invalid lexer table");
                    out.push(token);
                    self.current.clear();
                };

                self.state = next_state;
                if consume == Consume::Ungetc {
                    self.push_char(c, out)?;
                }

                Ok(())
            }
            Err(_) => Err(Error {
                file_name: self.file_name.to_owned(),
                line: self.line,
                kind: ErrorKind::InvalidCharacter(c),
            }),
        }
    }
}

impl<'a> StreamMap<u8, Result<Token>> for Lexer<'a> {
    fn produce(&mut self, c: u8) -> Vec<Result<Token>> {
        let mut v = Vec::new();
        match self.push_char(c, &mut v) {
            Ok(_) => v.into_iter().map(|x| Ok(x)).collect(),
            Err(e) => vec![Err(e)],
        }
    }
}
