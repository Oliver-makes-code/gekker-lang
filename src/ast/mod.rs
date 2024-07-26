use parse::error::ParserError;

use crate::{
    string::StringSlice,
    tokenizer::{
        token::{Symbol, TokenKind},
        Tokenizer,
    },
};

pub mod expr;
pub mod parse;
pub mod statement;
pub mod types;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdentPath<'a>(pub Vec<&'a str>);

impl<'a> IdentPath<'a> {
    pub fn try_parse(
        tokenizer: &mut Tokenizer<'a>,
    ) -> Result<Option<(StringSlice<'a>, Self)>, ParserError<'a>> {
        let peek = tokenizer.peek(0)?;

        let TokenKind::Identifier(ident) = peek.kind else {
            return Ok(None);
        };

        let start = peek.slice;

        tokenizer.next()?;

        let mut idents = vec![ident];

        let mut peek = tokenizer.peek(0)?;
        let mut end = start;

        while let TokenKind::Symbol(Symbol::DoubleColon) = peek.kind {
            tokenizer.next()?;
            let next = tokenizer.next()?;
            let TokenKind::Identifier(ident) = next.kind else {
                return Err(ParserError::UnexpectedToken(next, "Identifier"));
            };
            end = next.slice;
            idents.push(ident);
            peek = tokenizer.peek(0)?;
        }

        return Ok(Some((start.merge(end), Self(idents))));
    }
}
