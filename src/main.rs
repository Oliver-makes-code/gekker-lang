#![feature(decl_macro, let_chains, assert_matches, box_patterns)]

use parse_tree::parse::{error::ParserError, statement::parse_block};
use tokenizer::Tokenizer;

pub mod parse_tree;
pub mod string;
pub mod tokenizer;

const STR: &'static str = include_str!("test.txt");

fn main() -> Result<(), ParserError<'static>> {
    let mut tokenizer = Tokenizer::new(STR);

    println!("{:#?}", parse_block(&mut tokenizer)?);

    Ok(())
}
