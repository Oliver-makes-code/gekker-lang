use decl::Decl;

use crate::{
    ast::{
        parse::expr,
        statement::{Block, FunctionModifier, Statement, StatementKind, VariableModifier, VariableName},
    },
    string::StringSlice,
    tokenizer::{
        token::{Keyword, Symbol, TokenKind},
        Tokenizer,
    },
};

use super::{error::ParserError, types::parse_type};

mod decl;

type OptStatementResult<'a> = Result<Option<Statement<'a>>, ParserError<'a>>;
type StatementResult<'a> = Result<Statement<'a>, ParserError<'a>>;

pub fn parse_statement<'a>(tokenizer: &mut Tokenizer<'a>) -> OptStatementResult<'a> {
    if let Some(decl) = parse_decl(tokenizer)? {
        return Ok(Some(decl));
    }

    if let Some(expr) = expr::parse_expr(tokenizer)? {
        let peek = tokenizer.peek(0)?;
        let end = peek.slice;

        let TokenKind::Symbol(Symbol::Semicolon) = peek.kind else {
            return Err(ParserError::UnexpectedToken(peek, "Semicolon"));
        };
        tokenizer.next()?;

        return Ok(Some(Statement {
            slice: expr.slice.merge(end),
            kind: StatementKind::Expr(expr),
        }));
    }

    return Ok(None);
}

fn parse_decl<'a>(tokenizer: &mut Tokenizer<'a>) -> OptStatementResult<'a> {
    let Some(decl) = Decl::try_parse(tokenizer)? else {
        return Ok(None);
    };
    let slice = tokenizer.peek(0)?.slice;
    tokenizer.next()?;

    if let Some(decl) = decl.try_into_var() {
        return Ok(Some(parse_var_decl(tokenizer, decl, slice)?));
    }

    if let Some(decl) = decl.try_into_func() {
        return Ok(Some(parse_func_decl(tokenizer, decl, slice)?));
    }

    todo!("struct, enum decls");
}

fn parse_func_decl<'a>(
    tokenizer: &mut Tokenizer<'a>,
    decl: FunctionModifier,
    slice: StringSlice<'a>,
) -> StatementResult<'a> {
    todo!()
}

fn parse_var_decl<'a>(
    tokenizer: &mut Tokenizer<'a>,
    decl: VariableModifier,
    slice: StringSlice<'a>,
) -> StatementResult<'a> {
    let peek = tokenizer.peek(0)?;

    let name = match peek.kind {
        TokenKind::Identifier(ident) => VariableName::Identifier(ident),
        TokenKind::Keyword(Keyword::Discard) => VariableName::Discard,
        _ => return Err(ParserError::UnexpectedToken(peek, "Variable name")),
    };

    tokenizer.clear_peek_queue();

    let mut peek = tokenizer.peek(0)?;

    let mut ty = None;

    let end = if let TokenKind::Symbol(Symbol::Colon) = peek.kind {
        tokenizer.next()?;
        let parsed = parse_type(tokenizer)?;
        ty = Some(parsed.clone());
        peek = tokenizer.peek(0)?;
        parsed.slice
    } else {
        peek.slice
    };

    match peek.kind {
        TokenKind::Symbol(Symbol::Assign) => {
            tokenizer.next()?;
            let Some(expr) = expr::parse_expr(tokenizer)? else {
                return Err(ParserError::UnexpectedToken(tokenizer.peek(0)?, "Expr"));
            };

            let peek = tokenizer.peek(0)?;
            let end = peek.slice;

            let TokenKind::Symbol(Symbol::Semicolon) = peek.kind else {
                return Err(ParserError::UnexpectedToken(peek, "Semicolon"));
            };
            tokenizer.next()?;

            return Ok(Statement {
                slice: slice.merge(end),
                kind: StatementKind::VariableDecl(decl, name, ty, Some(expr)),
            });
        }
        TokenKind::Symbol(Symbol::Semicolon) => {
            tokenizer.next()?;
            return Ok(Statement {
                slice: slice.merge(end),
                kind: StatementKind::VariableDecl(decl, name, ty, None),
            });
        }
        _ => {
            return Err(ParserError::UnexpectedToken(
                peek,
                "Assignment or Semicolon",
            ))
        }
    }
}

pub fn parse_block<'a>(
    tokenizer: &mut Tokenizer<'a>,
) -> Result<Option<Block<'a>>, ParserError<'a>> {
    let peek = tokenizer.peek(0)?;

    let TokenKind::Symbol(Symbol::BraceOpen) = peek.kind else {
        return Ok(None);
    };

    let start = peek.slice;

    let mut peek = tokenizer.next()?;

    let mut statements = vec![];

    while peek.kind != TokenKind::Symbol(Symbol::BraceClose) {
        let Some(statement) = parse_statement(tokenizer)? else {
            return Err(ParserError::UnexpectedToken(peek, "Statement"));
        };

        statements.push(statement);

        peek = tokenizer.peek(0)?;
    }

    let end = peek.slice;

    return Ok(Some(Block {
        slice: start.merge(end),
        statements,
    }));
}
