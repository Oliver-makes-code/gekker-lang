use std::sync::Arc;

use crate::string::StringSlice;

use super::{decl::VariableDecl, expr::Expr, pattern::Pattern};

#[derive(Debug, Clone, PartialEq)]
pub struct Statement {
    pub slice: StringSlice,
    pub kind: StatementKind,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub slice: StringSlice,
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StatementKind {
    Decl(VariableDecl),
    Expr(Expr),
    If(IfStatement),
    LetMatchElse(LetMatchElseStatement),
    Match(MatchStatement),
    Return(ReturnStatement),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReturnStatement {
    pub slice: StringSlice,
    pub value: Option<Expr>,
    pub condition: Option<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LetMatchElseStatement {
    pub slice: StringSlice,
    pub clause: LetMatchClause,
    pub block: Block,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfStatement {
    pub slice: StringSlice,
    pub conditions: Vec<IfCondition>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfCondition {
    pub slice: StringSlice,
    pub condition: Option<IfClause>,
    pub block: Block,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfClause {
    pub slice: StringSlice,
    pub kind: IfClauseKind,
}

#[derive(Debug, Clone, PartialEq)]
pub enum IfClauseKind {
    Expr(Expr),
    LetMatch(LetMatchClause),
}

#[derive(Debug, Clone, PartialEq)]
pub struct LetMatchClause {
    pub slice: StringSlice,
    pub pat: Pattern,
    pub value: Expr,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatchStatement {
    pub slice: StringSlice,
    pub value: Expr,
    pub clauses: Vec<MatchClause>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatchClause {
    pub slice: StringSlice,
    pub pat: Pattern,
    pub block: MatchBlock,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatchBlock {
    pub slice: StringSlice,
    pub kind: MatchBlockKind,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MatchBlockKind {
    Statement(Box<Statement>),
    Block(Block),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VariableName {
    Identifier(Arc<str>),
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
