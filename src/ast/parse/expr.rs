use crate::{
    ast::{
        expr::{AccessKind, BinOp, Expr, ExprKind, UnaryOp},
        IdentPath,
    },
    tokenizer::{
        token::{Keyword, Symbol, TokenKind},
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

    while let Some((_, op)) = BinOp::try_parse(tokenizer)? {
        let (lhs_binding, rhs_binding) = op.binding();
        if lhs_binding < binding {
            break;
        }
        tokenizer.clear_peek_queue();

        let Some(rhs) = parse_operators(tokenizer, rhs_binding)? else {
            return Err(ParserError::UnexpectedToken(
                tokenizer.peek(0)?,
                "Expression",
            ));
        };
        let slice = expr.slice.merge(rhs.slice);

        expr = Expr {
            slice,
            kind: ExprKind::BinOp(Box::new(expr), op, Box::new(rhs)),
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
            kind: ExprKind::UnaryOp(op, Box::new(expr)),
        };
    }

    return Ok(Some(expr));
}

fn parse_access<'a>(tokenizer: &mut Tokenizer<'a>) -> ExprResult<'a> {
    let Some(mut expr) = parse_atom(tokenizer)? else {
        return Ok(None);
    };

    while let Some(access) = parse_access_arm(tokenizer, expr.clone())? {
        expr = access;
    }

    return Ok(Some(expr));
}

fn parse_access_arm<'a>(tokenizer: &mut Tokenizer<'a>, expr: Expr<'a>) -> ExprResult<'a> {
    let next = tokenizer.peek(0)?;

    if let Some(kind) = AccessKind::try_parse(next.kind.clone()) {
        tokenizer.next()?;
        let next = tokenizer.next()?;

        let TokenKind::Identifier(ident) = next.kind else {
            return Err(ParserError::UnexpectedToken(next, "Identifier"));
        };

        return Ok(Some(Expr {
            slice: expr.slice.merge(next.slice),
            kind: ExprKind::Field(Box::new(expr), kind, ident),
        }));
    }

    if let TokenKind::Symbol(Symbol::ParenOpen) = next.kind {
        let mut peek = tokenizer.peek(1)?;
        let mut exprs = vec![];

        while peek.kind != TokenKind::Symbol(Symbol::ParenClose) {
            tokenizer.next()?;

            let t = tokenizer.peek(0)?;
            let Some(expr) = parse_expr(tokenizer)? else {
                return Err(ParserError::UnexpectedToken(t, "Expr"));
            };
            exprs.push(expr);

            peek = tokenizer.peek(0)?;

            let TokenKind::Symbol(Symbol::Comma | Symbol::ParenClose) = peek.kind else {
                return Err(ParserError::UnexpectedToken(peek, "Comma or Close paren"));
            };
        }
        tokenizer.next()?;

        return Ok(Some(Expr {
            slice: expr.slice.merge(peek.slice),
            kind: ExprKind::Invoke(Box::new(expr), exprs),
        }));
    }

    let TokenKind::Symbol(Symbol::BracketOpen) = next.kind else {
        return Ok(None);
    };
    tokenizer.next()?;

    let Some(index) = parse_expr(tokenizer)? else {
        return Err(ParserError::UnexpectedToken(
            tokenizer.peek(0)?,
            "Expression",
        ));
    };

    let next = tokenizer.next()?;

    let TokenKind::Symbol(Symbol::BracketClose) = next.kind else {
        return Err(ParserError::UnexpectedToken(next, "Accessor"));
    };

    let slice = next.slice;

    return Ok(Some(Expr {
        slice: expr.slice.merge(slice),
        kind: ExprKind::Index(Box::new(expr), Box::new(index)),
    }));
}

fn parse_atom<'a>(tokenizer: &mut Tokenizer<'a>) -> ExprResult<'a> {
    if let Some((slice, idents)) = IdentPath::try_parse(tokenizer)? {
        return Ok(Some(Expr {
            slice,
            kind: ExprKind::IdentPath(idents),
        }));
    }

    let token = tokenizer.peek(0)?;

    let slice = token.slice;

    let kind = match token.kind {
        TokenKind::Char(c) => ExprKind::Char(c),
        TokenKind::Number(n) => ExprKind::Number(n),
        TokenKind::String(s) => ExprKind::String(s),
        TokenKind::Keyword(Keyword::Discard) => ExprKind::Discard,
        TokenKind::Keyword(Keyword::True) => ExprKind::Bool(true),
        TokenKind::Keyword(Keyword::False) => ExprKind::Bool(false),
        TokenKind::Symbol(Symbol::ParenOpen) => {
            tokenizer.clear_peek_queue();

            let Some(expr) = parse_expr(tokenizer)? else {
                return Err(ParserError::UnexpectedToken(
                    tokenizer.peek(0)?,
                    "Expression",
                ));
            };

            let next = tokenizer.next()?;

            let TokenKind::Symbol(Symbol::ParenClose) = next.kind else {
                return Err(ParserError::UnexpectedToken(next, "ParenClose"));
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

    tokenizer.clear_peek_queue();

    return Ok(Some(Expr { slice, kind }));
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
                    ExprKind::BinOp(
                        box Expr {
                            slice: _,
                            kind:
                                ExprKind::Number(Number {
                                    whole: 1,
                                    decimal: 0.0,
                                }),
                        },
                        BinOp::Add,
                        box Expr {
                            slice: _,
                            kind:
                                ExprKind::BinOp(
                                    box Expr {
                                        slice: _,
                                        kind:
                                            ExprKind::Number(Number {
                                                whole: 2,
                                                decimal: 0.0,
                                            }),
                                    },
                                    BinOp::Mul,
                                    box Expr {
                                        slice: _,
                                        kind:
                                            ExprKind::Number(Number {
                                                whole: 2,
                                                decimal: 0.0,
                                            }),
                                    },
                                ),
                        },
                    ),
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
