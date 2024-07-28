use crate::{
    ast::{
        decl::{Attr, Attrs, Decl, DeclKeyword, DeclKind, FuncBody, FuncParam, ThisParam},
        statement::{FunctionModifier, VariableModifier, VariableName},
        types::RefKind,
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

    let attrs = parse_attrs(tokenizer)?;

    let peek = tokenizer.peek(0)?;

    let Some((is_pub, decl)) = DeclKeyword::try_parse(tokenizer)? else {
        if attrs.is_some() {
            return Err(ParserError::UnexpectedToken(peek, "Decl keyword"));
        }
        return Ok(None);
    };
    tokenizer.next()?;

    if let Some(modifier) = decl.try_into_var() {
        return Ok(Some(parse_var_decl(
            tokenizer, slice, attrs, modifier, is_pub,
        )?));
    }

    if let Some(modifier) = decl.try_into_func() {
        return Ok(Some(parse_func_decl(
            tokenizer, slice, attrs, modifier, is_pub,
        )?));
    }

    if let DeclKeyword::Struct = decl {
        return Ok(Some(parse_struct_decl(tokenizer, slice, attrs, is_pub)?));
    }

    todo!("enum")
}

fn parse_attrs<'a>(tokenizer: &mut Tokenizer<'a>) -> Result<Option<Attrs<'a>>, ParserError<'a>> {
    let peek = tokenizer.peek(0)?;
    let TokenKind::Symbol(Symbol::Pound) = peek.kind else {
        return Ok(None);
    };
    let start = peek.slice;
    tokenizer.next()?;

    let next = tokenizer.next()?;
    let TokenKind::Symbol(Symbol::BracketOpen) = next.kind else {
        return Err(ParserError::UnexpectedToken(next, "Bracket open"));
    };

    let mut attrs = vec![];
    let mut last = start;

    loop {
        let attr = parse_attr(tokenizer)?;
        attrs.push(attr);

        let next = tokenizer.next()?;
        last = next.slice;
        match next.kind {
            TokenKind::Symbol(Symbol::Comma) => (),
            TokenKind::Symbol(Symbol::BracketClose) => break,
            _ => return Err(ParserError::UnexpectedToken(next, "Comma or Bracket close")),
        }
    }

    return Ok(Some(Attrs {
        slice: start.merge(last),
        attrs,
    }));
}

fn parse_attr<'a>(tokenizer: &mut Tokenizer<'a>) -> Result<Attr<'a>, ParserError<'a>> {
    let next = tokenizer.next()?;
    let start = next.slice;

    let TokenKind::Identifier(name) = next.kind else {
        return Err(ParserError::UnexpectedToken(next, "Identifier"));
    };

    let peek = tokenizer.peek(0)?;
    let TokenKind::Symbol(Symbol::ParenOpen) = peek.kind else {
        return Ok(Attr {
            slice: start,
            name,
            params: vec![],
        });
    };
    tokenizer.next()?;

    let mut params = vec![];
    let mut last = start;

    loop {
        let peek = tokenizer.peek(0)?;
        let Some(expr) = parse_expr(tokenizer)? else {
            return Err(ParserError::UnexpectedToken(peek, "Expr"));
        };

        params.push(expr);

        let next = tokenizer.next()?;
        last = next.slice;

        match next.kind {
            TokenKind::Symbol(Symbol::Comma) => (),
            TokenKind::Symbol(Symbol::ParenClose) => break,
            _ => return Err(ParserError::UnexpectedToken(next, "Comma or Paren close")),
        }
    }

    return Ok(Attr {
        slice: start.merge(last),
        name,
        params,
    });
}

fn parse_struct_decl<'a>(
    tokenizer: &mut Tokenizer<'a>,
    slice: StringSlice<'a>,
    attrs: Option<Attrs<'a>>,
    is_pub: bool,
) -> DeclResult<'a> {
    let ident = tokenizer.next()?;

    let TokenKind::Identifier(name) = ident.kind else {
        return Err(ParserError::UnexpectedToken(ident, "Ident"));
    };

    todo!()
}

fn parse_func_decl<'a>(
    tokenizer: &mut Tokenizer<'a>,
    slice: StringSlice<'a>,
    attrs: Option<Attrs<'a>>,
    modifier: FunctionModifier,
    is_pub: bool,
) -> DeclResult<'a> {
    let ident = tokenizer.next()?;

    let TokenKind::Identifier(name) = ident.kind else {
        return Err(ParserError::UnexpectedToken(ident, "Ident"));
    };

    let peek = tokenizer.next()?;

    let TokenKind::Symbol(Symbol::ParenOpen) = peek.kind else {
        return Err(ParserError::UnexpectedToken(peek, "Paren open"));
    };

    let this_param = parse_this_param(tokenizer)?;

    if this_param.is_some()
        && let TokenKind::Symbol(Symbol::Comma) = tokenizer.peek(0)?.kind
    {
        tokenizer.next()?;
    }

    let mut params = vec![];

    let peek = tokenizer.peek(0)?;

    match peek.kind {
        TokenKind::Symbol(Symbol::ParenClose) => _ = tokenizer.next()?,
        _ => loop {
            let peek = tokenizer.peek(0)?;

            let start = peek.slice;

            let is_mut = if let TokenKind::Keyword(Keyword::Mut) = peek.kind {
                tokenizer.next()?;
                true
            } else {
                false
            };

            let next = tokenizer.next()?;

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
                is_mut,
                name,
                ty,
            });

            let next = tokenizer.next()?;

            match next.kind {
                TokenKind::Symbol(Symbol::Comma) => continue,
                TokenKind::Symbol(Symbol::ParenClose) => break,
                _ => return Err(ParserError::UnexpectedToken(next, "Comma or paren close")),
            }
        },
    }

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
        }
        _ => {
            return Err(ParserError::UnexpectedToken(
                next,
                "Type, function body, or semicolon",
            ))
        }
    };

    Ok(Decl {
        slice: slice.merge(end),
        attrs,
        is_pub,
        kind: DeclKind::Function {
            modifier,
            name,
            this_param,
            params,
            ret,
            body,
        },
    })
}

fn parse_this_param<'a>(
    tokenizer: &mut Tokenizer<'a>,
) -> Result<Option<ThisParam<'a>>, ParserError<'a>> {
    let peek = tokenizer.peek(0)?;

    let start = peek.slice;

    let is_mut = if let TokenKind::Keyword(Keyword::Mut) = peek.kind {
        tokenizer.next()?;
        true
    } else {
        false
    };

    let ref_kind = RefKind::parse(tokenizer)?.map(|it| it.1);

    let next = tokenizer.peek(0)?;
    let TokenKind::Keyword(Keyword::ThisValue) = next.kind else {
        if is_mut || ref_kind.is_some() {
            return Err(ParserError::UnexpectedToken(next, "this"));
        }
        return Ok(None);
    };
    tokenizer.next()?;

    return Ok(Some(ThisParam {
        slice: start.merge(next.slice),
        is_mut,
        ref_kind,
    }));
}

fn parse_var_decl<'a>(
    tokenizer: &mut Tokenizer<'a>,
    slice: StringSlice<'a>,
    attrs: Option<Attrs<'a>>,
    modifier: VariableModifier,
    is_pub: bool,
) -> DeclResult<'a> {
    let ident = tokenizer.next()?;

    let name = match ident.kind {
        TokenKind::Identifier(ident) => VariableName::Identifier(ident),
        TokenKind::Keyword(Keyword::Discard) => VariableName::Discard,
        _ => return Err(ParserError::UnexpectedToken(ident, "Variable name")),
    };

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
                attrs,
                is_pub,
                kind: DeclKind::Variable {
                    modifier,
                    name,
                    ty,
                    init: Some(expr),
                },
            });
        }
        TokenKind::Symbol(Symbol::Semicolon) => {
            tokenizer.next()?;
            return Ok(Decl {
                slice: slice.merge(end),
                attrs,
                is_pub,
                kind: DeclKind::Variable {
                    modifier,
                    name,
                    ty,
                    init: None,
                },
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
