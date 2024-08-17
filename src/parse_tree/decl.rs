use std::fmt::Debug;

use crate::{
    string::StringSlice,
    tokenizer::token::{Keyword, TokenKind},
};

use super::{
    expr::Expr,
    statement::{Block, FunctionModifier, VariableModifier, VariableName},
    types::{RefKind, Type},
    IdentPath,
};

#[derive(Debug, Clone, PartialEq)]
pub struct DeclModifier<'a, T>
where
    T: Debug + Clone + PartialEq,
{
    pub slice: StringSlice<'a>,
    pub attrs: Option<Attrs<'a>>,
    pub generics: Option<GenericsDecl<'a>>,
    pub is_pub: bool,
    pub value: T,
}

/// Allowed in top-level code
#[derive(Debug, Clone, PartialEq)]
pub struct DeclLvl1<'a> {
    pub slice: StringSlice<'a>,
    pub kind: DeclLvl1Kind<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DeclLvl1Kind<'a> {
    Namespace(NamespaceDecl<'a>),
    Using(NamespaceDecl<'a>),
    Lvl2(DeclModifier<'a, DeclLvl2<'a>>),
}

/// Allowed to be public, have properties, and have generics
#[derive(Debug, Clone, PartialEq)]
pub struct DeclLvl2<'a> {
    pub slice: StringSlice<'a>,
    pub kind: DeclLvl2Kind<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DeclLvl2Kind<'a> {
    Enum(EnumDecl<'a>),
    Union(UnionDecl<'a>),
    Struct(StructDecl<'a>),
    Trait(TraitDecl<'a>),
    Impl(ImplDecl<'a>),
    Lvl3(DeclLvl3<'a>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct NamespaceDecl<'a> {
    pub slice: StringSlice<'a>,
    pub path: IdentPath<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ImplDecl<'a> {
    pub slice: StringSlice<'a>,
    pub tr: Type<'a>,
    pub ty: Type<'a>,
    pub body: TraitBody<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TraitDecl<'a> {
    pub slice: StringSlice<'a>,
    pub name: &'a str,
    pub body: TraitBody<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TraitBody<'a> {
    pub slice: StringSlice<'a>,
    pub decls: Vec<DeclModifier<'a, DeclLvl3<'a>>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnionDecl<'a> {
    pub slice: StringSlice<'a>,
    pub name: &'a str,
    pub body: StructBody<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnumDecl<'a> {
    pub slice: StringSlice<'a>,
    pub name: &'a str,
    pub kind: EnumDeclKind<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum EnumDeclKind<'a> {
    Int {
        ty: IntEnumType,
        body: IntEnumBody<'a>,
    },
    Value(StructBody<'a>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructDecl<'a> {
    pub slice: StringSlice<'a>,
    pub name: &'a str,
    pub kind: StructDeclKind<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StructDeclKind<'a> {
    Wrapper(Type<'a>),
    Value(StructBody<'a>),
}

/// Allowed in top-level code and trait impls
#[derive(Debug, Clone, PartialEq)]
pub struct DeclLvl3<'a> {
    pub slice: StringSlice<'a>,
    pub kind: DeclLvl3Kind<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DeclLvl3Kind<'a> {
    Function(FunctionDecl<'a>),
    Variable(VariableDecl<'a>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDecl<'a> {
    pub slice: StringSlice<'a>,
    pub modifier: FunctionModifier,
    pub name: &'a str,
    pub this_param: Option<ThisParam<'a>>,
    pub params: Vec<FuncParam<'a>>,
    pub ret: Option<Type<'a>>,
    pub body: Option<FuncBody<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VariableDecl<'a> {
    pub slice: StringSlice<'a>,
    pub modifier: VariableModifier,
    pub name: VariableName<'a>,
    pub ty: Option<Type<'a>>,
    pub init: Option<Expr<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GenericsDecl<'a> {
    pub slice: StringSlice<'a>,
    pub tys: Vec<GenericType<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GenericType<'a> {
    pub slice: StringSlice<'a>,
    pub name: &'a str,
    pub clauses: Vec<TypeClause<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeClause<'a> {
    pub slice: StringSlice<'a>,
    pub exclude: bool,
    pub ty: ClauseKind<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ClauseKind<'a> {
    RealType(Type<'a>),
    Default,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Attrs<'a> {
    pub slice: StringSlice<'a>,
    pub attrs: Vec<Attr<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Attr<'a> {
    pub slice: StringSlice<'a>,
    pub name: &'a str,
    pub params: Vec<Expr<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FuncBody<'a> {
    pub slice: StringSlice<'a>,
    pub kind: FuncBodyKind<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FuncBodyKind<'a> {
    Block(Block<'a>),
    Expr(Expr<'a>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ThisParam<'a> {
    pub slice: StringSlice<'a>,
    pub is_mut: bool,
    pub ref_kind: Option<RefKind>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FuncParam<'a> {
    pub slice: StringSlice<'a>,
    pub is_mut: bool,
    pub name: &'a str,
    pub ty: Type<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructBody<'a> {
    pub slice: StringSlice<'a>,
    pub params: Vec<StructParam<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructParam<'a> {
    pub slice: StringSlice<'a>,
    pub is_pub: bool,
    pub name: &'a str,
    pub ty: Type<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum IntEnumType {
    U8,
    I8,
    U16,
    I16,
    U32,
    I32,
    U64,
    I64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IntEnumBody<'a> {
    pub slice: StringSlice<'a>,
    pub params: Vec<IntEnumParam<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IntEnumParam<'a> {
    pub slice: StringSlice<'a>,
    pub name: &'a str,
    pub value: Option<Expr<'a>>,
}

impl IntEnumType {
    pub fn from<'a>(kind: TokenKind<'a>) -> Option<Self> {
        let val = match kind {
            TokenKind::Keyword(Keyword::U8) => IntEnumType::U8,
            TokenKind::Keyword(Keyword::I8) => IntEnumType::I8,
            TokenKind::Keyword(Keyword::U16) => IntEnumType::U16,
            TokenKind::Keyword(Keyword::I16) => IntEnumType::I16,
            TokenKind::Keyword(Keyword::U32) => IntEnumType::U32,
            TokenKind::Keyword(Keyword::I32) => IntEnumType::I32,
            TokenKind::Keyword(Keyword::U64) => IntEnumType::U64,
            TokenKind::Keyword(Keyword::I64) => IntEnumType::I64,
            _ => return None,
        };
        return Some(val);
    }
}
