#[derive(PartialEq, Debug)]
pub enum Token {
    OpenParen,
    CloseParen,
    Ident(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
}

pub struct SyntaxError {
    message: String,
    line: u32,
    column: u32,
}
