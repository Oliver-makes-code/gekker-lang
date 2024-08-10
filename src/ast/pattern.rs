use crate::{string::StringSlice, tokenizer::token::Number};

use super::{expr::GenericsInstance, IdentPath};

#[derive(Debug, Clone, PartialEq)]
pub struct Pattern<'a> {
    pub slice: StringSlice<'a>,
    pub kind: PatternKind<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PatternKind<'a> {
    Value {
        is_mut: bool,
        name: &'a str,
    },
    Initializer {
        name: IdentPath<'a>,
        generics: Option<GenericsInstance<'a>>,
        list: InitializerPattern<'a>,
    },
    Or(Vec<Pattern<'a>>),
    Number(Number),
    Bool(bool),
    String(String),
    Char(char),
    Invalid,
    Nullptr,
    Discard,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InitializerPattern<'a> {
    pub slice: StringSlice<'a>,
    pub kind: InitializerPatternKind<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum InitializerPatternKind<'a> {
    Expr(Vec<Pattern<'a>>),
    Named(Vec<NamedInitializerPattern<'a>>),
    Empty,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NamedInitializerPattern<'a> {
    pub slice: StringSlice<'a>,
    pub name: &'a str,
    pub value: Pattern<'a>,
}
