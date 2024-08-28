use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DeclaredSymbol {
    pub name: Arc<str>,
    pub is_pub: bool,
    pub kind: SymbolKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SymbolKind {
    Type(SymbolTypeKind),
    Value(SymbolValueKind),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SymbolTypeKind {
    Struct,
    WrapperStruct,
    Enum,
    IntEnum,
    Union,
    Trait,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SymbolValueKind {
    Function,
    Variable,
}
