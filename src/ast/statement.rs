use crate::string::StringSlice;

use super::{decl::Decl, expr::Expr};

#[derive(Debug, Clone, PartialEq)]
pub struct Statement<'a> {
    pub slice: StringSlice<'a>,
    pub kind: StatementKind<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Block<'a> {
    pub slice: StringSlice<'a>,
    pub statements: Vec<Statement<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StatementKind<'a> {
    Decl(Decl<'a>),
    Expr(Expr<'a>),
    If(IfStatement<'a>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfStatement<'a> {
    pub slice: StringSlice<'a>,
    pub conditions: Vec<IfCondition<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfCondition<'a> {
    pub slice: StringSlice<'a>,
    pub condition: Option<Expr<'a>>,
    pub block: Block<'a>,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FunctionModifier {
    Func,
    ConstFunc,
}
