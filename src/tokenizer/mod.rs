use token::{Keyword, Number, Symbol, Token, TokenKind};

use crate::string::{parser::StringParser, StringSlice};

pub mod token;

fn valid_ident_start(c: char) -> bool {
    return c.is_alphabetic() || c == '_';
}

fn valid_ident_cont(c: char) -> bool {
    return c.is_alphanumeric() || c == '_';
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenizeError<'a> {
    InvalidString(StringSlice<'a>),
    InvalidChar(StringSlice<'a>),
    UnclosedChar(StringSlice<'a>),
    InvalidEscape(StringSlice<'a>),
    UnclosedStr(StringSlice<'a>),
    UnexpectedEof,
}

pub struct Tokenizer<'a> {
    parser: StringParser<'a>,
    peek: Result<Option<Token<'a>>, TokenizeError<'a>>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(src: &'a str) -> Self {
        return Self {
            parser: StringParser::new(src),
            peek: Ok(None),
        };
    }

    fn try_parse_ident(&mut self) -> Option<StringSlice<'a>> {
        if self.parser.is_func(valid_ident_start) {
            return self.parser.while_func(valid_ident_cont);
        }
        return None;
    }

    fn try_parse_number(&mut self) -> Option<(StringSlice<'a>, Number)> {
        self.parser.checkout();

        if let Some(whole_slice) = self.parser.while_func(char::is_numeric) {
            let whole: u64 = whole_slice.value().parse().unwrap();

            if self.parser.is_char('.') {
                self.parser.checkout();
                self.parser.next();

                if let Some(_) = self.parser.while_func(char::is_numeric) {
                    let decimal = self.parser.commit()?.value().parse().unwrap();
                    return Some((self.parser.commit()?, Number { whole, decimal }));
                }
                self.parser.rollback();
            }

            return Some((
                self.parser.commit()?,
                Number {
                    whole,
                    decimal: 0.0,
                },
            ));
        }

        self.parser.rollback();

        return None;
    }

    fn try_parse_char(&mut self) -> Result<Option<(StringSlice<'a>, char)>, TokenizeError<'a>> {
        if !self.parser.is_char('\'') {
            return Ok(None);
        }
        self.parser.checkout();

        self.parser.next();

        let Some(c) = self.parser.curr() else {
            self.parser.rollback();
            return Err(TokenizeError::UnexpectedEof);
        };

        match c {
            'a'..='z'
            | 'A'..='Z'
            | '0'..='9'
            | ' '
            | '!'
            | '#'
            | '%'
            | '&'
            | '"'
            | '('
            | ')'
            | '['
            | ']'
            | '{'
            | '}'
            | '*'
            | '+'
            | ','
            | '-'
            | '.'
            | '/'
            | ':'
            | ';'
            | '<'
            | '>'
            | '='
            | '?'
            | '^'
            | '_'
            | '|'
            | '~' => {
                let Some('\'') = self.parser.next() else {
                    let Some(s) = self.parser.commit() else {
                        return Err(TokenizeError::UnexpectedEof);
                    };
                    return Err(TokenizeError::UnclosedChar(s));
                };

                self.parser.next();
                let Some(s) = self.parser.commit() else {
                    return Err(TokenizeError::UnexpectedEof);
                };
                return Ok(Some((s, c)));
            }
            '\\' => {
                let Some(c) = self.parser.next() else {
                    return Err(TokenizeError::UnexpectedEof);
                };
                let val = match c {
                    'n' => '\n',
                    'r' => '\r',
                    '\\' => '\\',
                    't' => '\t',
                    '"' => '"',
                    '\'' => '\'',
                    _ => {
                        let Some(s) = self.parser.commit() else {
                            return Err(TokenizeError::UnexpectedEof);
                        };

                        return Err(TokenizeError::InvalidEscape(s));
                    }
                };

                let Some('\'') = self.parser.next() else {
                    let Some(s) = self.parser.commit() else {
                        return Err(TokenizeError::UnexpectedEof);
                    };
                    return Err(TokenizeError::UnclosedChar(s));
                };

                self.parser.next();
                let Some(s) = self.parser.commit() else {
                    return Err(TokenizeError::UnexpectedEof);
                };
                return Ok(Some((s, val)));
            }
            _ => {
                let Some(s) = self.parser.commit() else {
                    return Err(TokenizeError::UnexpectedEof);
                };
                return Err(TokenizeError::UnclosedChar(s));
            }
        }
    }

    fn try_parse_string(&mut self) -> Result<Option<(StringSlice<'a>, String)>, TokenizeError<'a>> {
        if !self.parser.is_char('"') {
            return Ok(None);
        }

        self.parser.checkout();

        self.parser.next();

        let mut str = "".to_string();

        while let Some(c) = self.parser.curr() {
            match c {
                'a'..='z'
                | 'A'..='Z'
                | '0'..='9'
                | ' '
                | '!'
                | '#'
                | '%'
                | '&'
                | '\''
                | '('
                | ')'
                | '['
                | ']'
                | '{'
                | '}'
                | '*'
                | '+'
                | ','
                | '-'
                | '.'
                | '/'
                | ':'
                | ';'
                | '<'
                | '>'
                | '='
                | '?'
                | '^'
                | '_'
                | '|'
                | '~' => {
                    str.push(c);
                    self.parser.next();
                }
                '\\' => {
                    let Some(c) = self.parser.next() else {
                        return Err(TokenizeError::UnexpectedEof);
                    };
                    let val = match c {
                        'n' => '\n',
                        'r' => '\r',
                        '\\' => '\\',
                        't' => '\t',
                        '"' => '"',
                        '\'' => '\'',
                        _ => {
                            let Some(s) = self.parser.commit() else {
                                return Err(TokenizeError::UnexpectedEof);
                            };

                            return Err(TokenizeError::InvalidEscape(s));
                        }
                    };

                    str.push(val);
                    self.parser.next();
                }
                '"' => {
                    self.parser.next();
                    let Some(s) = self.parser.commit() else {
                        return Err(TokenizeError::UnexpectedEof);
                    };

                    return Ok(Some((s, str)));
                }
                '\n' => {
                    let Some(s) = self.parser.commit() else {
                        return Err(TokenizeError::UnexpectedEof);
                    };
                    return Err(TokenizeError::UnclosedStr(s));
                }
                _ => {
                    let Some(s) = self.parser.commit() else {
                        return Err(TokenizeError::UnexpectedEof);
                    };
                    return Err(TokenizeError::InvalidString(s));
                }
            }
        }

        self.parser.rollback();

        return Err(TokenizeError::UnexpectedEof);
    }

    pub fn peek(&mut self) -> Result<Token<'a>, TokenizeError<'a>> {
        let peek = self.peek.clone()?;
        if let Some(peek) = peek {
            return Ok(peek);
        }
        let token = self.next()?;
        self.peek = Ok(Some(token.clone()));
        return Ok(token);
    }

    pub fn next(&mut self) -> Result<Token<'a>, TokenizeError<'a>> {
        if let Some(peek) = self.peek.clone()? {
            self.peek = Ok(None);
            return Ok(peek);
        }

        self.parser.while_func(char::is_whitespace);

        if let None = self.parser.curr() {
            return Ok(Token {
                slice: None,
                kind: TokenKind::Eof,
            });
        }

        if let Some(slice) = self.try_parse_ident() {
            let value = slice.value();

            if let Some(keyword) = Keyword::from(value) {
                return Ok(Token {
                    slice: Some(slice),
                    kind: TokenKind::Keyword(keyword),
                });
            }

            return Ok(Token {
                slice: Some(slice),
                kind: TokenKind::Identifier(value),
            });
        }

        if let Some((slice, symbol)) = Symbol::from(&mut self.parser) {
            return Ok(Token {
                slice: Some(slice),
                kind: TokenKind::Symbol(symbol),
            });
        }

        if let Some((slice, number)) = self.try_parse_number() {
            return Ok(Token {
                slice: Some(slice),
                kind: TokenKind::Number(number),
            });
        }

        if let Some((slice, char)) = self.try_parse_char()? {
            return Ok(Token {
                slice: Some(slice),
                kind: TokenKind::Char(char),
            });
        }

        if let Some((slice, str)) = self.try_parse_string()? {
            return Ok(Token {
                slice: Some(slice),
                kind: TokenKind::String(str),
            });
        }

        return Err(TokenizeError::InvalidChar(self.parser.commit().unwrap()));
    }
}