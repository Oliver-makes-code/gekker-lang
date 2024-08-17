#![feature(decl_macro, let_chains, assert_matches, box_patterns)]

use parse_tree::parse::{self, error::ParserError};
use tokenizer::Tokenizer;

pub mod parse_tree;
pub mod string;
pub mod tokenizer;

const STR: &'static str = include_str!("../test/Main.gek");

fn main() -> Result<(), ParserError<'static>> {
    let mut tokenizer = Tokenizer::new(STR);

    println!("{:#?}", parse::parse_root(&mut tokenizer)?);

    Ok(())
}
