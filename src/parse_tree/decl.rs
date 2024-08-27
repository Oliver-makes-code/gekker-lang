use std::{fmt::Debug, sync::Arc};

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
pub struct DeclModifier<T>
where
    T: Debug + Clone + PartialEq,
{
    pub slice: StringSlice,
    pub attrs: Option<Attrs>,
    pub generics: Option<GenericsDecl>,
    pub is_pub: bool,
    pub value: T,
}

/// Allowed to be public, have properties, and have generics
#[derive(Debug, Clone, PartialEq)]
pub struct DeclLvl1 {
    pub slice: StringSlice,
    pub kind: DeclLvl1Kind,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DeclLvl1Kind {
    Enum(EnumDecl),
    Union(UnionDecl),
    Struct(StructDecl),
    Trait(TraitDecl),
    Impl(ImplDecl),
    Lvl2(DeclLvl2),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ImportDecl {
    pub slice: StringSlice,
    pub path: Arc<str>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NamespaceDecl {
    pub slice: StringSlice,
    pub path: IdentPath,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ImplDecl {
    pub slice: StringSlice,
    pub tr: Type,
    pub ty: Type,
    pub body: TraitBody,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TraitDecl {
    pub slice: StringSlice,
    pub name: Arc<str>,
    pub body: TraitBody,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TraitBody {
    pub slice: StringSlice,
    pub decls: Vec<DeclModifier<DeclLvl2>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnionDecl {
    pub slice: StringSlice,
    pub name: Arc<str>,
    pub body: StructBody,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnumDecl {
    pub slice: StringSlice,
    pub name: Arc<str>,
    pub kind: EnumDeclKind,
}

#[derive(Debug, Clone, PartialEq)]
pub enum EnumDeclKind {
    Int { ty: IntEnumType, body: IntEnumBody },
    Value(StructBody),
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructDecl {
    pub slice: StringSlice,
    pub name: Arc<str>,
    pub kind: StructDeclKind,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StructDeclKind {
    Wrapper(Type),
    Value(StructBody),
}

/// Allowed in top-level code and trait impls
#[derive(Debug, Clone, PartialEq)]
pub struct DeclLvl2 {
    pub slice: StringSlice,
    pub kind: DeclLvl2Kind,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DeclLvl2Kind {
    Function(FunctionDecl),
    Variable(VariableDecl),
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDecl {
    pub slice: StringSlice,
    pub modifier: FunctionModifier,
    pub name: Arc<str>,
    pub this_param: Option<ThisParam>,
    pub params: Vec<FuncParam>,
    pub ret: Option<Type>,
    pub body: Option<FuncBody>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VariableDecl {
    pub slice: StringSlice,
    pub modifier: VariableModifier,
    pub name: VariableName,
    pub ty: Option<Type>,
    pub init: Option<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GenericsDecl {
    pub slice: StringSlice,
    pub tys: Vec<GenericType>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GenericType {
    pub slice: StringSlice,
    pub name: Arc<str>,
    pub clauses: Vec<TypeClause>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeClause {
    pub slice: StringSlice,
    pub exclude: bool,
    pub ty: ClauseKind,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ClauseKind {
    RealType(Type),
    Default,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Attrs {
    pub slice: StringSlice,
    pub attrs: Vec<Attr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Attr {
    pub slice: StringSlice,
    pub name: Arc<str>,
    pub params: Vec<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FuncBody {
    pub slice: StringSlice,
    pub kind: FuncBodyKind,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FuncBodyKind {
    Block(Block),
    Expr(Expr),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ThisParam {
    pub slice: StringSlice,
    pub is_mut: bool,
    pub ref_kind: Option<RefKind>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FuncParam {
    pub slice: StringSlice,
    pub is_mut: bool,
    pub name: Arc<str>,
    pub ty: Type,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructBody {
    pub slice: StringSlice,
    pub params: Vec<StructParam>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructParam {
    pub slice: StringSlice,
    pub is_pub: bool,
    pub name: Arc<str>,
    pub ty: Type,
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
pub struct IntEnumBody {
    pub slice: StringSlice,
    pub params: Vec<IntEnumParam>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IntEnumParam {
    pub slice: StringSlice,
    pub name: Arc<str>,
    pub value: Option<Expr>,
}

impl IntEnumType {
    pub fn from(kind: TokenKind) -> Option<Self> {
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
