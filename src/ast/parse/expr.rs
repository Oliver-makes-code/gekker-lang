use crate::{
    ast::{Expr, ExprKind, UnaryOp},
    tokenizer::{
        token::{Keyword, Symbol, TokenKind},
        Tokenizer,
    },
};

use super::error::ParserError;

pub fn parse_root<'a>(tokenizer: &mut Tokenizer<'a>) -> Result<Expr<'a>, ParserError<'a>> {
    return parse_operators(tokenizer);
}

/// Pratt parsing!! Yippee!!!!
pub fn parse_operators<'a>(tokenizer: &mut Tokenizer<'a>) -> Result<Expr<'a>, ParserError<'a>> {
    return parse_unary(tokenizer);
}

pub fn parse_unary<'a>(tokenizer: &mut Tokenizer<'a>) -> Result<Expr<'a>, ParserError<'a>> {
    let mut unary_ops = vec![];
    let mut peek = tokenizer.peek()?;
    while let TokenKind::Symbol(Symbol::Add | Symbol::Sub | Symbol::BoolNot | Symbol::BitNot) =
        peek.kind
    {
        unary_ops.push((
            peek.slice.unwrap(),
            match peek.kind {
                TokenKind::Symbol(Symbol::Add) => UnaryOp::Add,
                TokenKind::Symbol(Symbol::Sub) => UnaryOp::Sub,
                TokenKind::Symbol(Symbol::BoolNot) => UnaryOp::BoolNot,
                TokenKind::Symbol(Symbol::BitNot) => UnaryOp::BitNot,
                _ => panic!("Impossible state."),
            },
        ));
        tokenizer.next()?;
        peek = tokenizer.peek()?;
    }

    let mut expr = parse_atom(tokenizer)?;

    while let Some((slice, op)) = unary_ops.pop() {
        expr = Expr {
            slice: slice.merge(&expr.slice),
            kind: ExprKind::UnaryOp(op, Box::new(expr)),
        };
    }

    return Ok(expr);
}

pub fn parse_atom<'a>(tokenizer: &mut Tokenizer<'a>) -> Result<Expr<'a>, ParserError<'a>> {
    let token = tokenizer.next()?;

    let Some(slice) = token.slice else {
        return Err(ParserError::UnexpectedEof);
    };

    let kind = match token.kind {
        TokenKind::Identifier(ident) => ExprKind::Identifier(ident),
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
                return Err(ParserError::UnexpectedToken(next));
            };

            return Ok(Expr {
                slice: slice.merge(&next.slice.unwrap()),
                ..expr
            });
        }
        _ => {
            return Err(ParserError::UnexpectedToken(token));
        }
    };

    return Ok(Expr { slice, kind });
}
