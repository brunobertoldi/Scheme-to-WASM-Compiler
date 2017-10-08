#[derive(Clone, Debug, PartialEq)]
pub enum Node {
    Quote,
    If,
    Bool(bool),
    Number(Number),
    String(String),
    Ident(String),
    List(Vec<Node>),
    DottedList(Vec<Node>),
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Number {
    Exact(i64),
    Inexact(f64),
}
