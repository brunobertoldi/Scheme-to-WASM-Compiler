use std::ascii::AsciiExt;
use std::collections::HashMap;

use lexer::{LexerState, TokenType};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TableTrans {
    pub output: Option<TokenType>,
    pub next_state: LexerState,
    pub consume: Consume,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Consume {
    Append,
    Skip,
    Ungetc,
}

pub type TableResult = Result<TableTrans, ()>;

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
            let empty_table = [Err(()); 256];
            let $new_table = $table.entry(LexerState::$from).or_insert(empty_table);
            for i in 0..256 {
                let $c = i as u8;
                if $c.is_ascii_whitespace() || $c.is_ascii_graphic() {
                    $($conds)* {}
                }
            }
        }
    };

    (
        BRANCH [$table:ident, $from:ident, $c:ident, $new_table:ident]
        CONDS [$($conds:tt)*]
        QUEUE [
            $cond:expr => $consume:ident $next:ident,
            $($tail:tt)*
        ]
    ) => {
        table_trans! {
            BRANCH [$table, $from, $c, $new_table]
            CONDS [
                $($conds)*
                if $cond {
                    $new_table[$c as usize] = Ok(TableTrans {
                        output: None,
                        next_state: LexerState::$next,
                        consume: Consume::$consume,
                    });
                } else
            ]
            QUEUE [$($tail)*]
        }
    };

    (
        BRANCH [$table:ident, $from:ident, $c:ident, $new_table:ident]
        CONDS [$($conds:tt)*]
        QUEUE [
            $cond:expr => $consume:ident ($next:ident, $out:ident),
            $($tail:tt)*
        ]
    ) => {
        table_trans! {
            BRANCH [$table, $from, $c, $new_table]
            CONDS [
                $($conds)*
                if $cond {
                    $new_table[$c as usize] = Ok(TableTrans {
                        output: Some(TokenType::$out),
                        next_state: LexerState::$next,
                        consume: Consume::$consume,
                    });
                } else
            ]
            QUEUE [$($tail)*]
        }
    };
}

lazy_static! {
    pub static ref LEXER_TABLE: HashMap<LexerState, [TableResult; 256]> = {
        fn is_delimiter(c: u8) -> bool {
            match c {
                b'(' | b')' | b';' | b'"' | b'\'' | b'|' | b'[' | b']' | b'{' | b'}' => true,
                _ => c.is_ascii_whitespace(),
            }
        }

        table_trans! {
            c,
            Ready => {
                c.is_ascii_whitespace() => Skip Ready,
                c == b'(' => Skip (Ready, OpenParen),
                c == b')' => Skip (Ready, CloseParen),
                c == b';' => Skip Comment,
                c == b'+' || c == b'-' => Append Sign,
                c.is_ascii_digit() => Append Int,
                c == b'"' => Ungetc String,
                !is_delimiter(c) && c != b',' => Append Ident,
            }
            Comment => {
                c == b'\n' => Skip Ready,
                true => Skip Comment,
            }
            Ident => {
                is_delimiter(c) => Ungetc (Ready, Ident),
                true => Append Ident,
            }
            Sign => {
                is_delimiter(c) => Ungetc (Ready, Ident),
                c.is_ascii_digit() => Append Int,
                true => Append Ident,
            }
            Int => {
                is_delimiter(c) => Ungetc (Ready, Int),
                c == b'.' => Append Float,
                !c.is_ascii_digit() => Append Ident,
            }
            Float => {
                is_delimiter(c) => Ungetc (Ready, Float),
                !c.is_ascii_digit() => Append Ident,
            }
            String => {
                c == b'"' => Skip (Ready, String),
                c == b'\\' => Skip StringEscape,
            }
            StringEscape => {
                true => Append String,
            }
        }
    };
}
