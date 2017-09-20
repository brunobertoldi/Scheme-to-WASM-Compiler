mod error;
#[macro_use] mod macros;
mod table;
mod token;

use std::str;

use iter::{StreamAdapter, StreamMap};

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

// impl LexerState {
//     fn consume(&self) -> bool {
//         match *self {
//             LexerState::Ready | LexerState::Comment => false,
//             _ => true,
//         }
//     }
// }

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
            Ok(TableTrans {output, next_state}) => {
                if let Some(output_type) = output {
                    let token = output_type.parse(&self.current).expect("Invalid lexer table");
                    out.push(token);
                    self.current.clear();
                };

                println!("Current: {}", unsafe {str::from_utf8_unchecked(&self.current)});
                println!("{:?} + '{}' => {:?}", self.state, (c as char).escape_default(), next_state);
                let old_state = self.state;
                self.state = next_state;
                self.current.push(c);
                if self.state != old_state {
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
        println!("Out, producting for '{}'", (c as char).escape_default());
        let mut v = Vec::new();
        match self.push_char(c, &mut v) {
            Ok(_) => v.into_iter().map(|x| Ok(x)).collect(),
            Err(e) => vec![Err(e)],
        }
    }
}
