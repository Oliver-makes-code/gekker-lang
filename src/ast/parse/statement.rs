use crate::{
    ast::{
        parse::expr::{self, parse_expr},
        statement::{Block, IfCondition, IfStatement, Statement, StatementKind},
    },
    tokenizer::{
        token::{Keyword, Symbol, TokenKind},
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

    if let Some(if_statement) = parse_if_statement(tokenizer)? {
        return Ok(Some(Statement {
            slice: if_statement.slice.clone(),
            kind: StatementKind::If(if_statement),
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

fn parse_if_statement<'a>(
    tokenizer: &mut Tokenizer<'a>,
) -> Result<Option<IfStatement<'a>>, ParserError<'a>> {
    let peek = tokenizer.peek(0)?;
    let TokenKind::Keyword(Keyword::If) = peek.kind else {
        return Ok(None);
    };
    let start = peek.slice;
    tokenizer.next()?;

    let peek = tokenizer.peek(0)?;
    let Some(expr) = parse_expr(tokenizer)? else {
        return Err(ParserError::UnexpectedToken(peek));
    };

    let peek = tokenizer.peek(0)?;
    let Some(block) = parse_block(tokenizer)? else {
        return Err(ParserError::UnexpectedToken(peek));
    };

    let mut last = block.slice.clone();
    let mut conditions = vec![IfCondition {
        slice: start.merge(block.slice),
        condition: Some(expr),
        block,
    }];

    while let Some(condition) = parse_else_block(tokenizer)? {
        last = condition.slice.clone();
        conditions.push(condition);
    }

    return Ok(Some(IfStatement {
        slice: start.merge(last),
        conditions,
    }));
}

fn parse_else_block<'a>(
    tokenizer: &mut Tokenizer<'a>,
) -> Result<Option<IfCondition<'a>>, ParserError<'a>> {
    let peek = tokenizer.peek(0)?;
    let TokenKind::Keyword(Keyword::Else) = peek.kind else {
        return Ok(None);
    };
    let start = peek.slice;
    tokenizer.next()?;

    let peek = tokenizer.peek(0)?;

    if let TokenKind::Keyword(Keyword::If) = peek.kind {
        tokenizer.next()?;
        let peek = tokenizer.peek(0)?;
        let Some(expr) = parse_expr(tokenizer)? else {
            return Err(ParserError::UnexpectedToken(peek));
        };
        let peek = tokenizer.peek(0)?;
        let Some(block) = parse_block(tokenizer)? else {
            return Err(ParserError::UnexpectedToken(peek));
        };

        let last = block.slice;
        return Ok(Some(IfCondition {
            slice: start.merge(last),
            condition: Some(expr),
            block,
        }));
    };

    let peek = tokenizer.peek(0)?;
    let Some(block) = parse_block(tokenizer)? else {
        return Err(ParserError::UnexpectedToken(peek));
    };

    let last = block.slice;
    return Ok(Some(IfCondition {
        slice: start.merge(last),
        condition: None,
        block,
    }));
}
