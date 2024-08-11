use crate::tokenizer::{token::Token, TokenizeError};

#[derive(Debug, Clone, PartialEq)]
pub enum ParserError<'a> {
    TokenizeError(TokenizeError<'a>),
    UnexpectedToken {
        token: Token<'a>,
        throwing_location: String,
    },
}

impl<'a> ParserError<'a> {
    #[track_caller]
    pub fn unexpected_token(token: Token<'a>) -> Self {
        return Self::UnexpectedToken {
            token,
            throwing_location: format!("{}", std::panic::Location::caller()),
        };
    }
}

impl<'a> From<TokenizeError<'a>> for ParserError<'a> {
    fn from(value: TokenizeError<'a>) -> Self {
        Self::TokenizeError(value)
    }
}
