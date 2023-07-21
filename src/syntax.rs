use crate::*;

lalrpop_mod!(pub _syntax);

#[derive(Debug)]
pub enum Ops {
    Add,
    Sub,
    Mul,
    Div,
    Neg,
}

#[derive(Debug)]
pub enum Expr {
    Var(String),
    Bin(Box<Expr>, Ops, Box<Expr>),
    Uni(Ops, Box<Expr>),
    Call(String, Vec<Expr>),
}

#[derive(Debug)]
pub enum Stmt {
    Let(String, Expr),
    Func(String, Vec<String>, Vec<Stmt>),
    Return(Box<Expr>),
    Extern(String, Vec<String>),
}
