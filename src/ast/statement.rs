use crate::string::StringSlice;

use super::expr::Expr;

#[derive(Debug, Clone, PartialEq)]
pub struct Statement<'a> {
    pub slice: StringSlice<'a>,
    pub kind: StatementKind<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StatementKind<'a> {
    VariableDecl(VariableModifier, VariableName<'a>, Option<Expr<'a>>),
    Expr(Expr<'a>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VariableName<'a> {
    Identifier(&'a str),
    Discard,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VariableModifier {
    Let,
    Mut,
    Const,
    Static,
}
