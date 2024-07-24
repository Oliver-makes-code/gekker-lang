use decl::Decl;

use crate::{ast::statement::Statement, tokenizer::Tokenizer};

use super::error::ParserError;

mod decl;

pub fn parse_root<'a>(tokenizer: &mut Tokenizer<'a>) -> Result<Statement<'a>, ParserError<'a>> {
    if let Some(decl) = Decl::try_parse(tokenizer)? {
        tokenizer.next()?;
        println!("{:?}", decl);
    }

    todo!()
}
