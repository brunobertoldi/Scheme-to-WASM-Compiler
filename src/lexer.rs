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
    fn output(output: TokenType, next: LexerState) -> TableTrans {
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

struct CharTable<T>([T; 256]);

impl<T> Index<u8> for CharTable<T> {
    type Output = T;

    fn index(&self, index: u8) -> &T {
        &self.0[index as usize]
    }
}

impl<T> IndexMut<u8> for CharTable<T> {
    fn index_mut(&mut self, index: u8) -> &mut T {
        &mut self.0[index as usize]
    }
}

fn char_class<P>(pred: P) -> Vec<u8>
    where P: Fn(char) -> bool {
    (0..256).filter(|c| pred((*c as u8) as char)).collect()
}

fn char_compl<P>(pred: P) -> Vec<u8>
    where P: Fn(char) -> bool {
    char_class(|c| !pred(c))
}

fn is_delimiter(c: char) -> bool {
    match c {
        '(' | ')' | ';' | '"' | '\'' | '|' | '[' | ']' | '{' | '}' => true,
        _ => c.is_whitespace(),
    }
}

macro_rules! table_trans {
    (
        $table:ident, $from:ident, $c:ident,
        $($chunk:tt)*
    ) => {
        table_trans! {
            $table, $from, $c,
            CONDS []
            QUEUE [$($chunk)*]
        }
    };

    (
        $table:ident, $from:ident, $c:ident
        CONDS [$($conds:tt)*]
        QUEUE []
    ) => {
        let empty_table = CharTable([Err("Invalid character"); 256]);
        let new_table = $table.entry(LexerState::$from).or_insert(empty_table);
        for i in 0..256 {
            let $c = (i as u8) as char;
            $($conds)*
        }
    };

    (
        $table:ident, $from:ident, $c:ident,
        CONDS [$($conds:tt)*]
        QUEUE [
            $cond:expr => $next:ident,
            $($tail:tt)*
        ]
    ) => {
        table_trans! {
            $table, $from, $c,
            CONDS [
                $($conds)*
                if $cond { new_table[c] = Ok(TableTrans::empty(LexerState::$next)); }
            ]
            QUEUE [$($tail)*]
        }
    };

    (
        $table:ident, $from:ident, $c:ident,
        CONDS [$($conds:tt)*]
        QUEUE [
            $cond:expr => ($next:ident, $out:ident),
            $($tail:tt)*
        ]
    ) => {
        table_trans! {
            $table, $from, $c,
            CONDS [
                $($conds)*
                if $cond {
                    new_table[c] = Ok(TableTrans::output(
                        LexerState::$next,
                        TokenTemplate<Tag>::$out(())
                    ));
                }
            ]
            QUEUE [$($tail)*]
        }
    };
}

lazy_static! {
    static ref LEXER_TABLE: HashMap<LexerState, CharTable<TableResult>> = {
        let mut table = HashMap::new();

        table_trans! { table, Ready, c, }

        // table_trans! {
        //     table, Ready, c,
        //     char::is_whitespace(c) => Ready,
        //     !is_delimiter(c) && (c != '#' && c != ',') => Ident,
        //     c == ';' => Comment,
        // }

        // table_trans! {
        //     table, Ident, c,
        //     is_delimiter(c) => (Ready, Ident(())),
        // }
        
        table
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
