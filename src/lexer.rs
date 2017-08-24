use std::ascii::AsciiExt;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::io;
use std::iter;
use std::ops::{Index, IndexMut};

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum TokenTemplate<T: TokenPayload> {
    OpenParen(()),
    CloseParen(()),
    Ident(T::String),
    Int(T::Int),
    Float(T::Float),
    Bool(T::Bool),
    String(T::String),
}

pub trait TokenPayload {
    type String;
    type Int;
    type Float;
    type Bool;
}

#[derive(Clone, PartialEq, Debug)]
pub struct Concrete;

impl TokenPayload for Concrete {
    type String = String;
    type Int = i64;
    type Float = f64;
    type Bool = bool;
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Tag;

impl TokenPayload for Tag {
    type String = ();
    type Int = ();
    type Float = ();
    type Bool = ();
}

pub type Token = TokenTemplate<Concrete>;
pub type TokenType = TokenTemplate<Tag>;

#[derive(Clone, Debug)]
pub struct SyntaxError<'a> {
    file_name: &'a str,
    line: u32,
    message: String,
}

impl<'a> fmt::Display for SyntaxError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SyntaxError ({}:{}): {}", self.file_name, self.line, self.message)
    }
}

impl<'a> Error for SyntaxError<'a> {
    fn description(&self) -> &str {
        "SyntaxError"
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}

macro_rules! syntax_error {
    ($file_name:expr, $lexer:ident, $($arg:tt)*) => {
        return Err(SyntaxError {
            file_name: $file_name,
            line: $lexer.line,
            message: format!($($arg)*),
        })
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum LexerState {
    Ready,
    Comment,
    Ident,
    Int,
    Float,
    Bool,
    String,
}

#[derive(Clone, Debug)]
pub struct Lexer {
    line: u32,
    state: LexerState,
    current: Vec<u8>,
}

#[derive(Clone, Copy, Debug)]
struct TableTrans {
    output: Option<TokenType>,
    next_state: LexerState,
}

impl TableTrans {
    fn output(next: LexerState, output: TokenType) -> TableTrans {
        TableTrans {
            output: Some(output),
            next_state: next,
        }
    }

    fn empty(next: LexerState) -> TableTrans {
        TableTrans {
            output: None,
            next_state: next,
        }
    }
}

type TableResult = Result<TableTrans, &'static str>;

macro_rules! table_trans {
    (
        $c:ident,
        $( $from:ident => { $($chunk:tt)* } )*
    ) => {
        {
            let mut table = HashMap::new();

            $(table_trans! {
                BRANCH [table, $from, $c, new_table]
                CONDS []
                QUEUE [$($chunk)*]
            })*

            table
        }
    };

    (
        BRANCH [$table:ident, $from:ident, $c:ident, $new_table:ident]
        CONDS [$($conds:tt)*]
        QUEUE []
    ) => {
        {
            let empty_table = [Err("Invalid character"); 256];
            let $new_table = $table.entry(LexerState::$from).or_insert(empty_table);
            for i in 0..256 {
                let $c = i as u8;
                $($conds)*
            }
        }
    };

    (
        BRANCH [$table:ident, $from:ident, $c:ident, $new_table:ident]
        CONDS [$($conds:tt)*]
        QUEUE [
            $cond:expr => $next:ident,
            $($tail:tt)*
        ]
    ) => {
        table_trans! {
            BRANCH [$table, $from, $c, $new_table]
            CONDS [
                $($conds)*
                if $cond {
                    $new_table[$c as usize] = Ok(TableTrans::empty(LexerState::$next));
                }
            ]
            QUEUE [$($tail)*]
        }
    };

    (
        BRANCH [$table:ident, $from:ident, $c:ident, $new_table:ident]
        CONDS [$($conds:tt)*]
        QUEUE [
            $cond:expr => ($next:ident, $out:ident),
            $($tail:tt)*
        ]
    ) => {
        table_trans! {
            BRANCH [$table, $from, $c, $new_table]
            CONDS [
                $($conds)*
                if $cond {
                    $new_table[$c as usize] = Ok(TableTrans::output(
                        LexerState::$next,
                        TokenTemplate::$out(())
                    ));
                }
            ]
            QUEUE [$($tail)*]
        }
    };
}

lazy_static! {
    static ref LEXER_TABLE: HashMap<LexerState, [TableResult; 256]> = {
        fn is_delimiter(c: u8) -> bool {
            match c {
                b'(' | b')' | b';' | b'"' | b'\'' | b'|' | b'[' | b']' | b'{' | b'}' => true,
                _ => c.is_ascii_whitespace(),
            }
        }

        table_trans! {
            c,
            Ready => {
                c.is_ascii_whitespace() => Ready,
                !is_delimiter(c) && (c != b'#' && c != b',') => Ident,
                c == b';' => Comment,
            }
            Ident => {
                is_delimiter(c) => (Ready, Ident),
            }
        }
    };
}

impl Lexer {
    pub fn new() -> Self {
        Lexer {
            line: 0,
            state: LexerState::Ready,
            current: Vec::new(),
        }
    }

    pub fn push_char<'a>(file_name: &'a str, c: u8) -> Result<Option<Token>, SyntaxError<'a>> {
        Ok(None)
    }
}
