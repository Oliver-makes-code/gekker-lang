use crate::{
    parse_tree::{
        pattern::{
            InitializerPattern, InitializerPatternKind, NamedInitializerPattern, Pattern,
            PatternKind,
        },
        IdentPath,
    },
    tokenizer::{
        token::{Keyword, Symbol, TokenKind},
        Tokenizer,
    },
};

use super::{error::ParserError, expr::parse_generics_instance};

type PatternResult = Result<Pattern, ParserError>;

pub fn parse_pattern(tokenizer: &mut Tokenizer) -> PatternResult {
    let base = parse_value(tokenizer)?;

    let peek = tokenizer.peek(0)?;
    let TokenKind::Symbol(Symbol::BitOr) = peek.kind else {
        return Ok(base);
    };
    tokenizer.next()?;

    let start = base.slice.clone();

    let mut values = vec![base];

    loop {
        let pattern = parse_value(tokenizer)?;
        let end = pattern.slice.clone();

        values.push(pattern);

        let peek = tokenizer.peek(0)?;
        let TokenKind::Symbol(Symbol::BitOr) = peek.kind else {
            return Ok(Pattern {
                slice: start.merge(&end),
                kind: PatternKind::Or(values),
            });
        };
        tokenizer.next()?;
    }
}

/// TODO: Parse array initializer
fn parse_value(tokenizer: &mut Tokenizer) -> PatternResult {
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
        TokenKind::Keyword(Keyword::Default) => {
            tokenizer.next()?;
            return Ok(Pattern {
                slice: peek.slice,
                kind: PatternKind::Default,
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
                return Err(ParserError::unexpected_token(next));
            };

            return Ok(Pattern {
                slice: peek.slice.merge(&next.slice),
                kind: PatternKind::Value { is_mut: true, name },
            });
        }
        TokenKind::Identifier(name) => {
            let slice = peek.slice;
            let peek = tokenizer.peek(1)?;
            if let TokenKind::Symbol(Symbol::DoubleColon | Symbol::Colon | Symbol::BraceOpen) =
                peek.kind
            {
                let Some(name) = IdentPath::try_parse(tokenizer)? else {
                    return Err(ParserError::unexpected_token(peek));
                };

                let generics = parse_generics_instance(tokenizer)?;

                let list = parse_initializer_pattern(tokenizer)?;

                return Ok(Pattern {
                    slice: slice.merge(&list.slice),
                    kind: PatternKind::Initializer {
                        name,
                        generics,
                        list,
                    },
                });
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
        _ => return Err(ParserError::unexpected_token(peek)),
    }
}

fn parse_initializer_pattern(tokenizer: &mut Tokenizer) -> Result<InitializerPattern, ParserError> {
    let next = tokenizer.next()?;
    let TokenKind::Symbol(Symbol::BraceOpen) = next.kind else {
        return Err(ParserError::unexpected_token(next));
    };
    let start = next.slice;

    let peek = tokenizer.peek(0)?;

    match peek.kind {
        TokenKind::Symbol(Symbol::BraceClose) => {
            tokenizer.next()?;
            return Ok(InitializerPattern {
                slice: start.merge(&peek.slice),
                kind: InitializerPatternKind::Empty,
            });
        }
        TokenKind::Symbol(Symbol::Dot) => {
            let mut values = vec![];

            loop {
                let init = parse_named_initializer_pattern(tokenizer)?;
                values.push(init);

                let peek = tokenizer.peek(0)?;
                match peek.kind {
                    TokenKind::Symbol(Symbol::Comma) => {
                        tokenizer.next()?;
                        let peek = tokenizer.peek(0)?;
                        if let TokenKind::Symbol(Symbol::BraceClose) = peek.kind {
                            break;
                        }
                    }
                    TokenKind::Symbol(Symbol::BraceClose) => {
                        break;
                    }
                    _ => return Err(ParserError::unexpected_token(peek)),
                }
            }

            let next = tokenizer.next()?;
            let TokenKind::Symbol(Symbol::BraceClose) = next.kind else {
                return Err(ParserError::unexpected_token(next));
            };

            return Ok(InitializerPattern {
                slice: start.merge(&next.slice),
                kind: InitializerPatternKind::Named(values),
            });
        }
        _ => {
            let mut values = vec![];

            loop {
                let value = parse_pattern(tokenizer)?;
                values.push(value);

                let peek = tokenizer.peek(0)?;
                match peek.kind {
                    TokenKind::Symbol(Symbol::Comma) => {
                        tokenizer.next()?;
                        let peek = tokenizer.peek(0)?;
                        if let TokenKind::Symbol(Symbol::BraceClose) = peek.kind {
                            break;
                        }
                    }
                    TokenKind::Symbol(Symbol::BraceClose) => {
                        break;
                    }
                    _ => return Err(ParserError::unexpected_token(peek)),
                }
            }

            let next = tokenizer.next()?;
            let TokenKind::Symbol(Symbol::BraceClose) = next.kind else {
                return Err(ParserError::unexpected_token(next));
            };

            return Ok(InitializerPattern {
                slice: start.merge(&next.slice),
                kind: InitializerPatternKind::Expr(values),
            });
        }
    }
}

fn parse_named_initializer_pattern(
    tokenizer: &mut Tokenizer,
) -> Result<NamedInitializerPattern, ParserError> {
    let next = tokenizer.next()?;
    let TokenKind::Symbol(Symbol::Dot) = next.kind else {
        return Err(ParserError::unexpected_token(next));
    };
    let start = next.slice;

    let next = tokenizer.next()?;
    let TokenKind::Identifier(name) = next.kind else {
        return Err(ParserError::unexpected_token(next));
    };

    let next = tokenizer.next()?;
    let TokenKind::Symbol(Symbol::Assign) = next.kind else {
        return Err(ParserError::unexpected_token(next));
    };

    let value = parse_pattern(tokenizer)?;

    return Ok(NamedInitializerPattern {
        slice: start.merge(&value.slice),
        name,
        value,
    });
}
