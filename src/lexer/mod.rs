mod error;
#[macro_use] mod macros;
mod table;
mod token;

use std::str;

pub use self::error::*;
pub use self::token::*;
use self::table::{LEXER_TABLE, TableTrans};

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

impl LexerState {
    fn accum(&self) -> bool {
        match *self {
            LexerState::Ready | LexerState::Comment => false,
            _ => true,
        }
    }
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

    pub fn push_char(&mut self, c: u8) -> Result<Option<Token>> {
        if c == b'\n' {
            self.line += 1;
        }

        match LEXER_TABLE[&self.state][c as usize] {
            Ok(TableTrans {output, next_state}) => {
                let result = if let Some(output_type) = output {
                    let r = output_type.parse(&self.current).expect("Invalid lexer table");
                    self.current.clear();
                    Some(r)
                } else {
                    None
                };

                println!("Current: {}", unsafe {str::from_utf8_unchecked(&self.current)});
                println!("{:?} + '{}' => {:?}", self.state, (c as char).escape_default(), next_state);
                self.state = next_state;
                if next_state.accum() {
                    self.current.push(c);
                }

                Ok(result)
            }
            Err(_) => Err(Error {
                file_name: self.file_name.to_owned(),
                line: self.line,
                kind: ErrorKind::InvalidCharacter(c),
            })
        }
    }
}

// impl<'a> StreamMap<u8, Token> for Lexer<'a> {
//     fn produce(&mut self, u8) -> Vec<Token> {
//     }
// }
