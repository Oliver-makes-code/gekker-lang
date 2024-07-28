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
    types::{RefKind, Type},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Decl<'a> {
    pub slice: StringSlice<'a>,
    pub attrs: Option<Attrs<'a>>,
    pub is_pub: bool,
    pub kind: DeclKind<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DeclKind<'a> {
    Variable {
        modifier: VariableModifier,
        name: VariableName<'a>,
        ty: Option<Type<'a>>,
        init: Option<Expr<'a>>,
    },
    Function {
        modifier: FunctionModifier,
        name: &'a str,
        this_param: Option<ThisParam<'a>>,
        params: Vec<FuncParam<'a>>,
        ret: Option<Type<'a>>,
        body: Option<FuncBody<'a>>,
    },
    Enum {
        name: &'a str,
        params: Vec<EnumParam<'a>>,
    },
    IntEnum {
        name: &'a str,
        ty: IntEnumType,
        params: Vec<IntEnumParam<'a>>,
    },
    Struct {
        name: &'a str,
        params: Vec<StructParam<'a>>,
    },
    WrapperStruct {
        name: &'a str,
        ty: Type<'a>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Attrs<'a> {
    pub slice: StringSlice<'a>,
    pub attrs: Vec<Attr<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Attr<'a> {
    pub slice: StringSlice<'a>,
    pub name: &'a str,
    pub params: Vec<Expr<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FuncBody<'a> {
    Block(Block<'a>),
    Expr(Expr<'a>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ThisParam<'a> {
    pub slice: StringSlice<'a>,
    pub is_mut: bool,
    pub ref_kind: Option<RefKind>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FuncParam<'a> {
    pub slice: StringSlice<'a>,
    pub is_mut: bool,
    pub name: &'a str,
    pub ty: Type<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnumParam<'a> {
    pub slice: StringSlice<'a>,
    pub name: &'a str,
    pub ty: Type<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructParam<'a> {
    pub slice: StringSlice<'a>,
    pub is_pub: bool,
    pub name: &'a str,
    pub ty: Type<'a>,
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
    pub slice: StringSlice<'a>,
    pub name: &'a str,
    pub value: Option<Expr<'a>>,
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
