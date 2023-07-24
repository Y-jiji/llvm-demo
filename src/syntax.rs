use crate::*;

lalrpop_mod!(_syntax);
pub use _syntax::{ItemsParser, ItemParser};

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
    F32(f32),
    F64(f64),
    I32(i32),
    I64(i64),
    Tup(Vec<Expr>),
    Elem(Box<Expr>, usize),
    Call(String, Vec<Expr>),
}

#[derive(Debug)]
pub enum Type {
    F32, F64, I32, I64,
    Tup(Vec<Type>),
}

#[derive(Debug)]
pub enum Stmt {
    Let(String, Expr),
    Func(String, Vec<(String, Type)>, Vec<Stmt>),
    LetTup(Vec<String>, Expr),
    Return(Box<Expr>),
}

#[derive(Debug)]
pub enum Item {
    Stmt(Stmt),
    Expr(Expr),
}
