use crate::{
    string::StringSlice,
    tokenizer::token::{Keyword, Number, Symbol, TokenKind},
};

use super::{types::Type, IdentPath};

#[derive(Debug, Clone, PartialEq)]
pub struct Expr<'a> {
    pub slice: StringSlice<'a>,
    pub kind: ExprKind<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExprKind<'a> {
    Invoke {
        value: Box<Expr<'a>>,
        params: Vec<Expr<'a>>,
    },
    Index {
        value: Box<Expr<'a>>,
        index: Box<Expr<'a>>,
    },
    Field {
        value: Box<Expr<'a>>,
        access: AccessKind,
        field: &'a str,
        generics: Option<GenericsInstance<'a>>,
    },
    BinOp {
        lhs: Box<Expr<'a>>,
        op: BinOp,
        rhs: Box<Expr<'a>>,
    },
    Cast {
        value: Box<Expr<'a>>,
        ty: Type<'a>,
    },
    UnaryOp {
        op: UnaryOp,
        value: Box<Expr<'a>>,
    },
    Variable {
        path: IdentPath<'a>,
        generics: Option<GenericsInstance<'a>>,
    },
    SizeofType(Type<'a>),
    SizeofValue(Box<Expr<'a>>),
    Number(Number),
    String(String),
    Char(char),
    Bool(bool),
    This,
    Default,
    Nullptr,
    Discard,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Initializer<'a> {
    pub slice: StringSlice<'a>,
    pub kind: InitializerKind<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum InitializerKind<'a> {
    Single(Expr<'a>),
    List(Vec<InitializerValue<'a>>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct InitializerValue<'a> {
    pub slice: StringSlice<'a>,
    pub name: &'a str,
    pub value: Expr<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GenericsInstance<'a> {
    pub slice: StringSlice<'a>,
    pub params: Vec<Type<'a>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessKind {
    /// .
    Value,
    /// ?.
    ValueCoalesce,
    /// !.
    ValueCascade,
    /// ->
    Reference,
    /// ?->
    ReferenceCoalesce,
    /// !->
    ReferenceCascade,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOp {
    Add,
    Sub,
    BoolNot,
    BitNot,
    Reference, // ref
    Pointer,   // &
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
    pub fn try_parse(kind: TokenKind) -> Option<Self> {
        let kind = match kind {
            TokenKind::Symbol(Symbol::Dot) => Self::Value,
            TokenKind::Symbol(Symbol::ValueCoalesce) => Self::ValueCascade,
            TokenKind::Symbol(Symbol::ValueCascade) => Self::ValueCascade,
            TokenKind::Symbol(Symbol::SmallArrow) => Self::Reference,
            TokenKind::Symbol(Symbol::ReferenceCascade) => Self::ReferenceCascade,
            TokenKind::Symbol(Symbol::ReferenceCoalesce) => Self::ReferenceCoalesce,
            _ => return None,
        };

        return Some(kind);
    }
}

impl UnaryOp {
    pub fn try_parse<'a>(kind: TokenKind) -> Option<Self> {
        let op = match kind {
            TokenKind::Symbol(Symbol::Add) => Self::Add,
            TokenKind::Symbol(Symbol::Sub) => Self::Sub,
            TokenKind::Symbol(Symbol::BoolNot) => Self::BoolNot,
            TokenKind::Symbol(Symbol::BitNot) => Self::BitNot,
            TokenKind::Symbol(Symbol::BitAnd) => Self::Pointer,
            TokenKind::Symbol(Symbol::Mul) => Self::Deref,
            TokenKind::Keyword(Keyword::Ref) => Self::Reference,
            _ => return None,
        };

        return Some(op);
    }
}

impl BinOp {
    pub fn binding(self) -> (usize, usize) {
        match self {
            BinOp::Range | BinOp::RangeFromTo | BinOp::RangeTo | BinOp::RangeFrom => (21, 22),

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

            BinOp::BoolAnd => (5, 6),
            BinOp::BoolOr => (3, 4),
            BinOp::BoolXor => (1, 2),
        }
    }

    pub fn try_parse<'a>(kind: TokenKind<'a>) -> Option<Self> {
        let op = match kind {
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

            TokenKind::Symbol(Symbol::Shl) => Self::Shl,
            TokenKind::Symbol(Symbol::Shr) => Self::Shr,

            _ => return None,
        };

        return Some(op);
    }
}
