use crate::{string::StringSlice, tokenizer::token::Number};

use super::{expr::GenericsInstance, types::Type, IdentPath};

#[derive(Debug, Clone, PartialEq)]
pub struct Pattern<'a> {
    pub slice: StringSlice<'a>,
    pub kind: PatternKind<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PatternKind<'a> {
    Let {
        is_mut: bool,
        name: &'a str,
        ty: Option<Type<'a>>,
    },
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
    pub name: &'a str,
    pub pat: Option<Pattern<'a>>,
}
