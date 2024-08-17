use std::sync::Arc;

use crate::{string::StringSlice, tokenizer::token::Number};

use super::{expr::GenericsInstance, IdentPath};

#[derive(Debug, Clone, PartialEq)]
pub struct Pattern {
    pub slice: StringSlice,
    pub kind: PatternKind,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PatternKind {
    Value {
        is_mut: bool,
        name: Arc<str>,
    },
    Initializer {
        name: IdentPath,
        generics: Option<GenericsInstance>,
        list: InitializerPattern,
    },
    Or(Vec<Pattern>),
    Number(Number),
    Bool(bool),
    String(Arc<str>),
    Char(char),
    Invalid,
    Nullptr,
    Discard,
    Default,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InitializerPattern {
    pub slice: StringSlice,
    pub kind: InitializerPatternKind,
}

#[derive(Debug, Clone, PartialEq)]
pub enum InitializerPatternKind {
    Expr(Vec<Pattern>),
    Named(Vec<NamedInitializerPattern>),
    Empty,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NamedInitializerPattern {
    pub slice: StringSlice,
    pub name: Arc<str>,
    pub value: Pattern,
}
