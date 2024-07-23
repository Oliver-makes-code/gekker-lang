use crate::tokenizer::{token::Token, TokenizeError};

#[derive(Debug, Clone, PartialEq)]
pub enum ParserError<'a> {
    TokenizeError(TokenizeError<'a>),
    UnexpectedToken(Token<'a>, &'static str),
    UnexpectedEof,
}

impl<'a> From<TokenizeError<'a>> for ParserError<'a> {
    fn from(value: TokenizeError<'a>) -> Self {
        Self::TokenizeError(value)
    }
}