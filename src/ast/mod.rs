use crate::{string::StringSlice, tokenizer::token::Number};

pub mod parse;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node<'a> {
    pub slice: StringSlice<'a>,
    pub kind: NodeKind<'a>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NodeKind<'a> {
    Expr(Expr<'a>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Expr<'a> {
    pub slice: StringSlice<'a>,
    pub kind: ExprKind<'a>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExprKind<'a> {
    BinOp(Box<Expr<'a>>, BinOp, Box<Expr<'a>>),
    UnaryOp(UnaryOp, Box<Expr<'a>>),
    Identifier(&'a str),
    Number(Number),
    String(String),
    Char(char),
    Bool(bool),
    Discard,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOp {
    Add,
    Sub,
    BoolNot,
    BitNot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Rem,

    Range,

    Equal,
    NotEqual,
    GreaterEqual,
    LessEqual,
    Greater,
    Less,

    BitAnd,
    BitOr,
    BitXor,

    BoolAnd,
    BoolOr,
    BoolXor,
}
