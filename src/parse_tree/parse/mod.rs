use decl::parse_lvl_1_decl;
use error::ParserError;

use crate::tokenizer::{token::TokenKind, Tokenizer};

use super::ParseTree;

pub mod decl;
pub mod error;
pub mod expr;
pub mod pattern;
pub mod statement;
pub mod types;

pub fn parse_root<'a>(tokenizer: &mut Tokenizer<'a>) -> Result<ParseTree<'a>, ParserError<'a>> {
    let mut body = vec![];

    let peek = tokenizer.peek(0)?;
    if let TokenKind::Eof = tokenizer.peek(0)?.kind {
        return Ok(ParseTree {
            slice: peek.slice,
            body,
        });
    }

    loop {
        let peek = tokenizer.peek(0)?;
        let Some(decl) = parse_lvl_1_decl(tokenizer)? else {
            return Err(ParserError::unexpected_token(peek));
        };
        body.push(decl);

        let peek = tokenizer.peek(0)?;
        if let TokenKind::Eof = tokenizer.peek(0)?.kind {
            return Ok(ParseTree {
                slice: peek.slice,
                body,
            });
        }
    }
}
