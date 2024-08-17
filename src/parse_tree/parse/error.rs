use crate::tokenizer::{token::Token, TokenizeError};

#[derive(Debug, Clone, PartialEq)]
pub enum ParserError {
    TokenizeError(TokenizeError),
    UnexpectedToken {
        token: Token,
        throwing_location: String,
    },
}

impl ParserError {
    #[track_caller]
    pub fn unexpected_token(token: Token) -> Self {
        return Self::UnexpectedToken {
            token,
            throwing_location: format!("{}", std::panic::Location::caller()),
        };
    }
}

impl From<TokenizeError> for ParserError {
    fn from(value: TokenizeError) -> Self {
        Self::TokenizeError(value)
    }
}
