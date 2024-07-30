use crate::{
    ast::{
        parse::expr,
        statement::{Block, Statement, StatementKind},
    },
    tokenizer::{
        token::{Symbol, TokenKind},
        Tokenizer,
    },
};

use super::{decl::parse_decl, error::ParserError};

type StatementResult<'a> = Result<Option<Statement<'a>>, ParserError<'a>>;

pub fn parse_statement<'a>(tokenizer: &mut Tokenizer<'a>) -> StatementResult<'a> {
    if let Some(decl) = parse_decl(tokenizer)? {
        return Ok(Some(Statement {
            slice: decl.slice.clone(),
            kind: StatementKind::Decl(decl),
        }));
    }

    if let Some(expr) = expr::parse_expr(tokenizer)? {
        let peek = tokenizer.peek(0)?;
        let end = peek.slice;

        let TokenKind::Symbol(Symbol::Semicolon) = peek.kind else {
            return Err(ParserError::UnexpectedToken(peek));
        };
        tokenizer.next()?;

        return Ok(Some(Statement {
            slice: expr.slice.merge(end),
            kind: StatementKind::Expr(expr),
        }));
    }

    return Ok(None);
}

pub fn parse_block<'a>(
    tokenizer: &mut Tokenizer<'a>,
) -> Result<Option<Block<'a>>, ParserError<'a>> {
    let peek = tokenizer.peek(0)?;

    let TokenKind::Symbol(Symbol::BraceOpen) = peek.kind else {
        return Ok(None);
    };

    tokenizer.next()?;

    let start = peek.slice;

    let mut statements = vec![];

    loop {
        let peek = tokenizer.peek(0)?;
        let Some(statement) = parse_statement(tokenizer)? else {
            return Err(ParserError::UnexpectedToken(peek));
        };

        statements.push(statement);

        let next = tokenizer.next()?;

        if let TokenKind::Symbol(Symbol::BraceClose) = next.kind {
            let end = next.slice;

            return Ok(Some(Block {
                slice: start.merge(end),
                statements,
            }));
        }
    }
}
