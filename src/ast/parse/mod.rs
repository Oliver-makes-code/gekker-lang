use error::ParserError;

use crate::tokenizer::Tokenizer;

use super::Expr;

pub mod error;
pub mod expr;

pub fn parse_root<'a>(_tokenizer: &mut Tokenizer<'a>) -> Result<Expr<'a>, ParserError<'a>> {
    todo!()
}
