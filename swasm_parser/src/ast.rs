#[derive(Clone, Debug, PartialEq)]
pub enum SExpr {
    Quote,
    If,
    Bool(bool),
    Number(Number),
    String(String),
    Ident(String),
    List(Vec<SExpr>),
    DottedList(Vec<SExpr>, Box<SExpr>),
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Number {
    Exact(i64),
    Inexact(f64),
}
