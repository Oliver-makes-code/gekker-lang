pub mod parser;

use std::{fmt::Debug, ops::Deref};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct StringSlice<'a> {
    pub src: &'a str,
    pub start: usize,
    pub end: usize,
}

impl<'a> StringSlice<'a> {
    pub fn value(&self) -> &'a str {
        if self.start > self.src.len() || self.end > self.src.len() || self.start >= self.end {
            return "";
        }

        return &self.src[self.start..self.end];
    }

    pub fn merge(self, other: Self) -> Self {
        let start = usize::min(self.start, other.start);
        let end = usize::max(self.end, other.end);
        return Self {
            src: self.src,
            start,
            end,
        };
    }
}

impl<'a> Debug for StringSlice<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return self.value().fmt(f);
    }
}

impl<'a> Deref for StringSlice<'a> {
    type Target = str;

    fn deref(&self) -> &'a Self::Target {
        return self.value();
    }
}

pub trait ToStringSlice<'a> {
    fn slice(&self, start: usize, end: usize) -> StringSlice<'a>;
}

impl<'a> ToStringSlice<'a> for &'a str {
    fn slice(&self, start: usize, end: usize) -> StringSlice<'a> {
        return StringSlice {
            src: self,
            start,
            end,
        };
    }
}

#[cfg(test)]
mod test {
    use crate::string::{StringSlice, ToStringSlice};

    #[test]
    fn slice() {
        let s = "Test!";
        assert_eq!(
            s.slice(0, 2),
            StringSlice {
                src: s,
                start: 0,
                end: 2
            }
        );
    }
}
