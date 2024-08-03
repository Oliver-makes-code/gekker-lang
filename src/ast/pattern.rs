use crate::string::StringSlice;

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
    Named {
        real: &'a str,
        name: &'a str,
    },
}
