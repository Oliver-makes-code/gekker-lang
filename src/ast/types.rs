use crate::{
    string::StringSlice,
    tokenizer::{
        token::{Keyword, Symbol, TokenKind},
        Tokenizer,
    },
};

use super::{decl::StructBody, parse::error::ParserError, IdentPath};

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

    This,

    Ref {
        ref_kind: RefKind,
        ty: Box<Type<'a>>,
    },

    Array {
        ty: Box<Type<'a>>,
        len: usize,
    },
    Slice(Box<Type<'a>>),

    Option(Box<Type<'a>>),
    Range(Box<Type<'a>>),

    Func {
        params: Vec<Type<'a>>,
        ret: Option<Box<Type<'a>>>,
    },

    Struct(StructBody<'a>),

    UserDefined {
        path: IdentPath<'a>,
        generics: Vec<Type<'a>>,
    },
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

            TokenKind::Keyword(Keyword::ThisType) => Self::This,

            _ => return None,
        });
    }
}

impl RefKind {
    pub fn parse<'a>(
        tokenizer: &mut Tokenizer<'a>,
    ) -> Result<Option<(StringSlice<'a>, Self)>, ParserError<'a>> {
        let peek = tokenizer.peek(0)?;

        match peek.kind {
            TokenKind::Symbol(Symbol::Mul) => {
                tokenizer.next()?;
                return Ok(Some((peek.slice, Self::Pointer)));
            }
            TokenKind::Keyword(Keyword::Ref) => {
                tokenizer.next()?;
                let start = peek.slice;
                let peek = tokenizer.peek(0)?;
                if let TokenKind::Keyword(Keyword::Mut) = peek.kind {
                    tokenizer.next()?;
                    return Ok(Some((start.merge(peek.slice), Self::Mutable)));
                }
                return Ok(Some((start, Self::Immutable)));
            }
            _ => return Ok(None),
        }
    }
}
