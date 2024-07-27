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
    statement::{FunctionModifier, VariableModifier, VariableName},
    types::Type,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Decl<'a> {
    pub slice: StringSlice<'a>,
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
    ),
}

#[derive(Debug, Clone, PartialEq)]
pub struct FuncParam<'a> {
    name: &'a str,
    ty: Type<'a>,
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
    pub fn try_parse<'a>(tokenizer: &mut Tokenizer<'a>) -> Result<Option<Self>, ParserError<'a>> {
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
                    return Ok(Some(Self::ConstFunc));
                }

                return Ok(Some(Self::Const));
            }
            _ => return Ok(None),
        };

        return Ok(Some(decl));
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
