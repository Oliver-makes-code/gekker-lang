use crate::{
    parse_tree::{
        parse::{
            expr::{self, parse_expr},
            pattern::parse_pattern,
        },
        statement::{
            Block, IfClause, IfClauseKind, IfCondition, IfStatement, LetMatchClause,
            LetMatchElseStatement, MatchBlock, MatchBlockKind, MatchClause, MatchStatement,
            ReturnStatement, Statement, StatementKind,
        },
    },
    tokenizer::{
        token::{Keyword, Symbol, TokenKind},
        Tokenizer,
    },
};

use super::{decl::parse_var_decl, error::ParserError};

type StatementResult = Result<Option<Statement>, ParserError>;

pub fn parse_statement(tokenizer: &mut Tokenizer) -> StatementResult {
    if let Some(ret) = parse_return(tokenizer)? {
        return Ok(Some(Statement {
            slice: ret.slice.clone(),
            kind: StatementKind::Return(ret),
        }));
    }

    if let Some(let_match_else) = parse_let_match_else(tokenizer)? {
        return Ok(Some(Statement {
            slice: let_match_else.slice.clone(),
            kind: StatementKind::LetMatchElse(let_match_else),
        }));
    }

    if let Some(mat) = parse_match(tokenizer)? {
        return Ok(Some(Statement {
            slice: mat.slice.clone(),
            kind: StatementKind::Match(mat),
        }));
    }

    if let Some(decl) = parse_var_decl(tokenizer)? {
        return Ok(Some(Statement {
            slice: decl.slice.clone(),
            kind: StatementKind::Decl(decl),
        }));
    }

    if let Some(expr) = expr::parse_expr(tokenizer)? {
        let peek = tokenizer.peek(0)?;
        let end = peek.slice.clone();

        let TokenKind::Symbol(Symbol::Semicolon) = peek.kind else {
            return Err(ParserError::unexpected_token(peek));
        };
        tokenizer.next()?;

        return Ok(Some(Statement {
            slice: expr.slice.merge(&end),
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

pub fn parse_block(tokenizer: &mut Tokenizer) -> Result<Option<Block>, ParserError> {
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
            slice: start.merge(&peek.slice),
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
                slice: start.merge(&end),
                statements,
            }));
        }
    }
}

fn parse_return(tokenizer: &mut Tokenizer) -> Result<Option<ReturnStatement>, ParserError> {
    let peek = tokenizer.peek(0)?;
    let TokenKind::Keyword(Keyword::Return) = peek.kind else {
        return Ok(None);
    };
    tokenizer.next()?;
    let start = peek.slice;

    let value = parse_expr(tokenizer)?;

    let peek = tokenizer.peek(0)?;
    let condition = if let TokenKind::Keyword(Keyword::If) = peek.kind {
        tokenizer.next()?;

        let next = tokenizer.next()?;
        let TokenKind::Symbol(Symbol::ParenOpen) = next.kind else {
            return Err(ParserError::unexpected_token(next));
        };

        let peek = tokenizer.peek(0)?;
        let Some(expr) = parse_expr(tokenizer)? else {
            return Err(ParserError::unexpected_token(peek));
        };

        let next = tokenizer.next()?;
        let TokenKind::Symbol(Symbol::ParenClose) = next.kind else {
            return Err(ParserError::unexpected_token(next));
        };

        Some(expr)
    } else {
        None
    };

    let next = tokenizer.next()?;
    let TokenKind::Symbol(Symbol::Semicolon) = next.kind else {
        return Err(ParserError::unexpected_token(next));
    };

    return Ok(Some(ReturnStatement {
        slice: start.merge(&next.slice),
        value,
        condition,
    }));
}

fn parse_match(tokenizer: &mut Tokenizer) -> Result<Option<MatchStatement>, ParserError> {
    let peek = tokenizer.peek(0)?;
    let TokenKind::Keyword(Keyword::Match) = peek.kind else {
        return Ok(None);
    };
    tokenizer.next()?;
    let start = peek.slice;

    let next = tokenizer.next()?;
    let TokenKind::Symbol(Symbol::ParenOpen) = next.kind else {
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

    let next = tokenizer.next()?;
    let TokenKind::Symbol(Symbol::BraceOpen) = next.kind else {
        return Err(ParserError::unexpected_token(next));
    };

    let peek = tokenizer.peek(0)?;
    if let TokenKind::Symbol(Symbol::BraceClose) = peek.kind {
        return Ok(Some(MatchStatement {
            slice: start.merge(&peek.slice),
            value,
            clauses: vec![],
        }));
    }

    let mut clauses = vec![];

    loop {
        let clause = parse_match_clause(tokenizer)?;
        clauses.push(clause);

        let next = tokenizer.peek(0)?;

        if let TokenKind::Symbol(Symbol::BraceClose) = next.kind {
            tokenizer.next()?;
            return Ok(Some(MatchStatement {
                slice: start.merge(&next.slice),
                value,
                clauses,
            }));
        }
    }
}

fn parse_match_clause(tokenizer: &mut Tokenizer) -> Result<MatchClause, ParserError> {
    let pat = parse_pattern(tokenizer)?;

    let next = tokenizer.next()?;
    let TokenKind::Symbol(Symbol::WideArrow) = next.kind else {
        return Err(ParserError::unexpected_token(next));
    };

    let block = parse_match_block(tokenizer)?;
    return Ok(MatchClause {
        slice: pat.slice.merge(&block.slice),
        pat,
        block,
    });
}

fn parse_match_block(tokenizer: &mut Tokenizer) -> Result<MatchBlock, ParserError> {
    let peek = tokenizer.peek(0)?;
    if let TokenKind::Symbol(Symbol::BraceOpen) = peek.kind {
        let Some(block) = parse_block(tokenizer)? else {
            return Err(ParserError::unexpected_token(peek));
        };

        return Ok(MatchBlock {
            slice: block.slice.clone(),
            kind: MatchBlockKind::Block(block),
        });
    }

    let Some(statement) = parse_statement(tokenizer)? else {
        return Err(ParserError::unexpected_token(peek))?;
    };

    return Ok(MatchBlock {
        slice: statement.slice.clone(),
        kind: MatchBlockKind::Statement(Box::new(statement)),
    });
}

fn parse_let_match_else(
    tokenizer: &mut Tokenizer,
) -> Result<Option<LetMatchElseStatement>, ParserError> {
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
        slice: clause.slice.merge(&block.slice),
        clause,
        block,
    }));
}

fn parse_if_statement(tokenizer: &mut Tokenizer) -> Result<Option<IfStatement>, ParserError> {
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
        slice: start.merge(&block.slice),
        condition: Some(condition),
        block,
    }];

    while let Some(condition) = parse_else_block(tokenizer)? {
        last = condition.slice.clone();
        conditions.push(condition);
    }

    return Ok(Some(IfStatement {
        slice: start.merge(&last),
        conditions,
    }));
}

fn parse_else_block(tokenizer: &mut Tokenizer) -> Result<Option<IfCondition>, ParserError> {
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

        return Ok(Some(IfCondition {
            slice: start.merge(&block.slice),
            condition: Some(condition),
            block,
        }));
    };

    let peek = tokenizer.peek(0)?;
    let Some(block) = parse_block(tokenizer)? else {
        return Err(ParserError::unexpected_token(peek));
    };

    return Ok(Some(IfCondition {
        slice: start.merge(&block.slice),
        condition: None,
        block,
    }));
}

fn parse_if_condition_kind(tokenizer: &mut Tokenizer) -> Result<IfClause, ParserError> {
    if let Some(clause) = parse_let_match_clause(tokenizer)? {
        return Ok(IfClause {
            slice: clause.slice.clone(),
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
        slice: start.merge(&next.slice),
        kind: IfClauseKind::Expr(expr),
    });
}

fn parse_let_match_clause(
    tokenizer: &mut Tokenizer,
) -> Result<Option<LetMatchClause>, ParserError> {
    let peek = [tokenizer.peek(0)?, tokenizer.peek(1)?];
    let start = peek[0].slice.clone();
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
        slice: start.merge(&end),
        pat,
        value,
    }));
}
