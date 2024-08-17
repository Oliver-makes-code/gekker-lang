use std::sync::Arc;

use crate::{
    string::StringSlice,
    tokenizer::token::{Keyword, Number, Symbol, TokenKind},
};

use super::{decl::FuncBody, types::Type, IdentPath};

#[derive(Debug, Clone, PartialEq)]
pub struct Expr {
    pub slice: StringSlice,
    pub kind: ExprKind,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExprKind {
    Invoke {
        value: Box<Expr>,
        params: Vec<Expr>,
    },
    Index {
        value: Box<Expr>,
        index: Box<Expr>,
    },
    Field {
        value: Box<Expr>,
        access: AccessKind,
        field: Arc<str>,
        generics: Option<GenericsInstance>,
    },
    BinOp {
        lhs: Box<Expr>,
        op: BinOp,
        rhs: Box<Expr>,
    },
    Cast {
        value: Box<Expr>,
        ty: Type,
    },
    UnaryOp {
        op: UnaryOp,
        value: Box<Expr>,
    },
    Variable {
        path: IdentPath,
        generics: Option<GenericsInstance>,
    },
    Initializer {
        path: IdentPath,
        generics: Option<GenericsInstance>,
        list: InitializerList,
    },
    AnonStructInitializer {
        list: InitializerList,
    },
    Lambda {
        params: Option<LambdaParams>,
        captures: Option<LambdaCaptures>,
        body: Box<FuncBody>,
    },
    SizeofType(Type),
    SizeofValue(Box<Expr>),
    Number(Number),
    String(Arc<str>),
    Char(char),
    Bool(bool),
    This,
    Default,
    Nullptr,
    Discard,
    Unit,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LambdaParams {
    pub slice: StringSlice,
    pub params: Vec<LambdaParam>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LambdaParam {
    pub slice: StringSlice,
    pub is_mut: bool,
    pub name: Arc<str>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LambdaCaptures {
    pub slice: StringSlice,
    pub captures: Vec<LambdaCapture>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LambdaCapture {
    pub slice: StringSlice,
    pub is_ref: bool,
    pub name: Arc<str>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InitializerList {
    pub slice: StringSlice,
    pub kind: InitializerKind,
}

#[derive(Debug, Clone, PartialEq)]
pub enum InitializerKind {
    Expr(Vec<Expr>),
    Named {
        values: Vec<NamedInitializer>,
        default: Option<DefaultedInitializer>,
    },
    Empty,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DefaultedInitializer {
    pub slice: StringSlice,
    pub value: Box<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NamedInitializer {
    pub slice: StringSlice,
    pub name: Arc<str>,
    pub value: Expr,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GenericsInstance {
    pub slice: StringSlice,
    pub params: Vec<Type>,
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

    Coalesce, // ?
    Cascade,  // !
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
    pub fn try_parse_prefix(kind: TokenKind) -> Option<Self> {
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

    pub fn try_parse_suffix(kind: TokenKind) -> Option<Self> {
        let op = match kind {
            TokenKind::Symbol(Symbol::Optional) => Self::Coalesce,
            TokenKind::Symbol(Symbol::BoolNot) => Self::Cascade,
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

    pub fn try_parse(kind: TokenKind) -> Option<Self> {
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
