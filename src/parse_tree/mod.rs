use std::{fmt::Debug, sync::Arc};

use decl::DeclLvl1;
use parse::error::ParserError;

use crate::{
    string::StringSlice,
    tokenizer::{
        token::{Symbol, TokenKind},
        Tokenizer,
    },
};

pub mod decl;
pub mod expr;
pub mod parse;
pub mod pattern;
pub mod statement;
pub mod types;

#[derive(Debug, Clone, PartialEq)]
pub struct ParseTree {
    pub slice: StringSlice,
    pub body: Vec<DeclLvl1>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdentPath {
    pub slice: StringSlice,
    pub path: Vec<Arc<str>>,
}

impl IdentPath {
    pub fn try_parse(tokenizer: &mut Tokenizer) -> Result<Option<Self>, ParserError> {
        let peek = tokenizer.peek(0)?;

        let TokenKind::Identifier(ident) = peek.kind else {
            return Ok(None);
        };

        let start = peek.slice;

        tokenizer.next()?;

        let mut idents = vec![ident];

        // let mut peek = tokenizer.peek(0)?;
        let mut end = start.clone();

        loop {
            let peek = tokenizer.peek(0)?;
            let TokenKind::Symbol(Symbol::DoubleColon) = peek.kind else {
                break;
            };
            let peek = tokenizer.peek(1)?;
            let TokenKind::Identifier(ident) = peek.kind else {
                break;
            };
            tokenizer.next()?;
            tokenizer.next()?;
            end = peek.slice;
            idents.push(ident);
        }

        return Ok(Some(Self {
            slice: start.merge(&end),
            path: idents,
        }));
    }
}
