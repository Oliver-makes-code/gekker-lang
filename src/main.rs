#![feature(decl_macro, let_chains, assert_matches, box_patterns, backtrace_frames)]

use ast::parse::{error::ParserError, pattern::parse_pattern, statement::parse_block};
use tokenizer::Tokenizer;

pub mod ast;
pub mod string;
pub mod tokenizer;

const STR: &'static str = include_str!("test.txt");

fn main() -> Result<(), ParserError<'static>> {
    let mut tokenizer = Tokenizer::new(STR);

    println!("{:#?}", parse_block(&mut tokenizer)?);

    Ok(())
}
