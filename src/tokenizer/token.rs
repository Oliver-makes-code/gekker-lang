use crate::string::{parser::StringParser, StringSlice};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token<'a> {
    pub slice: Option<StringSlice<'a>>,
    pub kind: TokenKind<'a>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind<'a> {
    Identifier(&'a str),
    String(String),
    Char(char),
    Number(Number),
    Symbol(Symbol),
    Keyword(Keyword),
    Eof,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Number {
    pub whole: u64,
    pub decimal: u64,
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
    Impl,
    Operator,
    Where,
    Namespace,

    // Builtin types
    Bool,
    Char,
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

    F32,
    F64,

    // Values
    ThisValue, // this
    Discard,   // _
    True,
    False,

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

    // Other symbols
    BoolNot,     // !
    Colon,       // :
    Semicolon,   // ;
    Assign,      // =
    Optional,    // ?
    Dot,         // .
    Range,       // ..
    RangeTo,     // ..=
    RangeFrom,   // <..
    RangeFromTo, // <..=
    Expand,      // ...
    Comma,       // ,
    WideArrow,   // =>
    SmallArrow,  // ->
    DoubleColon, // ::
}

impl Keyword {
    pub fn from(s: &str) -> Option<Self> {
        return Some(match s {
            "let" => Self::Let,
            "mut" => Self::Mut,
            "sonst" => Self::Const,
            "static" => Self::Static,
            "func" => Self::Func,
            "pub" => Self::Pub,
            "using" => Self::Using,
            "ref" => Self::Ref,
            "struct" => Self::Struct,
            "enum" => Self::Enum,
            "impl" => Self::Impl,
            "operator" => Self::Operator,
            "where" => Self::Where,
            "namespace" => Self::Namespace,

            "bool" => Self::Bool,
            "char" => Self::Char,
            "This" => Self::ThisType,
            "unit" => Self::Unit,
            "never" => Self::Never,

            "i8" => Self::I8,
            "u8" => Self::U8,
            "i16" => Self::I16,
            "u16" => Self::U16,
            "i32" => Self::I32,
            "u32" => Self::U32,
            "i64" => Self::I64,
            "U64" => Self::U64,

            "f32" => Self::F32,
            "f64" => Self::F64,

            "this" => Self::ThisValue,
            "_" => Self::Discard,
            "true" => Self::True,
            "false" => Self::False,

            "for" => Self::For,
            "while" => Self::While,
            "loop" => Self::Loop,
            "if" => Self::If,
            "else" => Self::Else,
            "match" => Self::Match,
            "in" => Self::In,
            "break" => Self::Break,
            "return" => Self::Return,
            _ => return None,
        });
    }
}

impl Symbol {
    pub fn from<'a>(parser: &mut StringParser<'a>) -> Option<(StringSlice<'a>, Self)> {
        symbol_match!(parser,
            "..." => Self::Expand,
            "<..=" => Self::RangeFromTo,
            "..=" => Self::RangeTo,
            "<.." => Self::RangeFrom,
            ".." => Self::Range,
            "." => Self::Dot,

            "::" => Self::DoubleColon,
            ":" => Self::Colon,

            "=>" => Self::WideArrow,
            "->" => Self::SmallArrow,
            "?" => Self::Optional,
            "!" => Self::BoolNot,
            "," => Self::Comma,
            ";" => Self::Semicolon,

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
