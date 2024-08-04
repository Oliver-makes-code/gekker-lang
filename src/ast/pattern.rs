use crate::{string::StringSlice, tokenizer::token::Number};

use super::{expr::GenericsInstance, IdentPath};

#[derive(Debug, Clone, PartialEq)]
pub struct Pattern<'a> {
    pub slice: StringSlice<'a>,
    pub kind: PatternKind<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PatternKind<'a> {
    Name(&'a str),
    Struct {
        name: IdentPath<'a>,
        generics: GenericsInstance<'a>,
        values: Vec<StructPattern<'a>>,
    },
    Array(Vec<Pattern<'a>>),
    Number(Number),
    Bool(bool),
    Discard,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructPattern<'a> {
    pub slice: StringSlice<'a>,
    pub kind: StructPatternKind<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StructPatternKind<'a> {
    Base(Pattern<'a>),
    Named { real: &'a str, name: &'a str },
}
