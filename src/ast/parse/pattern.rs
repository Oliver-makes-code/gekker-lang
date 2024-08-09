use crate::{
    ast::{
        pattern::{Pattern, PatternKind},
        IdentPath,
    },
    tokenizer::{
        token::{Keyword, Symbol, TokenKind},
        Tokenizer,
    },
};

use super::error::ParserError;

type PatternResult<'a> = Result<Pattern<'a>, ParserError<'a>>;

pub fn parse_pattern<'a>(tokenizer: &mut Tokenizer<'a>) -> PatternResult<'a> {
    let base = parse_value(tokenizer)?;

    let peek = tokenizer.peek(0)?;
    let TokenKind::Symbol(Symbol::BitOr) = peek.kind else {
        return Ok(base);
    };
    tokenizer.next()?;

    let start = base.slice;

    let mut values = vec![base];

    loop {
        let pattern = parse_value(tokenizer)?;
        let end = pattern.slice.clone();

        values.push(pattern);

        let peek = tokenizer.peek(0)?;
        let TokenKind::Symbol(Symbol::BitOr) = peek.kind else {
            return Ok(Pattern {
                slice: start.merge(end),
                kind: PatternKind::Or(values),
            });
        };
        tokenizer.next()?;
    }
}

fn parse_value<'a>(tokenizer: &mut Tokenizer<'a>) -> PatternResult<'a> {
    let peek = tokenizer.peek(0)?;
    match peek.kind {
        TokenKind::Keyword(Keyword::Discard) => {
            tokenizer.next()?;
            return Ok(Pattern {
                slice: peek.slice,
                kind: PatternKind::Discard,
            });
        }
        TokenKind::Keyword(Keyword::Nullptr) => {
            tokenizer.next()?;
            return Ok(Pattern {
                slice: peek.slice,
                kind: PatternKind::Nullptr,
            });
        }
        TokenKind::Keyword(Keyword::Invalid) => {
            tokenizer.next()?;
            return Ok(Pattern {
                slice: peek.slice,
                kind: PatternKind::Invalid,
            });
        }
        TokenKind::Char(c) => {
            tokenizer.next()?;
            return Ok(Pattern {
                slice: peek.slice,
                kind: PatternKind::Char(c),
            });
        }
        TokenKind::String(s) => {
            tokenizer.next()?;
            return Ok(Pattern {
                slice: peek.slice,
                kind: PatternKind::String(s),
            });
        }
        TokenKind::Number(n) => {
            tokenizer.next()?;
            return Ok(Pattern {
                slice: peek.slice,
                kind: PatternKind::Number(n),
            });
        }
        TokenKind::Keyword(Keyword::True) => {
            tokenizer.next()?;
            return Ok(Pattern {
                slice: peek.slice,
                kind: PatternKind::Bool(true),
            });
        }
        TokenKind::Keyword(Keyword::False) => {
            tokenizer.next()?;
            return Ok(Pattern {
                slice: peek.slice,
                kind: PatternKind::Bool(false),
            });
        }
        TokenKind::Keyword(Keyword::Mut) => {
            tokenizer.next()?;
            let next = tokenizer.next()?;
            let TokenKind::Identifier(name) = next.kind else {
                return Err(ParserError::UnexpectedToken(next));
            };

            return Ok(Pattern {
                slice: peek.slice.merge(next.slice),
                kind: PatternKind::Value { is_mut: true, name },
            });
        }
        TokenKind::Identifier(name) => {
            let slice = peek.slice;
            let peek = tokenizer.peek(1)?;
            if let TokenKind::Symbol(Symbol::DoubleColon | Symbol::BraceOpen) = peek.kind {
                let Some(path) = IdentPath::try_parse(tokenizer)? else {
                    return Err(ParserError::UnexpectedToken(peek));
                };

                todo!("initializer list pattern");
            }
            tokenizer.next()?;

            return Ok(Pattern {
                slice,
                kind: PatternKind::Value {
                    is_mut: false,
                    name,
                },
            });
        }
        a => todo!("{a:?}"),
    }
}
