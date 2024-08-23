#![feature(decl_macro, let_chains, assert_matches, box_patterns)]

use parse_tree::parse::error::ParserError;

pub mod parse_tree;
pub mod string;
pub mod tokenizer;

fn main() -> Result<(), ParserError> {
    Ok(())
}
