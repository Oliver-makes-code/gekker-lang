#![feature(decl_macro, let_chains, assert_matches, box_patterns)]

use std::{fs::File, io::Read};

use parse_tree::parse::parse_root;
use tokenizer::Tokenizer;

pub mod parse_tree;
pub mod semantic_model;
pub mod string;
pub mod tokenizer;

fn main() {
    let mut file = File::open("test/Main.gek").unwrap();
    let mut src = String::new();
    file.read_to_string(&mut src).unwrap();

    let mut tokenizer = Tokenizer::new(src.into());

    println!("{:#?}", parse_root(&mut tokenizer));
}
