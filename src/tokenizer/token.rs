use std::sync::Arc;

use crate::string::{parser::StringParser, StringSlice};

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub slice: StringSlice,
    pub kind: TokenKind,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Identifier(Arc<str>),
    String(Arc<str>),
    Char(char),
    Number(Number),
    Symbol(Symbol),
    Keyword(Keyword),
    Eof,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Number {
    pub whole: u64,
    pub decimal: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Keyword {
    // Modifiers and declaration keywords
    Let,
    Mut,
    Const,
    Static,
    Func,
    Pub,
    Using,
    Ref,
    Struct,
    Enum,
    Trait,
    Impl,
    Operator,
    Where,
    Namespace,
    Union,
    Import,

    // Builtin types
    Bool,
    Char,
    Str,
    ThisType, // This
    Unit,
    Never,

    I8,
    U8,
    I16,
    U16,
    I32,
    U32,
    I64,
    U64,
    Usize,
    Isize,

    F32,
    F64,

    // Values / Pattern matching
    ThisValue, // this
    Discard,   // _
    True,
    False,
    Default, // default
    Sizeof,  // sizeof
    Nullptr, // nullptr
    Invalid, // invalid

    // Statements / Control flow
    For,
    While,
    Loop,
    If,
    Else,
    Match,
    In,

    Break,
    Return,
    Goto,
    Label,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Symbol {
    // Parenthesis and co.
    BracketOpen,  // [
    BracketClose, // ]
    BraceOpen,    // {
    BraceClose,   // }
    ParenOpen,    // (
    ParenClose,   // )

    // Addition
    Add,       // +
    Increment, // ++
    AddAssign, // +=

    // Subtraction
    Sub,       // -
    Decrement, // --
    SubAssign, // -=

    // Multiplication
    Mul,       // *
    MulAssign, // *=

    // Division
    Div,       // /
    DivAssign, // /=

    // Modulo
    Rem,       // %
    RemAssign, // %=

    // And
    BitAnd,       // &
    BoolAnd,      // &&
    BitAndAssign, // &=

    // Or
    BitOr,       // |
    BoolOr,      // ||
    BitOrAssign, // |=

    // Xor
    BitXor,       // ^
    BoolXor,      // ^^
    BitXorAssign, // ^=

    // Bit not
    BitNot,       // ~
    BitNotAssign, // ~=

    // Comparisons
    Greater,      // >
    Less,         // <
    GreaterEqual, // >=
    LessEqual,    // <=
    Equal,        // ==
    NotEqual,     // !=

    // Range
    Range,       // ..
    RangeTo,     // ..=
    RangeFrom,   // <..
    RangeFromTo, // <..=

    // Shift
    Shl,       // <<
    ShlAssign, // <<=
    Shr,       // >>
    ShrAssign, // >>=

    // Field access
    Dot,               // .
    ValueCoalesce,     // ?.
    ValueCascade,      // !.
    SmallArrow,        // ->
    ReferenceCoalesce, // ?->
    ReferenceCascade,  // !->

    // Other symbols
    BoolNot,     // !
    Colon,       // :
    Semicolon,   // ;
    Assign,      // =
    Optional,    // ?
    Comma,       // ,
    WideArrow,   // =>
    DoubleColon, // ::
    Pound,       // #
    Rest,        // ...
}

impl Keyword {
    pub fn from(s: &str) -> Option<Self> {
        return Some(match s {
            "let" => Self::Let,
            "mut" => Self::Mut,
            "const" => Self::Const,
            "static" => Self::Static,
            "func" => Self::Func,
            "pub" => Self::Pub,
            "using" => Self::Using,
            "ref" => Self::Ref,
            "struct" => Self::Struct,
            "enum" => Self::Enum,
            "trait" => Self::Trait,
            "impl" => Self::Impl,
            "operator" => Self::Operator,
            "where" => Self::Where,
            "namespace" => Self::Namespace,
            "union" => Self::Union,
            "import" => Self::Import,

            "bool" => Self::Bool,
            "char" => Self::Char,
            "This" => Self::ThisType,
            "unit" => Self::Unit,
            "never" => Self::Never,
            "str" => Self::Str,

            "i8" => Self::I8,
            "u8" => Self::U8,
            "i16" => Self::I16,
            "u16" => Self::U16,
            "i32" => Self::I32,
            "u32" => Self::U32,
            "i64" => Self::I64,
            "u64" => Self::U64,
            "isize" => Self::Isize,
            "usize" => Self::Usize,

            "f32" => Self::F32,
            "f64" => Self::F64,

            "this" => Self::ThisValue,
            "_" => Self::Discard,
            "true" => Self::True,
            "false" => Self::False,
            "default" => Self::Default,
            "sizeof" => Self::Sizeof,
            "nullptr" => Self::Nullptr,
            "invalid" => Self::Invalid,

            "for" => Self::For,
            "while" => Self::While,
            "loop" => Self::Loop,
            "if" => Self::If,
            "else" => Self::Else,
            "match" => Self::Match,
            "in" => Self::In,

            "break" => Self::Break,
            "return" => Self::Return,
            "label" => Self::Label,
            "goto" => Self::Goto,
            _ => return None,
        });
    }
}

impl Symbol {
    pub fn from(parser: &mut StringParser) -> Option<(StringSlice, Self)> {
        symbol_match!(parser,
            "..." => Self::Rest,

            "<..=" => Self::RangeFromTo,
            "..=" => Self::RangeTo,
            "<.." => Self::RangeFrom,
            ".." => Self::Range,

            "::" => Self::DoubleColon,
            ":" => Self::Colon,

            "?." => Self::ValueCoalesce,
            "!." => Self::ValueCascade,
            "?->" => Self::ReferenceCoalesce,
            "!->" => Self::ReferenceCascade,
            "." => Self::Dot,
            "->" => Self::SmallArrow,
            "?" => Self::Optional,
            "!" => Self::BoolNot,

            "=>" => Self::WideArrow,
            "," => Self::Comma,
            ";" => Self::Semicolon,

            "<<=" => Self::ShlAssign,
            "<<" => Self::Shl,
            ">>=" => Self::ShrAssign,
            ">>" => Self::Shr,

            ">=" => Self::GreaterEqual,
            ">" => Self::Greater,
            "<=" => Self::LessEqual,
            "<" => Self::Less,
            "!=" => Self::NotEqual,
            "==" => Self::Equal,
            "=" => Self::Assign,

            "|=" => Self::BitOrAssign,
            "||" => Self::BoolOr,
            "|" => Self::BitOr,

            "&=" => Self::BitAndAssign,
            "&&" => Self::BoolAnd,
            "&" => Self::BitAnd,

            "^=" => Self::BitXorAssign,
            "^^" => Self::BoolXor,
            "^" => Self::BitXor,

            "*=" => Self::MulAssign,
            "*" => Self::Mul,

            "%=" => Self::RemAssign,
            "%" => Self::Rem,

            "/=" => Self::DivAssign,
            "/" => Self::Div,

            "+=" => Self::AddAssign,
            "++" => Self::Increment,
            "+" => Self::Add,

            "-=" => Self::SubAssign,
            "--" => Self::Decrement,
            "-" => Self::Sub,

            "~=" => Self::BitNotAssign,
            "~" => Self::BitNot,

            "[" => Self::BracketOpen,
            "]" => Self::BracketClose,
            "(" => Self::ParenOpen,
            ")" => Self::ParenClose,
            "{" => Self::BraceOpen,
            "}" => Self::BraceClose,

            "#" => Self::Pound,
        );
        return None;
    }
}

macro symbol_match($parser: expr, $($st: expr => $sy: expr),+ $(,)?) {
    $(symbol_try!($parser, $st, $sy));+
}

macro symbol_try($parser: expr, $st: expr, $sy: expr) {
    if let Some(st) = $parser.try_consume_str($st) {
        return Some((st, $sy));
    }
}
