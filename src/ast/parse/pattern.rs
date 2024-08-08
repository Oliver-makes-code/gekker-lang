use crate::{
    ast::pattern::{Pattern, PatternKind},
    tokenizer::{
        token::{Keyword, TokenKind},
        Tokenizer,
    },
};

use super::error::ParserError;

type PatternResult<'a> = Result<Pattern<'a>, ParserError<'a>>;

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
                kind: PatternKind::Discard,
            });
        }
        TokenKind::Keyword(Keyword::Invalid) => {
            tokenizer.next()?;
            return Ok(Pattern {
                slice: peek.slice,
                kind: PatternKind::Discard,
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
        _ => {}
    }
    todo!()
}
