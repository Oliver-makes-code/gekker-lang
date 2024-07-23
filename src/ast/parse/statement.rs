use crate::{ast::statement::Statement, tokenizer::Tokenizer};

use super::error::ParserError;

pub fn parse_root<'a>(_tokenizer: &mut Tokenizer<'a>) -> Result<Statement<'a>, ParserError<'a>> {
    todo!()
}
