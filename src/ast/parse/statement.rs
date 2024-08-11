use crate::{
    ast::{
        parse::{
            expr::{self, parse_expr},
            pattern::parse_pattern,
        },
        statement::{
            Block, IfClause, IfClauseKind, IfCondition, IfStatement, LetMatchClause,
            LetMatchElseStatement, Statement, StatementKind,
        },
    },
    tokenizer::{
        token::{Keyword, Symbol, TokenKind},
        Tokenizer,
    },
};

use super::{decl::parse_decl, error::ParserError};

type StatementResult<'a> = Result<Option<Statement<'a>>, ParserError<'a>>;

pub fn parse_statement<'a>(tokenizer: &mut Tokenizer<'a>) -> StatementResult<'a> {
    if let Some(let_match_else) = parse_let_match_else(tokenizer)? {
        return Ok(Some(Statement {
            slice: let_match_else.slice,
            kind: StatementKind::LetMatchElse(let_match_else),
        }));
    }

    if let Some(decl) = parse_decl(tokenizer)? {
        return Ok(Some(Statement {
            slice: decl.slice,
            kind: StatementKind::Decl(decl),
        }));
    }

    if let Some(expr) = expr::parse_expr(tokenizer)? {
        let peek = tokenizer.peek(0)?;
        let end = peek.slice;

        let TokenKind::Symbol(Symbol::Semicolon) = peek.kind else {
            return Err(ParserError::unexpected_token(peek));
        };
        tokenizer.next()?;

        return Ok(Some(Statement {
            slice: expr.slice.merge(end),
            kind: StatementKind::Expr(expr),
        }));
    }

    if let Some(if_statement) = parse_if_statement(tokenizer)? {
        return Ok(Some(Statement {
            slice: if_statement.slice,
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

    let peek = tokenizer.peek(0)?;
    if let TokenKind::Symbol(Symbol::BraceClose) = peek.kind {
        tokenizer.next()?;
        return Ok(Some(Block {
            slice: start.merge(peek.slice),
            statements: vec![],
        }));
    }

    let mut statements = vec![];

    loop {
        let peek = tokenizer.peek(0)?;
        let Some(statement) = parse_statement(tokenizer)? else {
            return Err(ParserError::unexpected_token(peek));
        };

        statements.push(statement);

        let next = tokenizer.peek(0)?;

        if let TokenKind::Symbol(Symbol::BraceClose) = next.kind {
            tokenizer.next()?;
            let end = next.slice;

            return Ok(Some(Block {
                slice: start.merge(end),
                statements,
            }));
        }
    }
}

fn parse_let_match_else<'a>(
    tokenizer: &mut Tokenizer<'a>,
) -> Result<Option<LetMatchElseStatement<'a>>, ParserError<'a>> {
    let Some(clause) = parse_let_match_clause(tokenizer)? else {
        return Ok(None);
    };

    let next = tokenizer.next()?;
    let TokenKind::Keyword(Keyword::Else) = next.kind else {
        return Err(ParserError::unexpected_token(next));
    };

    let peek = tokenizer.peek(0)?;
    let Some(block) = parse_block(tokenizer)? else {
        return Err(ParserError::unexpected_token(peek));
    };

    return Ok(Some(LetMatchElseStatement {
        slice: clause.slice.merge(block.slice),
        clause,
        block,
    }));
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

    let condition = parse_if_condition_kind(tokenizer)?;

    let peek = tokenizer.peek(0)?;
    let Some(block) = parse_block(tokenizer)? else {
        return Err(ParserError::unexpected_token(peek));
    };

    let mut last = block.slice.clone();
    let mut conditions = vec![IfCondition {
        slice: start.merge(block.slice),
        condition: Some(condition),
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

        let condition = parse_if_condition_kind(tokenizer)?;

        let peek = tokenizer.peek(0)?;
        let Some(block) = parse_block(tokenizer)? else {
            return Err(ParserError::unexpected_token(peek));
        };

        let last = block.slice;
        return Ok(Some(IfCondition {
            slice: start.merge(last),
            condition: Some(condition),
            block,
        }));
    };

    let peek = tokenizer.peek(0)?;
    let Some(block) = parse_block(tokenizer)? else {
        return Err(ParserError::unexpected_token(peek));
    };

    let last = block.slice;
    return Ok(Some(IfCondition {
        slice: start.merge(last),
        condition: None,
        block,
    }));
}

fn parse_if_condition_kind<'a>(
    tokenizer: &mut Tokenizer<'a>,
) -> Result<IfClause<'a>, ParserError<'a>> {
    if let Some(clause) = parse_let_match_clause(tokenizer)? {
        return Ok(IfClause {
            slice: clause.slice,
            kind: IfClauseKind::LetMatch(clause),
        });
    }

    let next = tokenizer.next()?;
    let TokenKind::Symbol(Symbol::ParenOpen) = next.kind else {
        return Err(ParserError::unexpected_token(next));
    };
    let start = next.slice;

    let peek = tokenizer.peek(0)?;
    let Some(expr) = parse_expr(tokenizer)? else {
        return Err(ParserError::unexpected_token(peek));
    };

    let next = tokenizer.next()?;
    let TokenKind::Symbol(Symbol::ParenClose) = next.kind else {
        return Err(ParserError::unexpected_token(next));
    };

    return Ok(IfClause {
        slice: start.merge(next.slice),
        kind: IfClauseKind::Expr(expr),
    });
}

fn parse_let_match_clause<'a>(
    tokenizer: &mut Tokenizer<'a>,
) -> Result<Option<LetMatchClause<'a>>, ParserError<'a>> {
    let peek = [tokenizer.peek(0)?, tokenizer.peek(1)?];
    let start = peek[0].slice;
    let [TokenKind::Keyword(Keyword::Let), TokenKind::Keyword(Keyword::Match)] =
        peek.map(|it| it.kind)
    else {
        return Ok(None);
    };
    tokenizer.next()?;
    tokenizer.next()?;

    let next = tokenizer.next()?;
    let TokenKind::Symbol(Symbol::ParenOpen) = next.kind else {
        return Err(ParserError::unexpected_token(next));
    };

    let pat = parse_pattern(tokenizer)?;

    let next = tokenizer.next()?;
    let TokenKind::Symbol(Symbol::WideArrow) = next.kind else {
        return Err(ParserError::unexpected_token(next));
    };

    let peek = tokenizer.peek(0)?;
    let Some(value) = parse_expr(tokenizer)? else {
        return Err(ParserError::unexpected_token(peek));
    };

    let next = tokenizer.next()?;
    let TokenKind::Symbol(Symbol::ParenClose) = next.kind else {
        return Err(ParserError::unexpected_token(next));
    };
    let end = next.slice;

    return Ok(Some(LetMatchClause {
        slice: start.merge(end),
        pat,
        value,
    }));
}
