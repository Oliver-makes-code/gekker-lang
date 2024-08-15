use crate::string::StringSlice;

use super::{decl::Decl, IdentPath, ParseTree};

#[derive(Debug, Clone, PartialEq)]
pub struct TopLevelStatement<'a> {
    pub slice: StringSlice<'a>,
    pub kind: TopLevelStatementKind<'a>
}

#[derive(Debug, Clone, PartialEq)]
pub enum TopLevelStatementKind<'a> {
    Decl(Decl<'a>),
    Namespace {
        path: IdentPath<'a>,
        tree: Option<ParseTree<'a>>
    }
}
