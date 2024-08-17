pub mod parser;

use std::{fmt::Debug, sync::Arc};

#[derive(Clone, PartialEq, Eq)]
pub struct StringSlice {
    pub src: Arc<str>,
    pub start: usize,
    pub end: usize,
}

impl StringSlice {
    pub fn value(&self) -> Arc<str> {
        if self.start > self.src.len() || self.end > self.src.len() || self.start >= self.end {
            return "".into();
        }

        return self.src[self.start..self.end].into();
    }

    pub fn merge(&self, other: &Self) -> Self {
        let start = usize::min(self.start, other.start);
        let end = usize::max(self.end, other.end);
        return Self {
            src: self.src.clone(),
            start,
            end,
        };
    }
}

impl Debug for StringSlice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return self.value().fmt(f);
    }
}

pub trait ToStringSlice {
    fn slice(&self, start: usize, end: usize) -> StringSlice;
}

impl ToStringSlice for Arc<str> {
    fn slice(&self, start: usize, end: usize) -> StringSlice {
        return StringSlice {
            src: self.clone(),
            start,
            end,
        };
    }
}

#[cfg(test)]
mod test {
    use std::sync::Arc;

    use crate::string::{StringSlice, ToStringSlice};

    #[test]
    fn slice() {
        let s: Arc<str> = "Test!".into();
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
