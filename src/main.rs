#![feature(decl_macro, let_chains, assert_matches, box_patterns)]

use fs_queue::SourceError;

pub mod fs_queue;
pub mod parse_tree;
pub mod string;
pub mod tokenizer;

fn main() -> Result<(), SourceError> {
    fs_queue::load_all_source();

    fs_queue::parse_trees()?;

    Ok(())
}
