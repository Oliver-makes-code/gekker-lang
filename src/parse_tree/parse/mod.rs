use decl::{parse_import, parse_lvl_1_decl, parse_namespace, parse_using};
use error::ParserError;

use crate::tokenizer::{token::TokenKind, Tokenizer};

use super::ParseTree;

pub mod decl;
pub mod error;
pub mod expr;
pub mod pattern;
pub mod statement;
pub mod types;

pub fn parse_root(tokenizer: &mut Tokenizer) -> Result<ParseTree, ParserError> {
    let start = tokenizer.peek(0)?.slice;

    let mut imports = vec![];
    while let Some(import) = parse_import(tokenizer)? {
        imports.push(import);
    }

    let namespace = parse_namespace(tokenizer)?;

    let mut usings = vec![];
    while let Some(using) = parse_using(tokenizer)? {
        usings.push(using);
    }

    let peek = tokenizer.peek(0)?;
    if let TokenKind::Eof = peek.kind {
        return Ok(ParseTree {
            slice: start.merge(&peek.slice),
            imports,
            namespace,
            usings,
            body: vec![],
        });
    }

    let mut body = vec![];

    loop {
        let peek = tokenizer.peek(0)?;
        let Some(decl) = parse_lvl_1_decl(tokenizer)? else {
            return Err(ParserError::unexpected_token(peek));
        };

        body.push(decl);

        let peek = tokenizer.peek(0)?;
        if let TokenKind::Eof = peek.kind {
            return Ok(ParseTree {
                slice: start.merge(&peek.slice),
                imports,
                namespace,
                usings,
                body,
            });
        }
    }
}
