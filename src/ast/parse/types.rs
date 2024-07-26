use crate::{
    ast::{
        types::{RefKind, Type, TypeKind},
        IdentPath,
    },
    tokenizer::{
        token::{Keyword, Number, Symbol, TokenKind},
        Tokenizer,
    },
};

use super::error::ParserError;

pub fn parse_root<'a>(tokenizer: &mut Tokenizer<'a>) -> Result<Type<'a>, ParserError<'a>> {
    let peek = tokenizer.peek(0)?;

    if let Some(primitive) = TypeKind::try_from_primitive(peek.kind.clone()) {
        tokenizer.next()?;
        return Ok(Type {
            slice: peek.slice,
            kind: primitive,
        });
    }

    if let Some((start, idents)) = IdentPath::try_parse(tokenizer)? {
        let peek = tokenizer.peek(0)?;
        if let TokenKind::Symbol(Symbol::Less) = peek.kind {
            let mut peek = peek;

            let mut params = vec![];

            while peek.kind != TokenKind::Symbol(Symbol::Greater) {
                tokenizer.next()?;

                params.push(parse_root(tokenizer)?);

                peek = tokenizer.peek(0)?;

                let TokenKind::Symbol(Symbol::Comma | Symbol::Greater) = peek.kind else {
                    return Err(ParserError::UnexpectedToken(peek, "Comma or Greater"));
                };
            }

            let end = peek.slice;

            return Ok(Type {
                slice: start.merge(end),
                kind: TypeKind::UserDefined(idents, params),
            });
        }
        return Ok(Type {
            slice: start,
            kind: TypeKind::UserDefined(idents, vec![]),
        });
    }

    match peek.kind {
        TokenKind::Keyword(Keyword::Ref) => {
            tokenizer.next()?;
            let start = peek.slice;

            let ref_kind = if let TokenKind::Keyword(Keyword::Mut) = tokenizer.peek(0)?.kind {
                tokenizer.next()?;
                RefKind::Mutable
            } else {
                RefKind::Immutable
            };

            let referenced = parse_root(tokenizer)?;

            return Ok(Type {
                slice: start.merge(referenced.slice),
                kind: TypeKind::Ref(ref_kind, Box::new(referenced)),
            });
        }
        TokenKind::Symbol(Symbol::Mul) => {
            tokenizer.next()?;
            let start = peek.slice;
            let referenced = parse_root(tokenizer)?;
            return Ok(Type {
                slice: start.merge(referenced.slice),
                kind: TypeKind::Ref(RefKind::Pointer, Box::new(referenced)),
            });
        }
        TokenKind::Symbol(Symbol::Optional) => {
            tokenizer.next()?;
            let start = peek.slice;
            let value = parse_root(tokenizer)?;
            return Ok(Type {
                slice: start.merge(value.slice),
                kind: TypeKind::Option(Box::new(value)),
            });
        }
        TokenKind::Symbol(Symbol::Range) => {
            tokenizer.next()?;
            let start = peek.slice;
            let value = parse_root(tokenizer)?;
            return Ok(Type {
                slice: start.merge(value.slice),
                kind: TypeKind::Range(Box::new(value)),
            });
        }
        TokenKind::Symbol(Symbol::BracketOpen) => {
            tokenizer.next()?;
            let start = peek.slice;
            let value = parse_root(tokenizer)?;

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
                        return Err(ParserError::UnexpectedToken(next, "Integer"));
                    };

                    let next = tokenizer.next()?;
                    let TokenKind::Symbol(Symbol::BracketClose) = next.kind else {
                        return Err(ParserError::UnexpectedToken(next, "Bracket close"));
                    };

                    return Ok(Type {
                        slice: start.merge(next.slice),
                        kind: TypeKind::Array(Box::new(value), count as usize),
                    });
                }
                TokenKind::Symbol(Symbol::BracketClose) => {
                    tokenizer.next()?;
                    return Ok(Type {
                        slice: start.merge(peek.slice),
                        kind: TypeKind::Slice(Box::new(value)),
                    });
                }
                _ => return Err(ParserError::UnexpectedToken(peek, "Comma or Bracket close")),
            }
        }
        TokenKind::Keyword(Keyword::Func) => {
            tokenizer.next()?;
            let start = peek.slice;

            let mut peek = tokenizer.peek(0)?;
            let TokenKind::Symbol(Symbol::ParenOpen) = peek.kind else {
                return Err(ParserError::UnexpectedToken(peek, "Paren open"));
            };
            let mut params = vec![];

            while peek.kind != TokenKind::Symbol(Symbol::ParenClose) {
                tokenizer.next()?;

                params.push(parse_root(tokenizer)?);

                peek = tokenizer.peek(0)?;

                let TokenKind::Symbol(Symbol::Comma | Symbol::ParenClose) = peek.kind else {
                    return Err(ParserError::UnexpectedToken(peek, "Comma or Paren close"));
                };
            }
            tokenizer.next()?;

            let end = peek.slice;

            let peek = tokenizer.peek(0)?;

            let TokenKind::Symbol(Symbol::Colon) = peek.kind else {
                return Ok(Type {
                    slice: start.merge(end),
                    kind: TypeKind::Func(params, None),
                });
            };
            tokenizer.next()?;

            let ret = parse_root(tokenizer)?;
            let end = ret.slice;

            return Ok(Type {
                slice: start.merge(end),
                kind: TypeKind::Func(params, Some(Box::new(ret))),
            });
        }
        _ => return Err(ParserError::UnexpectedToken(peek, "Type")),
    }
}
