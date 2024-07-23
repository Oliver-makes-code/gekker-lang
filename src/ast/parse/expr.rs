use crate::{
    ast::expr::{AccessKind, BinOp, Expr, ExprKind, UnaryOp},
    tokenizer::{
        token::{Keyword, Symbol, TokenKind},
        Tokenizer,
    },
};

use super::error::ParserError;

pub fn parse_root<'a>(tokenizer: &mut Tokenizer<'a>) -> Result<Expr<'a>, ParserError<'a>> {
    return parse_operators(tokenizer, 0);
}

/// Pratt parsing!! Yippee!!!!
pub fn parse_operators<'a>(
    tokenizer: &mut Tokenizer<'a>,
    binding: usize,
) -> Result<Expr<'a>, ParserError<'a>> {
    let mut expr = parse_unary(tokenizer)?;

    while let Some((_, op)) = BinOp::try_parse(tokenizer)? {
        let (lhs_binding, rhs_binding) = op.binding();
        if lhs_binding < binding {
            break;
        }
        tokenizer.next()?;

        let rhs = parse_operators(tokenizer, rhs_binding)?;
        let slice = expr.slice.merge(rhs.slice);

        expr = Expr {
            slice,
            kind: ExprKind::BinOp(Box::new(expr), op, Box::new(rhs)),
        };
    }

    return Ok(expr);
}

pub fn parse_unary<'a>(tokenizer: &mut Tokenizer<'a>) -> Result<Expr<'a>, ParserError<'a>> {
    let mut unary_ops = vec![];
    while let Some(op) = UnaryOp::try_parse(tokenizer)? {
        tokenizer.next()?;
        unary_ops.push(op);
    }

    let mut expr = parse_access(tokenizer)?;

    while let Some((slice, op)) = unary_ops.pop() {
        tokenizer.next()?;
        expr = Expr {
            slice: slice.merge(expr.slice),
            kind: ExprKind::UnaryOp(op, Box::new(expr)),
        };
    }

    return Ok(expr);
}

pub fn parse_access<'a>(tokenizer: &mut Tokenizer<'a>) -> Result<Expr<'a>, ParserError<'a>> {
    let mut expr = parse_atom(tokenizer)?;

    while let Some(access) = parse_access_arm(tokenizer, expr.clone())? {
        expr = access;
    }

    return Ok(expr);
}

pub fn parse_access_arm<'a>(
    tokenizer: &mut Tokenizer<'a>,
    expr: Expr<'a>,
) -> Result<Option<Expr<'a>>, ParserError<'a>> {
    if let Some((_, kind)) = AccessKind::try_parse(tokenizer)? {
        tokenizer.next()?;
        let next = tokenizer.next()?;

        let TokenKind::Identifier(ident) = next.kind else {
            return Err(ParserError::UnexpectedToken(next, "Identifier"));
        };

        return Ok(Some(Expr {
            slice: expr.slice.merge(next.slice.unwrap()),
            kind: ExprKind::Field(Box::new(expr), kind, ident),
        }));
    }

    let next = tokenizer.peek()?;

    let TokenKind::Symbol(Symbol::BracketOpen) = next.kind else {
        return Ok(None);
    };
    tokenizer.next()?;

    let index = parse_root(tokenizer)?;

    let next = tokenizer.next()?;

    let TokenKind::Symbol(Symbol::BracketClose) = next.kind else {
        return Err(ParserError::UnexpectedToken(next, "Accessor"));
    };

    let slice = next.slice.unwrap();

    return Ok(Some(Expr {
        slice: expr.slice.merge(slice),
        kind: ExprKind::Index(Box::new(expr), Box::new(index)),
    }));
}

pub fn parse_atom<'a>(tokenizer: &mut Tokenizer<'a>) -> Result<Expr<'a>, ParserError<'a>> {
    let token = tokenizer.next()?;

    let Some(slice) = token.slice else {
        return Err(ParserError::UnexpectedEof);
    };

    let kind = match token.kind {
        TokenKind::Identifier(ident) => {
            let mut slice = slice;
            let mut idents = vec![ident];

            while let TokenKind::Symbol(Symbol::DoubleColon) = tokenizer.peek()?.kind {
                tokenizer.next()?;
                let next = tokenizer.next()?;
                let TokenKind::Identifier(ident) = next.kind else {
                    return Err(ParserError::UnexpectedToken(next, "Identifier"));
                };
                idents.push(ident);
                slice = slice.merge(next.slice.unwrap());
            }

            return Ok(Expr {
                slice,
                kind: ExprKind::Identifier(idents),
            });
        }
        TokenKind::Char(c) => ExprKind::Char(c),
        TokenKind::Number(n) => ExprKind::Number(n),
        TokenKind::String(s) => ExprKind::String(s),
        TokenKind::Keyword(Keyword::Discard) => ExprKind::Discard,
        TokenKind::Keyword(Keyword::True) => ExprKind::Bool(true),
        TokenKind::Keyword(Keyword::False) => ExprKind::Bool(false),
        TokenKind::Symbol(Symbol::ParenOpen) => {
            let expr = parse_root(tokenizer)?;

            let next = tokenizer.next()?;

            let TokenKind::Symbol(Symbol::ParenClose) = next.kind else {
                return Err(ParserError::UnexpectedToken(next, "ParenClose"));
            };

            return Ok(Expr {
                slice: slice.merge(next.slice.unwrap()),
                ..expr
            });
        }
        _ => {
            return Err(ParserError::UnexpectedToken(token, "Value"));
        }
    };

    return Ok(Expr { slice, kind });
}
