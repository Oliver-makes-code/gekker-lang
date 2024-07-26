#![feature(decl_macro, let_chains, assert_matches, box_patterns)]

use ast::parse::{error::ParserError, types};
use tokenizer::Tokenizer;

pub mod ast;
pub mod string;
pub mod tokenizer;

const STR: &'static str = include_str!("test.txt");

fn main() -> Result<(), ParserError<'static>> {
    let mut tokenizer = Tokenizer::new(STR);

    println!("{:#?}", types::parse_root(&mut tokenizer)?);

    Ok(())
}
