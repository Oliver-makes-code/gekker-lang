use crate::{
    string::StringSlice,
    tokenizer::token::{Keyword, TokenKind},
};

use super::IdentPath;

#[derive(Debug, Clone, PartialEq)]
pub struct Type<'a> {
    pub slice: StringSlice<'a>,
    pub kind: TypeKind<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypeKind<'a> {
    Char,
    Bool,

    U8,
    I8,
    U16,
    I16,
    U32,
    I32,
    U64,
    I64,
    Usize,
    Isize,

    F32,
    F64,

    Unit,
    Never,

    Ref(RefKind, Box<Type<'a>>),

    Array(Box<Type<'a>>, usize),
    Slice(Box<Type<'a>>),

    Option(Box<Type<'a>>),
    Range(Box<Type<'a>>),

    Func(Vec<Type<'a>>, Option<Box<Type<'a>>>),

    UserDefined(IdentPath<'a>, Vec<Type<'a>>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RefKind {
    Mutable,
    Immutable,
    Pointer,
}

impl<'a> TypeKind<'a> {
    pub fn try_from_primitive(token: TokenKind) -> Option<Self> {
        return Some(match token {
            TokenKind::Keyword(Keyword::Char) => Self::Char,
            TokenKind::Keyword(Keyword::Bool) => Self::Bool,

            TokenKind::Keyword(Keyword::U8) => Self::U8,
            TokenKind::Keyword(Keyword::I8) => Self::I8,
            TokenKind::Keyword(Keyword::U16) => Self::U16,
            TokenKind::Keyword(Keyword::I16) => Self::I16,
            TokenKind::Keyword(Keyword::U32) => Self::U32,
            TokenKind::Keyword(Keyword::I32) => Self::I32,
            TokenKind::Keyword(Keyword::U64) => Self::U16,
            TokenKind::Keyword(Keyword::I64) => Self::I64,
            TokenKind::Keyword(Keyword::Usize) => Self::Usize,
            TokenKind::Keyword(Keyword::Isize) => Self::Isize,

            TokenKind::Keyword(Keyword::F32) => Self::F32,
            TokenKind::Keyword(Keyword::F64) => Self::F64,

            TokenKind::Keyword(Keyword::Unit) => Self::Unit,
            TokenKind::Keyword(Keyword::Never) => Self::Never,

            _ => return None,
        });
    }
}
