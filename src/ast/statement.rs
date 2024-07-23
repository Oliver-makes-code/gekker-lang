use crate::{
    string::StringSlice,
    tokenizer::{
        token::{Keyword, TokenKind},
        Tokenizer,
    },
};

use super::{expr::Expr, parse::error::ParserError};

pub struct Statement<'a> {
    pub slice: StringSlice<'a>,
    pub kind: StatementKind<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StatementKind<'a> {
    Let(Mutability, &'a str, Expr<'a>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mutability {
    Regular,
    Mut,
    Const,
    Static,
}

impl Mutability {
    pub fn try_parse_variable<'a>(
        tokenizer: &mut Tokenizer<'a>,
    ) -> Result<Option<(StringSlice<'a>, Self)>, ParserError<'a>> {
        let peek = tokenizer.peek()?;

        let kind = match peek.kind {
            TokenKind::Keyword(Keyword::Let) => Self::Regular,
            TokenKind::Keyword(Keyword::Mut) => Self::Mut,
            TokenKind::Keyword(Keyword::Const) => Self::Const,
            TokenKind::Keyword(Keyword::Static) => Self::Static,
            _ => return Ok(None),
        };

        return Ok(Some((peek.slice.unwrap(), kind)));
    }
}
