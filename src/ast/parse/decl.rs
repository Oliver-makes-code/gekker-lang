use crate::{
    ast::{
        decl::{Decl, DeclKeyword, DeclKind},
        statement::{FunctionModifier, VariableModifier, VariableName},
    },
    string::StringSlice,
    tokenizer::{
        token::{Keyword, Symbol, TokenKind},
        Tokenizer,
    },
};

use super::{error::ParserError, expr::parse_expr, types::parse_type};

type DeclResult<'a> = Result<Decl<'a>, ParserError<'a>>;
type OptDeclResult<'a> = Result<Option<Decl<'a>>, ParserError<'a>>;

pub fn parse_decl<'a>(tokenizer: &mut Tokenizer<'a>) -> OptDeclResult<'a> {
    let Some(decl) = DeclKeyword::try_parse(tokenizer)? else {
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
) -> DeclResult<'a> {
    todo!()
}

fn parse_var_decl<'a>(
    tokenizer: &mut Tokenizer<'a>,
    decl: VariableModifier,
    slice: StringSlice<'a>,
) -> DeclResult<'a> {
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
            let Some(expr) = parse_expr(tokenizer)? else {
                return Err(ParserError::UnexpectedToken(tokenizer.peek(0)?, "Expr"));
            };

            let peek = tokenizer.peek(0)?;
            let end = peek.slice;

            let TokenKind::Symbol(Symbol::Semicolon) = peek.kind else {
                return Err(ParserError::UnexpectedToken(peek, "Semicolon"));
            };
            tokenizer.next()?;

            return Ok(Decl {
                slice: slice.merge(end),
                kind: DeclKind::Variable(decl, name, ty, Some(expr)),
            });
        }
        TokenKind::Symbol(Symbol::Semicolon) => {
            tokenizer.next()?;
            return Ok(Decl {
                slice: slice.merge(end),
                kind: DeclKind::Variable(decl, name, ty, None),
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
