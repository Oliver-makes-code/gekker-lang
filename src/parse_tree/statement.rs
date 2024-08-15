use crate::string::StringSlice;

use super::{decl::Decl, expr::Expr, pattern::Pattern};

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
    LetMatchElse(LetMatchElseStatement<'a>),
    Match(MatchStatement<'a>),
    Return(ReturnStatement<'a>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReturnStatement<'a> {
    pub slice: StringSlice<'a>,
    pub value: Option<Expr<'a>>,
    pub condition: Option<Expr<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LetMatchElseStatement<'a> {
    pub slice: StringSlice<'a>,
    pub clause: LetMatchClause<'a>,
    pub block: Block<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfStatement<'a> {
    pub slice: StringSlice<'a>,
    pub conditions: Vec<IfCondition<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfCondition<'a> {
    pub slice: StringSlice<'a>,
    pub condition: Option<IfClause<'a>>,
    pub block: Block<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfClause<'a> {
    pub slice: StringSlice<'a>,
    pub kind: IfClauseKind<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum IfClauseKind<'a> {
    Expr(Expr<'a>),
    LetMatch(LetMatchClause<'a>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct LetMatchClause<'a> {
    pub slice: StringSlice<'a>,
    pub pat: Pattern<'a>,
    pub value: Expr<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatchStatement<'a> {
    pub slice: StringSlice<'a>,
    pub value: Expr<'a>,
    pub clauses: Vec<MatchClause<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatchClause<'a> {
    pub slice: StringSlice<'a>,
    pub pat: Pattern<'a>,
    pub block: MatchBlock<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatchBlock<'a> {
    pub slice: StringSlice<'a>,
    pub kind: MatchBlockKind<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MatchBlockKind<'a> {
    Statement(Box<Statement<'a>>),
    Block(Block<'a>),
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
