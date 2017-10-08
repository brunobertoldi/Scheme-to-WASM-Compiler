#[derive(Clone, Debug, PartialEq)]
pub struct Program(pub Vec<Form>);

#[derive(Clone, Debug, PartialEq)]
pub enum Form{
    Def(Def),
    Expr(Expr),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Def {
    Var(VarDef),
    Begin(Vec<Def>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum VarDef {
    Var(String, Body),
    Fun(Vec<String>, Option<String>, Body),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Body {
    definitions: Vec<Def>,
    expressions: Vec<Expr>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Const(Const),
    Var(String),
    Quote(Datum),
    Lambda(Args, Body),
    If(Box<Expr>, Box<Expr>, Option<Expr>),
    Set(String, Box<Expr>),
    Call(Box<Expr>, Vec<Expr>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Args {
    Single(String),
    Multi(Vec<String>, Option<String>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Const {
    Bool(bool),
    Number(Number),
    String(String),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Datum {
    Bool(bool),
    Number(Number),
    String(String),
    List(Vec<Datum>, Option<Box<Datum>>),
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Number {
    Exact(i64),
    Inexact(f64),
}
