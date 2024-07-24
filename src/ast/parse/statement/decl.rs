use crate::{ast::parse::error::ParserError, tokenizer::{token::{Keyword, TokenKind}, Tokenizer}};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Decl {
    Let,
    Mut,
    Const,
    Static,
    Func,
    ConstFunc
}

impl Decl {
    pub fn try_parse<'a>(tokenizer: &mut Tokenizer<'a>) -> Result<Option<Self>, ParserError<'a>> {
        let peek = tokenizer.peek(0)?;

        let decl = match peek.kind {
            TokenKind::Keyword(Keyword::Let) => Decl::Let,
            TokenKind::Keyword(Keyword::Mut) => Decl::Mut,
            TokenKind::Keyword(Keyword::Static) => Decl::Static,
            TokenKind::Keyword(Keyword::Func) => Decl::Func,
            TokenKind::Keyword(Keyword::Const) => {
                let next = tokenizer.peek(1)?;
                
                if let TokenKind::Keyword(Keyword::Func) = next.kind {
                    tokenizer.next()?;
                    return Ok(Some(Decl::ConstFunc));
                }
                
                return Ok(Some(Decl::Const));
            },
            _ => return Ok(None)
        };

        return Ok(Some(decl));
    }
}
