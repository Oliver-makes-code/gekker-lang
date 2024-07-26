use decl::Decl;

use crate::{
    ast::{
        parse::expr,
        statement::{Statement, StatementKind, VariableModifier, VariableName},
    },
    string::StringSlice,
    tokenizer::{
        token::{Keyword, Symbol, TokenKind},
        Tokenizer,
    },
};

use super::error::ParserError;

mod decl;

type OptStatementResult<'a> = Result<Option<Statement<'a>>, ParserError<'a>>;
type StatementResult<'a> = Result<Statement<'a>, ParserError<'a>>;

pub fn parse_root<'a>(tokenizer: &mut Tokenizer<'a>) -> OptStatementResult<'a> {
    if let Some(decl) = parse_decl(tokenizer)? {
        return Ok(Some(decl));
    }

    if let Some(expr) = expr::parse_root(tokenizer)? {
        let peek = tokenizer.peek(0)?;
        let end = peek.slice;

        let TokenKind::Symbol(Symbol::Semicolon) = peek.kind else {
            return Err(ParserError::UnexpectedToken(peek, "Semicolon"));
        };

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

    todo!("Function decls");
}

fn parse_var_decl<'a>(
    tokenizer: &mut Tokenizer<'a>,
    decl: VariableModifier,
    slice: StringSlice<'a>,
) -> StatementResult<'a> {
    let peek = tokenizer.peek(0)?;
    // TODO: parse type def

    let name = match peek.kind {
        TokenKind::Identifier(ident) => VariableName::Identifier(ident),
        TokenKind::Keyword(Keyword::Discard) => VariableName::Discard,
        _ => return Err(ParserError::UnexpectedToken(peek, "Variable name")),
    };

    tokenizer.clear_peek_queue();

    let peek = tokenizer.peek(0)?;
    let end = peek.slice;

    if let TokenKind::Symbol(Symbol::Colon) = peek.kind {
        todo!("Parse type");
    }

    match peek.kind {
        TokenKind::Symbol(Symbol::Assign) => {
            tokenizer.clear_peek_queue();
            let Some(expr) = expr::parse_root(tokenizer)? else {
                return Err(ParserError::UnexpectedToken(tokenizer.peek(0)?, "Expr"));
            };

            let peek = tokenizer.peek(0)?;
            let end = peek.slice;

            let TokenKind::Symbol(Symbol::Semicolon) = peek.kind else {
                return Err(ParserError::UnexpectedToken(peek, "Semicolon"));
            };

            return Ok(Statement {
                slice: slice.merge(end),
                kind: StatementKind::VariableDecl(decl, name, Some(expr)),
            });
        }
        TokenKind::Symbol(Symbol::Semicolon) => {
            return Ok(Statement {
                slice: slice.merge(end),
                kind: StatementKind::VariableDecl(decl, name, None),
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
