use crate::{
    ast::{
        decl::{Decl, DeclKeyword, DeclKind, FuncBody, FuncParam}, statement::{FunctionModifier, VariableModifier, VariableName}
    },
    string::StringSlice,
    tokenizer::{
        token::{Keyword, Symbol, TokenKind},
        Tokenizer,
    },
};

use super::{error::ParserError, expr::parse_expr, statement::parse_block, types::parse_type};

type DeclResult<'a> = Result<Decl<'a>, ParserError<'a>>;
type OptDeclResult<'a> = Result<Option<Decl<'a>>, ParserError<'a>>;

pub fn parse_decl<'a>(tokenizer: &mut Tokenizer<'a>) -> OptDeclResult<'a> {
    let slice = tokenizer.peek(0)?.slice;
    let Some((is_pub, decl)) = DeclKeyword::try_parse(tokenizer)? else {
        return Ok(None);
    };
    tokenizer.next()?;

    if let Some(decl) = decl.try_into_var() {
        return Ok(Some(parse_var_decl(tokenizer, decl, slice, is_pub)?));
    }

    if let Some(decl) = decl.try_into_func() {
        return Ok(Some(parse_func_decl(tokenizer, decl, slice, is_pub)?));
    }

    todo!("struct, enum decls");
}

fn parse_func_decl<'a>(
    tokenizer: &mut Tokenizer<'a>,
    modifier: FunctionModifier,
    slice: StringSlice<'a>,
    is_pub: bool,
) -> DeclResult<'a> {
    let peek = tokenizer.peek(0)?;

    let TokenKind::Identifier(name) = peek.kind else {
        return Err(ParserError::UnexpectedToken(peek, "Ident"));
    };

    tokenizer.next()?;

    let peek = tokenizer.next()?;

    let TokenKind::Symbol(Symbol::ParenOpen) = peek.kind else {
        return Err(ParserError::UnexpectedToken(peek, "Paren open"));
    };

    let mut peek = tokenizer.peek(0)?;

    let mut params = vec![];

    while TokenKind::Symbol(Symbol::ParenClose) != peek.kind {
        let next = tokenizer.next()?;

        let start = next.slice;

        let TokenKind::Identifier(name) = next.kind else {
            return Err(ParserError::UnexpectedToken(next, "Identifier"));
        };

        let next = tokenizer.next()?;

        let TokenKind::Symbol(Symbol::Colon) = next.kind else {
            return Err(ParserError::UnexpectedToken(next, "Colon"));
        };

        let ty = parse_type(tokenizer)?;

        params.push(FuncParam {
            slice: start.merge(ty.slice),
            name,
            ty,
        });

        peek = tokenizer.peek(0)?;

        let TokenKind::Symbol(Symbol::Comma | Symbol::ParenClose) = peek.kind else {
            return Err(ParserError::UnexpectedToken(peek, "Comma or Paren close"));
        };
        if let TokenKind::Symbol(Symbol::Comma) = peek.kind {
            tokenizer.next()?;
        }
    }
    tokenizer.next()?;

    let next = tokenizer.peek(0)?;

    let ret = if let TokenKind::Symbol(Symbol::Colon) = next.kind {
        tokenizer.next()?;
        Some(parse_type(tokenizer)?)
    } else {
        None
    };

    let next = tokenizer.peek(0)?;
    let mut end = next.slice;

    let body = match next.kind {
        TokenKind::Symbol(Symbol::WideArrow) => {
            tokenizer.next()?;
            let t = tokenizer.peek(0)?;
            let Some(expr) = parse_expr(tokenizer)? else {
                return Err(ParserError::UnexpectedToken(t, "Expr"));
            };
            let next = tokenizer.next()?;
            let TokenKind::Symbol(Symbol::Semicolon) = next.kind else {
                return Err(ParserError::UnexpectedToken(next, "Semicolon"));
            };
            end = next.slice;
            Some(FuncBody::Expr(expr))
        }
        TokenKind::Symbol(Symbol::BraceOpen) => {
            let block = parse_block(tokenizer)?.unwrap();

            end = block.slice;

            Some(FuncBody::Block(block))
        }
        TokenKind::Symbol(Symbol::Semicolon) => {
            tokenizer.next()?;
            None
        },
        _ => {
            return Err(ParserError::UnexpectedToken(
                next,
                "Type, function body, or semicolon",
            ))
        }
    };

    Ok(Decl { slice: slice.merge(end), is_pub, kind: DeclKind::Function { modifier, name, params, ret, body } })
}

fn parse_var_decl<'a>(
    tokenizer: &mut Tokenizer<'a>,
    modifier: VariableModifier,
    slice: StringSlice<'a>,
    is_pub: bool,
) -> DeclResult<'a> {
    let peek = tokenizer.peek(0)?;

    let name = match peek.kind {
        TokenKind::Identifier(ident) => VariableName::Identifier(ident),
        TokenKind::Keyword(Keyword::Discard) => VariableName::Discard,
        _ => return Err(ParserError::UnexpectedToken(peek, "Variable name")),
    };

    tokenizer.next()?;

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
                kind: DeclKind::Variable {
                    modifier,
                    name,
                    ty,
                    init: Some(expr),
                },
                is_pub,
            });
        }
        TokenKind::Symbol(Symbol::Semicolon) => {
            tokenizer.next()?;
            return Ok(Decl {
                slice: slice.merge(end),
                kind: DeclKind::Variable {
                    modifier,
                    name,
                    ty,
                    init: None,
                },
                is_pub,
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
