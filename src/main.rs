#![feature(decl_macro, let_chains)]

use ast::parse::{
    error::ParserError,
    expr::{self},
};
use tokenizer::Tokenizer;

pub mod ast;
pub mod string;
pub mod tokenizer;

const STR: &'static str = include_str!("test.txt");

fn main() -> Result<(), ParserError<'static>> {
    let mut tokenizer = Tokenizer::new(STR);

    println!("{:#?}", expr::parse_root(&mut tokenizer)?);

    Ok(())
}
