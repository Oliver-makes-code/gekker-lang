use crate::{
    string::StringSlice,
    tokenizer::{
        token::{Keyword, TokenKind},
        Tokenizer,
    },
};

use super::{
    expr::Expr,
    parse::error::ParserError,
    statement::{Block, FunctionModifier, VariableModifier, VariableName},
    types::Type,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Decl<'a> {
    pub slice: StringSlice<'a>,
    pub is_pub: bool,
    pub kind: DeclKind<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DeclKind<'a> {
    Variable(
        VariableModifier,
        VariableName<'a>,
        Option<Type<'a>>,
        Option<Expr<'a>>,
    ),
    Function(
        FunctionModifier,
        &'a str,
        Vec<FuncParam<'a>>,
        Option<Type<'a>>,
        Option<Block<'a>>,
    ),
    Enum(&'a str, Vec<EnumParam<'a>>),
    IntEnum(&'a str, IntEnumType, Vec<IntEnumParam<'a>>),
    Struct(&'a str, Vec<StructParam<'a>>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct FuncParam<'a> {
    name: &'a str,
    ty: Type<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnumParam<'a> {
    name: &'a str,
    ty: Type<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructParam<'a> {
    is_pub: bool,
    name: &'a str,
    ty: Type<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum IntEnumType {
    U8,
    I8,
    U16,
    I16,
    U32,
    I32,
    U64,
    I64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IntEnumParam<'a> {
    name: &'a str,
    value: Option<Expr<'a>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeclKeyword {
    Let,
    Mut,
    Const,
    Static,
    Func,
    ConstFunc,
    Struct,
    Enum,
}

impl DeclKeyword {
    pub fn try_parse<'a>(
        tokenizer: &mut Tokenizer<'a>,
    ) -> Result<Option<(bool, Self)>, ParserError<'a>> {
        let peek = tokenizer.peek(0)?;

        let is_pub = if let TokenKind::Keyword(Keyword::Pub) = peek.kind {
            tokenizer.next()?;
            true
        } else {
            false
        };

        let peek = tokenizer.peek(0)?;

        let decl = match peek.kind {
            TokenKind::Keyword(Keyword::Let) => Self::Let,
            TokenKind::Keyword(Keyword::Mut) => Self::Mut,
            TokenKind::Keyword(Keyword::Static) => Self::Static,
            TokenKind::Keyword(Keyword::Func) => Self::Func,
            TokenKind::Keyword(Keyword::Struct) => Self::Struct,
            TokenKind::Keyword(Keyword::Enum) => Self::Enum,
            TokenKind::Keyword(Keyword::Const) => {
                let next = tokenizer.peek(1)?;

                if let TokenKind::Keyword(Keyword::Func) = next.kind {
                    tokenizer.next()?;
                    return Ok(Some((is_pub, Self::ConstFunc)));
                }

                return Ok(Some((is_pub, Self::Const)));
            }
            _ => {
                if is_pub {
                    return Err(ParserError::UnexpectedToken(peek, "Declaration"));
                }

                return Ok(None);
            }
        };

        return Ok(Some((is_pub, decl)));
    }

    pub fn try_into_var(self) -> Option<VariableModifier> {
        return Some(match self {
            Self::Let => VariableModifier::Let,
            Self::Mut => VariableModifier::Mut,
            Self::Const => VariableModifier::Const,
            Self::Static => VariableModifier::Static,
            _ => return None,
        });
    }

    pub fn try_into_func(self) -> Option<FunctionModifier> {
        return Some(match self {
            Self::Func => FunctionModifier::Func,
            Self::ConstFunc => FunctionModifier::ConstFunc,
            _ => return None,
        });
    }
}
