#![feature(decl_macro, let_chains)]

use ast::parse::{
    error::ParserError,
    expr::{self}, statement,
};
use tokenizer::{token, Tokenizer};

pub mod ast;
pub mod string;
pub mod tokenizer;

const STR: &'static str = include_str!("test.txt");

fn main() -> Result<(), ParserError<'static>> {
    let mut tokenizer = Tokenizer::new(STR);

    statement::parse_root(&mut tokenizer)?;

    Ok(())
}
