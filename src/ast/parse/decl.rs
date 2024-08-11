use crate::{
    ast::{
        decl::{
            Attr, Attrs, ClauseKind, Decl, DeclKeyword, DeclKind, FuncBody, FuncParam, GenericType,
            GenericsDecl, IntEnumBody, IntEnumParam, IntEnumType, StructBody, StructParam,
            ThisParam, TypeClause,
        },
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

    let generics = parse_generic_list(tokenizer)?;

    let peek = tokenizer.peek(0)?;

    let Some((is_pub, decl)) = DeclKeyword::try_parse(tokenizer)? else {
        if attrs.is_some() || generics.is_some() {
            return Err(ParserError::unexpected_token(peek));
        }
        return Ok(None);
    };
    tokenizer.next()?;

    if let Some(modifier) = decl.try_into_var() {
        return Ok(Some(parse_var_decl(
            tokenizer, slice, attrs, generics, modifier, is_pub,
        )?));
    }

    if let Some(modifier) = decl.try_into_func() {
        return Ok(Some(parse_func_decl(
            tokenizer, slice, attrs, generics, modifier, is_pub,
        )?));
    }

    if let DeclKeyword::Struct = decl {
        return Ok(Some(parse_struct_decl(
            tokenizer, slice, attrs, generics, is_pub,
        )?));
    }

    if let DeclKeyword::Enum = decl {
        return Ok(Some(parse_enum_decl(
            tokenizer, slice, attrs, generics, is_pub,
        )?));
    }

    panic!("Impossible state");
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
        return Err(ParserError::unexpected_token(next));
    };

    let mut attrs = vec![];
    let mut last;

    loop {
        let attr = parse_attr(tokenizer)?;
        attrs.push(attr);

        let next = tokenizer.next()?;
        last = next.slice;
        match next.kind {
            TokenKind::Symbol(Symbol::Comma) => (),
            TokenKind::Symbol(Symbol::BracketClose) => break,
            _ => return Err(ParserError::unexpected_token(next)),
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
        return Err(ParserError::unexpected_token(next));
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
    let mut last;

    loop {
        let peek = tokenizer.peek(0)?;

        let Some(expr) = parse_expr(tokenizer)? else {
            return Err(ParserError::unexpected_token(peek));
        };

        params.push(expr);

        let next = tokenizer.next()?;
        last = next.slice;

        match next.kind {
            TokenKind::Symbol(Symbol::Comma) => (),
            TokenKind::Symbol(Symbol::ParenClose) => break,
            _ => return Err(ParserError::unexpected_token(next)),
        }
    }

    return Ok(Attr {
        slice: start.merge(last),
        name,
        params,
    });
}

fn parse_generic_list<'a>(
    tokenizer: &mut Tokenizer<'a>,
) -> Result<Option<GenericsDecl<'a>>, ParserError<'a>> {
    let peek = tokenizer.peek(0)?;
    let TokenKind::Keyword(Keyword::Where) = peek.kind else {
        return Ok(None);
    };
    tokenizer.next()?;
    let start = peek.slice;

    let mut tys = vec![];
    let mut last = start;

    while let Some(ty) = parse_generic_type(tokenizer)? {
        last = ty.slice.clone();
        tys.push(ty);
    }

    return Ok(Some(GenericsDecl {
        slice: start.merge(last),
        tys,
    }));
}

fn parse_generic_type<'a>(
    tokenizer: &mut Tokenizer<'a>,
) -> Result<Option<GenericType<'a>>, ParserError<'a>> {
    let peek = tokenizer.peek(0)?;

    let TokenKind::Identifier(name) = peek.kind else {
        return Ok(None);
    };
    tokenizer.next()?;

    let start = peek.slice;

    let next = tokenizer.next()?;
    match next.kind {
        TokenKind::Symbol(Symbol::Colon) => (),
        TokenKind::Symbol(Symbol::Semicolon) => {
            return Ok(Some(GenericType {
                slice: start.merge(next.slice),
                name,
                clauses: vec![],
            }))
        }
        _ => return Err(ParserError::unexpected_token(next)),
    }

    let mut clauses = vec![];

    loop {
        let clause = parse_type_clause(tokenizer)?;

        clauses.push(clause);

        let next = tokenizer.next()?;
        match next.kind {
            TokenKind::Symbol(Symbol::Comma) => (),
            TokenKind::Symbol(Symbol::Semicolon) => {
                return Ok(Some(GenericType {
                    slice: start.merge(next.slice),
                    name,
                    clauses,
                }))
            }
            _ => return Err(ParserError::unexpected_token(next)),
        }
    }
}

fn parse_type_clause<'a>(tokenizer: &mut Tokenizer<'a>) -> Result<TypeClause<'a>, ParserError<'a>> {
    let peek = tokenizer.peek(0)?;
    let start = peek.slice;

    let exclude = if let TokenKind::Symbol(Symbol::BoolNot) = peek.kind {
        tokenizer.next()?;
        true
    } else {
        false
    };

    let peek = tokenizer.peek(0)?;

    match peek.kind {
        TokenKind::Keyword(Keyword::Default) => {
            tokenizer.next()?;
            return Ok(TypeClause {
                slice: start.merge(peek.slice),
                exclude,
                ty: ClauseKind::Default,
            });
        }
        _ => {
            let ty = parse_type(tokenizer)?;
            return Ok(TypeClause {
                slice: start.merge(ty.slice),
                exclude,
                ty: ClauseKind::RealType(ty),
            });
        }
    };
}

fn parse_enum_decl<'a>(
    tokenizer: &mut Tokenizer<'a>,
    slice: StringSlice<'a>,
    attrs: Option<Attrs<'a>>,
    generics: Option<GenericsDecl<'a>>,
    is_pub: bool,
) -> DeclResult<'a> {
    let ident = tokenizer.next()?;

    let TokenKind::Identifier(name) = ident.kind else {
        return Err(ParserError::unexpected_token(ident));
    };

    let peek = tokenizer.peek(0)?;

    match peek.kind {
        TokenKind::Symbol(Symbol::Colon) => {
            tokenizer.next()?;

            let next = tokenizer.next()?;

            let Some(ty) = IntEnumType::from(next.kind.clone()) else {
                return Err(ParserError::unexpected_token(next));
            };

            let body = parse_int_enum_body(tokenizer)?;

            return Ok(Decl {
                slice: slice.merge(body.slice),
                attrs,
                generics,
                is_pub,
                kind: DeclKind::IntEnum { name, ty, body },
            });
        }
        TokenKind::Symbol(Symbol::BraceOpen) => {
            let body = parse_struct_body(tokenizer)?;

            return Ok(Decl {
                slice: slice.merge(body.slice),
                attrs,
                generics,
                is_pub,
                kind: DeclKind::Enum { name, body },
            });
        }
        _ => return Err(ParserError::unexpected_token(peek)),
    }
}

pub fn parse_int_enum_body<'a>(
    tokenizer: &mut Tokenizer<'a>,
) -> Result<IntEnumBody<'a>, ParserError<'a>> {
    let next = tokenizer.peek(0)?;
    let TokenKind::Symbol(Symbol::BraceOpen) = next.kind else {
        return Err(ParserError::unexpected_token(next));
    };

    let start = next.slice;

    let mut params = vec![];

    if let TokenKind::Symbol(Symbol::BraceClose) = tokenizer.peek(1)?.kind {
        tokenizer.next()?;

        let last = tokenizer.next()?.slice;

        return Ok(IntEnumBody {
            slice: start.merge(last),
            params,
        });
    }

    loop {
        tokenizer.next()?;

        let next = tokenizer.next()?;
        let TokenKind::Identifier(name) = next.kind else {
            return Err(ParserError::unexpected_token(next));
        };
        let start = next.slice;

        let peek = tokenizer.peek(0)?;
        if let TokenKind::Symbol(Symbol::Assign) = peek.kind {
            tokenizer.next()?;

            let peek = tokenizer.peek(0)?;

            let Some(expr) = parse_expr(tokenizer)? else {
                return Err(ParserError::unexpected_token(peek));
            };

            params.push(IntEnumParam {
                slice: start.merge(expr.slice),
                name,
                value: Some(expr),
            });
        } else {
            params.push(IntEnumParam {
                slice: start,
                name,
                value: None,
            });
        }

        let next = tokenizer.peek(0)?;
        match next.kind {
            TokenKind::Symbol(Symbol::Comma) => {
                if let TokenKind::Symbol(Symbol::BraceClose) = tokenizer.peek(1)?.kind {
                    tokenizer.next()?;
                    break;
                }
            }
            TokenKind::Symbol(Symbol::BraceClose) => break,
            _ => return Err(ParserError::unexpected_token(next)),
        }
    }

    let next = tokenizer.next()?;

    return Ok(IntEnumBody {
        slice: start.merge(next.slice),
        params,
    });
}

fn parse_struct_decl<'a>(
    tokenizer: &mut Tokenizer<'a>,
    slice: StringSlice<'a>,
    attrs: Option<Attrs<'a>>,
    generics: Option<GenericsDecl<'a>>,
    is_pub: bool,
) -> DeclResult<'a> {
    let ident = tokenizer.next()?;

    let TokenKind::Identifier(name) = ident.kind else {
        return Err(ParserError::unexpected_token(ident));
    };

    let peek = tokenizer.peek(0)?;

    match peek.kind {
        TokenKind::Symbol(Symbol::Colon) => {
            tokenizer.next()?;
            let ty = parse_type(tokenizer)?;
            let next = tokenizer.next()?;
            let TokenKind::Symbol(Symbol::Semicolon) = next.kind else {
                return Err(ParserError::unexpected_token(next));
            };
            return Ok(Decl {
                slice: slice.merge(next.slice),
                attrs,
                generics,
                is_pub,
                kind: DeclKind::WrapperStruct { name, ty },
            });
        }
        TokenKind::Symbol(Symbol::BraceOpen) => {
            let body = parse_struct_body(tokenizer)?;

            return Ok(Decl {
                slice: slice.merge(body.slice),
                attrs,
                generics,
                is_pub,
                kind: DeclKind::Struct { name, body },
            });
        }
        _ => return Err(ParserError::unexpected_token(peek)),
    }
}

pub fn parse_struct_body<'a>(
    tokenizer: &mut Tokenizer<'a>,
) -> Result<StructBody<'a>, ParserError<'a>> {
    let next = tokenizer.peek(0)?;
    let TokenKind::Symbol(Symbol::BraceOpen) = next.kind else {
        return Err(ParserError::unexpected_token(next));
    };

    let start = next.slice;

    let mut params = vec![];

    if let TokenKind::Symbol(Symbol::BraceClose) = tokenizer.peek(1)?.kind {
        tokenizer.next()?;

        let last = tokenizer.next()?.slice;

        return Ok(StructBody {
            slice: start.merge(last),
            params,
        });
    }

    loop {
        tokenizer.next()?;
        let peek = tokenizer.peek(0)?;
        let start = next.slice;

        let is_pub = if let TokenKind::Keyword(Keyword::Pub) = peek.kind {
            tokenizer.next()?;
            true
        } else {
            false
        };

        let next = tokenizer.next()?;
        let TokenKind::Identifier(name) = next.kind else {
            return Err(ParserError::unexpected_token(next));
        };

        let next = tokenizer.next()?;
        let TokenKind::Symbol(Symbol::Colon) = next.kind else {
            return Err(ParserError::unexpected_token(next));
        };

        let ty = parse_type(tokenizer)?;

        params.push(StructParam {
            slice: start.merge(ty.slice),
            is_pub,
            name,
            ty,
        });

        let next = tokenizer.peek(0)?;
        match next.kind {
            TokenKind::Symbol(Symbol::Comma) => {
                if let TokenKind::Symbol(Symbol::BraceClose) = tokenizer.peek(1)?.kind {
                    tokenizer.next()?;
                    break;
                }
            }
            TokenKind::Symbol(Symbol::BraceClose) => break,
            _ => return Err(ParserError::unexpected_token(next)),
        }
    }

    let last = tokenizer.next()?.slice;

    return Ok(StructBody {
        slice: start.merge(last),
        params,
    });
}

fn parse_func_decl<'a>(
    tokenizer: &mut Tokenizer<'a>,
    slice: StringSlice<'a>,
    attrs: Option<Attrs<'a>>,
    generics: Option<GenericsDecl<'a>>,
    modifier: FunctionModifier,
    is_pub: bool,
) -> DeclResult<'a> {
    let ident = tokenizer.next()?;

    let TokenKind::Identifier(name) = ident.kind else {
        return Err(ParserError::unexpected_token(ident));
    };

    let peek = tokenizer.next()?;

    let TokenKind::Symbol(Symbol::ParenOpen) = peek.kind else {
        return Err(ParserError::unexpected_token(peek));
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
                return Err(ParserError::unexpected_token(next));
            };

            let next = tokenizer.next()?;

            let TokenKind::Symbol(Symbol::Colon) = next.kind else {
                return Err(ParserError::unexpected_token(next));
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
                TokenKind::Symbol(Symbol::Comma) => {
                    if let TokenKind::Symbol(Symbol::ParenClose) = tokenizer.peek(0)?.kind {
                        tokenizer.next()?;
                        break;
                    }
                }
                TokenKind::Symbol(Symbol::ParenClose) => break,
                _ => return Err(ParserError::unexpected_token(next)),
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
                return Err(ParserError::unexpected_token(t));
            };
            let next = tokenizer.next()?;
            let TokenKind::Symbol(Symbol::Semicolon) = next.kind else {
                return Err(ParserError::unexpected_token(next));
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
        _ => return Err(ParserError::unexpected_token(next)),
    };

    Ok(Decl {
        slice: slice.merge(end),
        attrs,
        generics,
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
            return Err(ParserError::unexpected_token(next));
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
    generics: Option<GenericsDecl<'a>>,
    modifier: VariableModifier,
    is_pub: bool,
) -> DeclResult<'a> {
    let ident = tokenizer.next()?;

    let name = match ident.kind {
        TokenKind::Identifier(ident) => VariableName::Identifier(ident),
        TokenKind::Keyword(Keyword::Discard) => VariableName::Discard,
        _ => return Err(ParserError::unexpected_token(ident)),
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
                return Err(ParserError::unexpected_token(tokenizer.peek(0)?));
            };

            let peek = tokenizer.peek(0)?;
            let end = peek.slice;

            let TokenKind::Symbol(Symbol::Semicolon) = peek.kind else {
                return Err(ParserError::unexpected_token(peek));
            };
            tokenizer.next()?;

            return Ok(Decl {
                slice: slice.merge(end),
                attrs,
                generics,
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
                generics,
                is_pub,
                kind: DeclKind::Variable {
                    modifier,
                    name,
                    ty,
                    init: None,
                },
            });
        }
        _ => return Err(ParserError::unexpected_token(peek)),
    }
}
