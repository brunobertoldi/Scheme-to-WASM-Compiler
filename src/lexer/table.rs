use std::ascii::AsciiExt;
use std::collections::HashMap;

use lexer::{LexerState, TokenTemplate, TokenType};

#[derive(Clone, Copy, Debug)]
pub struct TableTrans {
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
                    $($conds)*
                }
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
                c.is_ascii_whitespace() => Ready,
                c == b'(' => (Ready, OpenParen),
                c == b')' => (Ready, CloseParen),
                c == b';' => Comment,
                c == b'+' || c == b'-' => Sign,
                c.is_ascii_digit() => Int,
                c == b'"' => String,
                !is_delimiter(c) && c != b',' => Ident,
            }
            Comment => {
                c == b'\n' => Ready,
                true => Comment,
            }
            Ident => {
                is_delimiter(c) => (Ready, Ident),
                true => Ident,
            }
            Sign => {
                is_delimiter(c) => (Ready, Ident),
                c.is_ascii_digit() => Int,
                true => Ident,
            }
            Int => {
                is_delimiter(c) => (Ready, Int),
                c == b'.' => Float,
                !c.is_ascii_digit() => Ident,
            }
            Float => {
                is_delimiter(c) => (Ready, Float),
                !c.is_ascii_digit() => Ident,
            }
            String => {
                c == b'"' => (Ready, String),
                c == b'\\' => StringEscape,
            }
            StringEscape => {
                true => String,
            }
        }
    };
}
