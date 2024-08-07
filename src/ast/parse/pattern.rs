use crate::{ast::pattern::Pattern, tokenizer::Tokenizer};

use super::error::ParserError;

type PatternResult<'a> = Result<Pattern<'a>, ParserError<'a>>;

fn parse_pattern<'a>(tokenizer: &mut Tokenizer<'a>) -> PatternResult<'a> {
    todo!()
}
