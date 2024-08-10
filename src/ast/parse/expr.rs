use crate::{
    ast::{
        expr::{
            AccessKind, BinOp, DefaultedInitializer, Expr, ExprKind, GenericsInstance,
            InitializerKind, InitializerList, NamedInitializer, UnaryOp,
        },
        parse::types::parse_type,
        IdentPath,
    },
    tokenizer::{
        token::{Keyword, Symbol, Token, TokenKind},
        Tokenizer,
    },
};

use super::error::ParserError;

type ExprResult<'a> = Result<Option<Expr<'a>>, ParserError<'a>>;

pub fn parse_expr<'a>(tokenizer: &mut Tokenizer<'a>) -> ExprResult<'a> {
    return parse_operators(tokenizer, 0);
}

/// Pratt parsing!! Yippee!!!!
fn parse_operators<'a>(tokenizer: &mut Tokenizer<'a>, binding: usize) -> ExprResult<'a> {
    let Some(mut expr) = parse_unary(tokenizer)? else {
        return Ok(None);
    };

    loop {
        let peek = tokenizer.peek(0)?;
        let Some(op) = BinOp::try_parse(peek.kind.clone()) else {
            break;
        };

        let (lhs_binding, rhs_binding) = op.binding();
        if lhs_binding < binding {
            break;
        }
        tokenizer.next()?;

        let Some(rhs) = parse_operators(tokenizer, rhs_binding)? else {
            return Err(ParserError::UnexpectedToken(tokenizer.peek(0)?));
        };
        let slice = expr.slice.merge(rhs.slice);

        expr = Expr {
            slice,
            kind: ExprKind::BinOp {
                lhs: Box::new(expr),
                op,
                rhs: Box::new(rhs),
            },
        };
    }

    return Ok(Some(expr));
}

fn parse_unary<'a>(tokenizer: &mut Tokenizer<'a>) -> ExprResult<'a> {
    let mut unary_ops = vec![];
    let mut peek = tokenizer.peek(0)?;
    while let Some(op) = UnaryOp::try_parse(peek.kind) {
        tokenizer.next()?;
        unary_ops.push((peek.slice, op));
        peek = tokenizer.peek(0)?;
    }

    let Some(mut expr) = parse_access(tokenizer)? else {
        return Ok(None);
    };

    while let Some((slice, op)) = unary_ops.pop() {
        expr = Expr {
            slice: slice.merge(expr.slice),
            kind: ExprKind::UnaryOp {
                op,
                value: Box::new(expr),
            },
        };
    }

    return Ok(Some(expr));
}

fn parse_access<'a>(tokenizer: &mut Tokenizer<'a>) -> ExprResult<'a> {
    let Some(mut expr) = parse_cast(tokenizer)? else {
        return Ok(None);
    };

    while let Some(access) = parse_access_arm(tokenizer, expr.clone())? {
        expr = access;
    }

    return Ok(Some(expr));
}

fn parse_cast<'a>(tokenizer: &mut Tokenizer<'a>) -> ExprResult<'a> {
    let Some(atom) = parse_atom(tokenizer)? else {
        return Ok(None);
    };

    let peek = tokenizer.peek(0)?;

    let TokenKind::Symbol(Symbol::Colon) = peek.kind else {
        return Ok(Some(atom));
    };

    tokenizer.next()?;

    let ty = parse_type(tokenizer)?;

    return Ok(Some(Expr {
        slice: atom.slice.merge(ty.slice),
        kind: ExprKind::Cast {
            value: Box::new(atom),
            ty,
        },
    }));
}

fn parse_access_arm<'a>(tokenizer: &mut Tokenizer<'a>, expr: Expr<'a>) -> ExprResult<'a> {
    let next = tokenizer.peek(0)?;

    if let Some(kind) = AccessKind::try_parse(next.kind.clone()) {
        tokenizer.next()?;
        let next = tokenizer.next()?;

        let TokenKind::Identifier(ident) = next.kind else {
            return Err(ParserError::UnexpectedToken(next));
        };

        let generics = parse_generics_instance(tokenizer)?;

        let slice = if let Some(g) = generics.clone() {
            g.slice
        } else {
            next.slice
        };

        return Ok(Some(Expr {
            slice: expr.slice.merge(slice),
            kind: ExprKind::Field {
                value: Box::new(expr),
                access: kind,
                field: ident,
                generics,
            },
        }));
    }

    if let TokenKind::Symbol(Symbol::ParenOpen) = next.kind {
        tokenizer.next()?;
        let mut exprs = vec![];

        let peek = tokenizer.peek(0)?;

        if peek.kind != TokenKind::Symbol(Symbol::ParenClose) {
            let start = expr.slice.clone();
            let value = expr;
            loop {
                let t = tokenizer.peek(0)?;
                let Some(expr) = parse_expr(tokenizer)? else {
                    return Err(ParserError::UnexpectedToken(t));
                };
                exprs.push(expr);

                let peek = tokenizer.next()?;

                match peek.kind {
                    TokenKind::Symbol(Symbol::Comma) => (),
                    TokenKind::Symbol(Symbol::ParenClose) => {
                        return Ok(Some(Expr {
                            slice: start.merge(peek.slice),
                            kind: ExprKind::Invoke {
                                value: Box::new(value),
                                params: exprs,
                            },
                        }))
                    }
                    _ => return Err(ParserError::UnexpectedToken(peek)),
                }

                let TokenKind::Symbol(Symbol::Comma | Symbol::ParenClose) = peek.kind else {
                    return Err(ParserError::UnexpectedToken(peek));
                };
            }
        }
        tokenizer.next()?;

        return Ok(Some(Expr {
            slice: expr.slice.merge(peek.slice),
            kind: ExprKind::Invoke {
                value: Box::new(expr),
                params: exprs,
            },
        }));
    }

    let TokenKind::Symbol(Symbol::BracketOpen) = next.kind else {
        return Ok(None);
    };
    tokenizer.next()?;

    let Some(index) = parse_expr(tokenizer)? else {
        return Err(ParserError::UnexpectedToken(tokenizer.peek(0)?));
    };

    let next = tokenizer.next()?;

    let TokenKind::Symbol(Symbol::BracketClose) = next.kind else {
        return Err(ParserError::UnexpectedToken(next));
    };

    let slice = next.slice;

    return Ok(Some(Expr {
        slice: expr.slice.merge(slice),
        kind: ExprKind::Index {
            value: Box::new(expr),
            index: Box::new(index),
        },
    }));
}

/// TODO: Parse array initializer
fn parse_atom<'a>(tokenizer: &mut Tokenizer<'a>) -> ExprResult<'a> {
    if let Some(ident) = parse_ident(tokenizer)? {
        return Ok(Some(ident));
    }

    let token = tokenizer.peek(0)?;

    let slice = token.slice;

    let kind = match token.kind {
        TokenKind::Char(c) => ExprKind::Char(c),
        TokenKind::Number(n) => ExprKind::Number(n),
        TokenKind::String(s) => ExprKind::String(s),
        TokenKind::Keyword(Keyword::Discard) => ExprKind::Discard,
        TokenKind::Keyword(Keyword::ThisValue) => ExprKind::This,
        TokenKind::Keyword(Keyword::Default) => ExprKind::Default,
        TokenKind::Keyword(Keyword::True) => ExprKind::Bool(true),
        TokenKind::Keyword(Keyword::False) => ExprKind::Bool(false),
        TokenKind::Keyword(Keyword::Nullptr) => ExprKind::Nullptr,
        TokenKind::Keyword(Keyword::Sizeof) => {
            tokenizer.next()?;

            let next = tokenizer.next()?;

            match next.kind {
                TokenKind::Symbol(Symbol::Less) => {
                    let ty = parse_type(tokenizer)?;

                    let next = tokenizer.next()?;
                    let TokenKind::Symbol(Symbol::Greater) = next.kind else {
                        return Err(ParserError::UnexpectedToken(next));
                    };

                    return Ok(Some(Expr {
                        slice: slice.merge(next.slice),
                        kind: ExprKind::SizeofType(ty),
                    }));
                }
                TokenKind::Symbol(Symbol::ParenOpen) => {
                    let next = tokenizer.peek(0)?;
                    let Some(expr) = parse_expr(tokenizer)? else {
                        return Err(ParserError::UnexpectedToken(next));
                    };

                    let next = tokenizer.next()?;
                    let TokenKind::Symbol(Symbol::ParenClose) = next.kind else {
                        return Err(ParserError::UnexpectedToken(next));
                    };

                    return Ok(Some(Expr {
                        slice: slice.merge(next.slice),
                        kind: ExprKind::SizeofValue(Box::new(expr)),
                    }));
                }
                _ => return Err(ParserError::UnexpectedToken(next)),
            }
        }
        TokenKind::Symbol(Symbol::ParenOpen) => {
            tokenizer.next()?;

            let Some(expr) = parse_expr(tokenizer)? else {
                return Err(ParserError::UnexpectedToken(tokenizer.peek(0)?));
            };

            let next = tokenizer.next()?;

            let TokenKind::Symbol(Symbol::ParenClose) = next.kind else {
                return Err(ParserError::UnexpectedToken(next));
            };

            return Ok(Some(Expr {
                slice: slice.merge(next.slice),
                ..expr
            }));
        }
        _ => {
            return Ok(None);
        }
    };

    tokenizer.next()?;

    return Ok(Some(Expr { slice, kind }));
}

fn parse_ident<'a>(tokenizer: &mut Tokenizer<'a>) -> ExprResult<'a> {
    let Some(path) = IdentPath::try_parse(tokenizer)? else {
        return Ok(None);
    };

    let generics = parse_generics_instance(tokenizer)?;

    let slice = if let Some(g) = generics.clone() {
        path.slice.merge(g.slice)
    } else {
        path.slice
    };

    if let Some(list) = parse_initializer_list(tokenizer)? {
        return Ok(Some(Expr {
            slice: slice.merge(list.slice.clone()),
            kind: ExprKind::Initializer {
                path,
                generics,
                list,
            },
        }));
    }

    return Ok(Some(Expr {
        slice,
        kind: ExprKind::Variable { path, generics },
    }));
}

pub fn parse_generics_instance<'a>(
    tokenizer: &mut Tokenizer<'a>,
) -> Result<Option<GenericsInstance<'a>>, ParserError<'a>> {
    let peek = tokenizer.peek(0)?;
    let TokenKind::Symbol(Symbol::Colon) = peek.kind else {
        return Ok(None);
    };
    let start = peek.slice;
    tokenizer.next()?;

    let peek = tokenizer.peek(0)?;
    let TokenKind::Symbol(Symbol::Less) = peek.kind else {
        return Ok(None);
    };
    tokenizer.next()?;

    let mut params = vec![];

    let peek = tokenizer.peek(0)?;

    let TokenKind::Symbol(Symbol::Greater) = peek.kind else {
        loop {
            let param = parse_type(tokenizer)?;
            params.push(param);

            let next = tokenizer.next()?;

            match next.kind {
                TokenKind::Symbol(Symbol::Comma) => {
                    let peek = tokenizer.peek(0)?;
                    if let TokenKind::Symbol(Symbol::Greater) = peek.kind {
                        tokenizer.next()?;
                        return Ok(Some(GenericsInstance {
                            slice: start.merge(peek.slice),
                            params,
                        }));
                    }
                }
                TokenKind::Symbol(Symbol::Greater) => {
                    return Ok(Some(GenericsInstance {
                        slice: start.merge(next.slice),
                        params,
                    }));
                }
                _ => return Err(ParserError::UnexpectedToken(next)),
            }
        }
    };

    return Ok(Some(GenericsInstance {
        slice: start.merge(tokenizer.next()?.slice),
        params,
    }));
}

pub fn parse_initializer_list<'a>(
    tokenizer: &mut Tokenizer<'a>,
) -> Result<Option<InitializerList<'a>>, ParserError<'a>> {
    let peek = tokenizer.peek(0)?;
    let TokenKind::Symbol(Symbol::BraceOpen) = peek.kind else {
        return Ok(None);
    };
    tokenizer.next()?;
    let start = peek.slice;

    let peek = tokenizer.peek(0)?;

    match peek.kind {
        TokenKind::Symbol(Symbol::BraceClose) => {
            tokenizer.next()?;
            return Ok(Some(InitializerList {
                slice: start.merge(peek.slice),
                kind: InitializerKind::Empty,
            }));
        }
        TokenKind::Symbol(Symbol::Dot) => {
            let mut values = vec![];

            loop {
                let init = parse_named_initializer(tokenizer)?;
                values.push(init);

                let peek = tokenizer.peek(0)?;
                match peek.kind {
                    TokenKind::Symbol(Symbol::Comma) => {
                        tokenizer.next()?;
                        let peek = tokenizer.peek(0)?;
                        if let TokenKind::Symbol(Symbol::BraceClose | Symbol::Rest) = peek.kind {
                            break;
                        }
                    }
                    TokenKind::Symbol(Symbol::BraceClose | Symbol::Rest) => {
                        break;
                    }
                    _ => return Err(ParserError::UnexpectedToken(peek)),
                }
            }

            let default = parse_defaulted_initializer(tokenizer)?;

            let peek = tokenizer.peek(0)?;
            if default.is_some()
                && let TokenKind::Symbol(Symbol::Comma) = peek.kind
            {
                tokenizer.next()?;
            }

            let next = tokenizer.next()?;
            let TokenKind::Symbol(Symbol::BraceClose) = next.kind else {
                return Err(ParserError::UnexpectedToken(next));
            };

            return Ok(Some(InitializerList {
                slice: start.merge(next.slice),
                kind: InitializerKind::Named { values, default },
            }));
        }
        _ => {
            let mut values = vec![];
            loop {
                let peek = tokenizer.peek(0)?;
                let Some(value) = parse_expr(tokenizer)? else {
                    return Err(ParserError::UnexpectedToken(peek));
                };
                values.push(value);

                let next = tokenizer.next()?;
                match next.kind {
                    TokenKind::Symbol(Symbol::BraceClose) => {
                        return Ok(Some(InitializerList {
                            slice: start.merge(next.slice),
                            kind: InitializerKind::Expr(values),
                        }))
                    }
                    TokenKind::Symbol(Symbol::Comma) => {
                        let peek = tokenizer.peek(0)?;
                        if let TokenKind::Symbol(Symbol::BraceClose) = peek.kind {
                            tokenizer.next()?;
                            return Ok(Some(InitializerList {
                                slice: start.merge(peek.slice),
                                kind: InitializerKind::Expr(values),
                            }));
                        }
                    }
                    _ => return Err(ParserError::UnexpectedToken(next)),
                }
            }
        }
    }
}

fn parse_defaulted_initializer<'a>(
    tokenizer: &mut Tokenizer<'a>,
) -> Result<Option<DefaultedInitializer<'a>>, ParserError<'a>> {
    let peek = tokenizer.peek(0)?;
    let TokenKind::Symbol(Symbol::Rest) = peek.kind else {
        return Ok(None);
    };
    tokenizer.next()?;
    let start = peek.slice;

    let peek = tokenizer.peek(0)?;
    let Some(value) = parse_expr(tokenizer)? else {
        return Err(ParserError::UnexpectedToken(peek));
    };

    return Ok(Some(DefaultedInitializer {
        slice: start.merge(value.slice),
        value: Box::new(value),
    }));
}

fn parse_named_initializer<'a>(
    tokenizer: &mut Tokenizer<'a>,
) -> Result<NamedInitializer<'a>, ParserError<'a>> {
    let next = tokenizer.next()?;
    let TokenKind::Symbol(Symbol::Dot) = next.kind else {
        return Err(ParserError::UnexpectedToken(next));
    };
    let start = next.slice;

    let next = tokenizer.next()?;
    let TokenKind::Identifier(name) = next.kind else {
        return Err(ParserError::UnexpectedToken(next));
    };

    let next = tokenizer.next()?;
    let TokenKind::Symbol(Symbol::Assign) = next.kind else {
        return Err(ParserError::UnexpectedToken(next));
    };

    let peek = tokenizer.peek(0)?;
    let Some(value) = parse_expr(tokenizer)? else {
        return Err(ParserError::UnexpectedToken(peek));
    };

    return Ok(NamedInitializer {
        slice: start.merge(value.slice),
        name,
        value,
    });
}

#[cfg(test)]
mod test {
    use std::assert_matches::assert_matches;

    use crate::{
        ast::{
            expr::{BinOp, Expr, ExprKind},
            parse::{error::ParserError, expr::parse_expr},
        },
        tokenizer::{token::Number, Tokenizer},
    };

    type TestResult = Result<(), ParserError<'static>>;

    #[test]
    fn single_value() -> TestResult {
        const SRC: &str = "15";
        let mut tokenizer = Tokenizer::new(SRC);

        let tree = parse_expr(&mut tokenizer)?;

        assert_matches!(
            tree,
            Some(Expr {
                slice: _,
                kind: ExprKind::Number(Number {
                    whole: 15,
                    decimal: 0.0
                })
            })
        );

        Ok(())
    }

    #[test]
    fn order_operations() -> TestResult {
        const SRC: &str = "1 + 2 * 2";
        let mut tokenizer = Tokenizer::new(SRC);

        let tree = parse_expr(&mut tokenizer)?;

        assert_matches!(
            tree,
            Some(Expr {
                slice: _,
                kind:
                    ExprKind::BinOp {
                        lhs:
                            box Expr {
                                slice: _,
                                kind:
                                    ExprKind::Number(Number {
                                        whole: 1,
                                        decimal: 0.0,
                                    }),
                            },
                        op: BinOp::Add,
                        rhs:
                            box Expr {
                                slice: _,
                                kind:
                                    ExprKind::BinOp {
                                        lhs:
                                            box Expr {
                                                slice: _,
                                                kind:
                                                    ExprKind::Number(Number {
                                                        whole: 2,
                                                        decimal: 0.0,
                                                    }),
                                            },
                                        op: BinOp::Mul,
                                        rhs:
                                            box Expr {
                                                slice: _,
                                                kind:
                                                    ExprKind::Number(Number {
                                                        whole: 2,
                                                        decimal: 0.0,
                                                    }),
                                            },
                                    },
                            },
                    },
            })
        );

        Ok(())
    }

    #[test]
    fn paren() -> TestResult {
        const SRC: &str = "(123)";
        let mut tokenizer = Tokenizer::new(SRC);

        let tree = parse_expr(&mut tokenizer)?;

        assert_matches!(
            tree,
            Some(Expr {
                slice: _,
                kind: ExprKind::Number(Number {
                    whole: 123,
                    decimal: 0.0
                })
            })
        );

        Ok(())
    }
}
