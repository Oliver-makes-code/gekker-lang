use crate::tokenizer::{token::Token, TokenizeError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParserError<'a> {
    TokenizeError(TokenizeError<'a>),
    UnexpectedToken(Token<'a>),
    UnexpectedEof,
}

impl<'a> From<TokenizeError<'a>> for ParserError<'a> {
    fn from(value: TokenizeError<'a>) -> Self {
        Self::TokenizeError(value)
    }
}
