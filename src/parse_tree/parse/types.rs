use crate::{
    parse_tree::{
        types::{RefKind, Type, TypeKind},
        IdentPath,
    },
    tokenizer::{
        token::{Keyword, Number, Symbol, TokenKind},
        Tokenizer,
    },
};

use super::{decl::parse_struct_body, error::ParserError};

pub fn parse_type<'a>(tokenizer: &mut Tokenizer<'a>) -> Result<Type<'a>, ParserError<'a>> {
    let peek = tokenizer.peek(0)?;

    if let Some(primitive) = TypeKind::try_from_primitive(peek.kind.clone()) {
        tokenizer.next()?;
        return Ok(Type {
            slice: peek.slice,
            kind: primitive,
        });
    }

    if let Some(idents) = IdentPath::try_parse(tokenizer)? {
        let start = idents.slice;
        let peek = tokenizer.peek(0)?;
        if let TokenKind::Symbol(Symbol::Colon) = peek.kind {
            tokenizer.next()?;

            let next = tokenizer.next()?;
            let TokenKind::Symbol(Symbol::Less) = next.kind else {
                return Err(ParserError::unexpected_token(next));
            };

            let peek = tokenizer.peek(0)?;

            let TokenKind::Symbol(Symbol::Greater) = peek.kind else {
                let mut params = vec![];

                loop {
                    let ty = parse_type(tokenizer)?;
                    params.push(ty);

                    let next = tokenizer.next()?;
                    match next.kind {
                        TokenKind::Symbol(Symbol::Comma) => {}
                        TokenKind::Symbol(Symbol::Greater) => {
                            return Ok(Type {
                                slice: start.merge(next.slice),
                                kind: TypeKind::UserDefined {
                                    path: idents,
                                    generics: params,
                                },
                            });
                        }
                        _ => return Err(ParserError::unexpected_token(next)),
                    }
                }
            };
            tokenizer.next()?;

            let end = peek.slice;

            return Ok(Type {
                slice: start.merge(end),
                kind: TypeKind::UserDefined {
                    path: idents,
                    generics: vec![],
                },
            });
        }
        return Ok(Type {
            slice: start,
            kind: TypeKind::UserDefined {
                path: idents,
                generics: vec![],
            },
        });
    }

    if let Some((slice, ref_kind)) = RefKind::parse(tokenizer)? {
        let referenced = parse_type(tokenizer)?;
        return Ok(Type {
            slice: slice.merge(referenced.slice),
            kind: TypeKind::Ref {
                ref_kind,
                ty: Box::new(referenced),
            },
        });
    }

    match peek.kind {
        TokenKind::Symbol(Symbol::Optional) => {
            tokenizer.next()?;
            let start = peek.slice;
            let value = parse_type(tokenizer)?;
            return Ok(Type {
                slice: start.merge(value.slice),
                kind: TypeKind::Option(Box::new(value)),
            });
        }
        TokenKind::Symbol(Symbol::Range) => {
            tokenizer.next()?;
            let start = peek.slice;
            let value = parse_type(tokenizer)?;
            return Ok(Type {
                slice: start.merge(value.slice),
                kind: TypeKind::Range(Box::new(value)),
            });
        }
        TokenKind::Symbol(Symbol::BracketOpen) => {
            tokenizer.next()?;
            let start = peek.slice;
            let value = parse_type(tokenizer)?;

            let peek = tokenizer.peek(0)?;

            match peek.kind {
                TokenKind::Symbol(Symbol::Comma) => {
                    tokenizer.next()?;
                    let next = tokenizer.next()?;
                    let TokenKind::Number(Number {
                        whole: count,
                        decimal: 0.0,
                    }) = next.kind
                    else {
                        return Err(ParserError::unexpected_token(next));
                    };

                    let next = tokenizer.next()?;
                    let TokenKind::Symbol(Symbol::BracketClose) = next.kind else {
                        return Err(ParserError::unexpected_token(next));
                    };

                    return Ok(Type {
                        slice: start.merge(next.slice),
                        kind: TypeKind::Array {
                            ty: Box::new(value),
                            len: count as usize,
                        },
                    });
                }
                TokenKind::Symbol(Symbol::BracketClose) => {
                    tokenizer.next()?;
                    return Ok(Type {
                        slice: start.merge(peek.slice),
                        kind: TypeKind::Slice(Box::new(value)),
                    });
                }
                _ => return Err(ParserError::unexpected_token(peek)),
            }
        }
        TokenKind::Keyword(Keyword::Func) => {
            tokenizer.next()?;
            let start = peek.slice;

            let peek = tokenizer.next()?;
            let TokenKind::Symbol(Symbol::ParenOpen) = peek.kind else {
                return Err(ParserError::unexpected_token(peek));
            };
            let mut peek = tokenizer.peek(0)?;
            let mut params = vec![];

            println!("{:?}", peek);

            while peek.kind != TokenKind::Symbol(Symbol::ParenClose) {
                params.push(parse_type(tokenizer)?);

                peek = tokenizer.next()?;

                let TokenKind::Symbol(Symbol::Comma | Symbol::ParenClose) = peek.kind else {
                    return Err(ParserError::unexpected_token(peek));
                };
            }

            let end = peek.slice;

            let peek = tokenizer.peek(0)?;

            let TokenKind::Symbol(Symbol::Colon) = peek.kind else {
                return Ok(Type {
                    slice: start.merge(end),
                    kind: TypeKind::Func { params, ret: None },
                });
            };
            tokenizer.next()?;

            let ret = parse_type(tokenizer)?;
            let end = ret.slice;

            return Ok(Type {
                slice: start.merge(end),
                kind: TypeKind::Func {
                    params,
                    ret: Some(Box::new(ret)),
                },
            });
        }
        TokenKind::Keyword(Keyword::Struct) => {
            tokenizer.next()?;
            let start = peek.slice;

            let body = parse_struct_body(tokenizer)?;

            return Ok(Type {
                slice: start.merge(body.slice),
                kind: TypeKind::Struct(body),
            });
        }
        _ => return Err(ParserError::unexpected_token(peek)),
    }
}
