#![feature(decl_macro, let_chains, assert_matches, box_patterns)]

use std::marker::PhantomData;

use ast::parse::{error::ParserError, expr::parse_initializer_list, pattern::parse_pattern};
use tokenizer::Tokenizer;

pub mod ast;
pub mod string;
pub mod tokenizer;

const STR: &'static str = include_str!("test.txt");

fn main() -> Result<(), ParserError<'static>> {
    let mut tokenizer = Tokenizer::new(STR);

    println!("{:#?}", parse_initializer_list(&mut tokenizer)?);

    Ok(())
}
