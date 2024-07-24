use crate::{
    ast::{parse::error::ParserError, statement::VariableModifier},
    tokenizer::{
        token::{Keyword, TokenKind},
        Tokenizer,
    },
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Decl {
    Let,
    Mut,
    Const,
    Static,
    Func,
    ConstFunc,
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
            }
            _ => return Ok(None),
        };

        return Ok(Some(decl));
    }

    pub fn try_into_var(self) -> Option<VariableModifier> {
        return Some(match self {
            Self::Let => VariableModifier::Let,
            Self::Mut => VariableModifier::Mut,
            Self::Const => VariableModifier::Const,
            Self::Static => VariableModifier::Static,
            _ => return None,
        });
    }

    pub fn try_into_func(self) {
        todo!()
    }
}
