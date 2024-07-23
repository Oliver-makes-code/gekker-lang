use parse::error::ParserError;

use crate::{
    string::StringSlice,
    tokenizer::{
        token::{Keyword, Number, Symbol, TokenKind},
        Tokenizer,
    },
};

pub mod parse;

#[derive(Debug, Clone, PartialEq)]
pub struct Node<'a> {
    pub slice: StringSlice<'a>,
    pub kind: NodeKind<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum NodeKind<'a> {
    Expr(Expr<'a>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Expr<'a> {
    pub slice: StringSlice<'a>,
    pub kind: ExprKind<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExprKind<'a> {
    Index(Box<Expr<'a>>, Box<Expr<'a>>),
    Field(Box<Expr<'a>>, AccessKind, &'a str),
    BinOp(Box<Expr<'a>>, BinOp, Box<Expr<'a>>),
    UnaryOp(UnaryOp, Box<Expr<'a>>),
    Identifier(Vec<&'a str>),
    Number(Number),
    String(String),
    Char(char),
    Bool(bool),
    Discard,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessKind {
    /// .
    Value,
    /// ->
    Reference
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOp {
    Add,
    Sub,
    BoolNot,
    BitNot,
    SafeRef,   // ref
    UnsafeRef, // &
    Deref,     // *
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinOp {
    Mul,
    Div,
    Rem,

    Add,
    Sub,

    Shr,
    Shl,

    Equal,
    NotEqual,
    GreaterEqual,
    LessEqual,
    Greater,
    Less,

    BitAnd,
    BitOr,
    BitXor,

    Range,
    RangeFromTo,
    RangeTo,
    RangeFrom,

    BoolAnd,
    BoolOr,
    BoolXor,
}

impl AccessKind {
    pub fn try_parse<'a>(tokenizer: &mut Tokenizer<'a>) -> Result<Option<(StringSlice<'a>, Self)>, ParserError<'a>> {
        let peek = tokenizer.peek()?;

        let kind = match peek.kind {
            TokenKind::Symbol(Symbol::Dot) => Self::Value,
            TokenKind::Symbol(Symbol::SmallArrow) => Self::Reference,
            _ => return Ok(None)
        };

        return Ok(Some((peek.slice.unwrap(), kind)));
    }
}

impl UnaryOp {
    pub fn try_parse<'a>(
        tokenizer: &mut Tokenizer<'a>,
    ) -> Result<Option<(StringSlice<'a>, Self)>, ParserError<'a>> {
        let peek = tokenizer.peek()?;

        let op = match peek.kind {
            TokenKind::Symbol(Symbol::Add) => Self::Add,
            TokenKind::Symbol(Symbol::Sub) => Self::Sub,
            TokenKind::Symbol(Symbol::BoolNot) => Self::BoolNot,
            TokenKind::Symbol(Symbol::BitNot) => Self::BitNot,
            TokenKind::Symbol(Symbol::BitAnd) => Self::UnsafeRef,
            TokenKind::Symbol(Symbol::Mul) => Self::Deref,
            TokenKind::Keyword(Keyword::Ref) => Self::SafeRef,
            _ => return Ok(None),
        };

        return Ok(Some((peek.slice.unwrap(), op)));
    }
}

impl BinOp {
    pub fn binding(self) -> (usize, usize) {
        match self {
            BinOp::Mul | BinOp::Div | BinOp::Rem => (19, 20),

            BinOp::Add | BinOp::Sub => (17, 18),

            BinOp::Equal
            | BinOp::NotEqual
            | BinOp::GreaterEqual
            | BinOp::LessEqual
            | BinOp::Greater
            | BinOp::Less => (15, 16),

            BinOp::Shr | BinOp::Shl => (13, 14),

            BinOp::BitAnd => (11, 12),
            BinOp::BitXor => (9, 10),
            BinOp::BitOr => (7, 8),

            BinOp::Range | BinOp::RangeFromTo | BinOp::RangeTo | BinOp::RangeFrom => todo!(),

            BinOp::BoolAnd => (5, 6),
            BinOp::BoolOr => (3, 4),
            BinOp::BoolXor => (1, 2),
        }
    }

    pub fn try_parse<'a>(
        tokenizer: &mut Tokenizer<'a>,
    ) -> Result<Option<(StringSlice<'a>, Self)>, ParserError<'a>> {
        let peek = tokenizer.peek()?;

        let op = match peek.kind {
            TokenKind::Symbol(Symbol::Add) => Self::Add,
            TokenKind::Symbol(Symbol::Sub) => Self::Sub,
            TokenKind::Symbol(Symbol::Mul) => Self::Mul,
            TokenKind::Symbol(Symbol::Div) => Self::Div,
            TokenKind::Symbol(Symbol::Rem) => Self::Rem,

            TokenKind::Symbol(Symbol::Range) => Self::Range,
            TokenKind::Symbol(Symbol::RangeFromTo) => Self::RangeFromTo,
            TokenKind::Symbol(Symbol::RangeFrom) => Self::RangeFrom,
            TokenKind::Symbol(Symbol::RangeTo) => Self::RangeTo,

            TokenKind::Symbol(Symbol::Equal) => Self::Equal,
            TokenKind::Symbol(Symbol::NotEqual) => Self::NotEqual,
            TokenKind::Symbol(Symbol::GreaterEqual) => Self::GreaterEqual,
            TokenKind::Symbol(Symbol::LessEqual) => Self::LessEqual,
            TokenKind::Symbol(Symbol::Greater) => Self::Greater,
            TokenKind::Symbol(Symbol::Less) => Self::Less,

            TokenKind::Symbol(Symbol::BitAnd) => Self::BitAnd,
            TokenKind::Symbol(Symbol::BitOr) => Self::BitOr,
            TokenKind::Symbol(Symbol::BitXor) => Self::BitXor,

            TokenKind::Symbol(Symbol::BoolAnd) => Self::BoolAnd,
            TokenKind::Symbol(Symbol::BoolOr) => Self::BoolOr,
            TokenKind::Symbol(Symbol::BoolXor) => Self::BoolXor,

            _ => return Ok(None),
        };

        return Ok(Some((peek.slice.unwrap(), op)));
    }
}