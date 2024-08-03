use super::{StringSlice, ToStringSlice};

#[derive(Debug, Clone)]
pub struct StringParser<'a> {
    pub src: &'a str,
    idx: usize,
    idx_stack: Vec<usize>,
}

impl<'a> StringParser<'a> {
    pub fn new(src: &'a str) -> Self {
        return Self {
            src,
            idx: 0,
            idx_stack: vec![],
        };
    }

    pub fn idx(&self) -> usize {
        return self.idx;
    }

    pub fn checkout(&mut self) {
        self.idx_stack.push(self.idx);
    }

    pub fn commit(&mut self) -> Option<StringSlice<'a>> {
        let start = self.idx_stack.pop()?;

        return Some(self.src.slice(start, self.idx));
    }

    pub fn rollback(&mut self) -> bool {
        let Some(start) = self.idx_stack.pop() else {
            return false;
        };
        self.idx = start;

        return true;
    }

    pub fn curr(&self) -> Option<char> {
        return self.src.chars().nth(self.idx);
    }

    pub fn next(&mut self) -> Option<char> {
        self.idx += 1;
        if self.idx > self.src.len() {
            self.idx = self.src.len();
        }
        return self.curr();
    }

    pub fn is_char(&self, char: char) -> bool {
        if let Some(c) = self.curr() {
            return c == char;
        }
        return false;
    }

    pub fn is_func(&self, f: fn(char) -> bool) -> bool {
        if let Some(c) = self.curr() {
            return f(c);
        }
        return false;
    }

    pub fn while_char(&mut self, char: char) -> Option<StringSlice<'a>> {
        if !self.is_char(char) {
            return None;
        }

        self.checkout();
        while self.is_char(char) {
            self.next();
        }

        if let Some(s) = self.commit() {
            return Some(s);
        }
        self.rollback();
        return None;
    }

    pub fn while_func(&mut self, f: fn(char) -> bool) -> Option<StringSlice<'a>> {
        if !self.is_func(f) {
            return None;
        }

        self.checkout();
        while self.is_func(f) {
            self.next();
        }

        if let Some(s) = self.commit() {
            return Some(s);
        }
        self.rollback();
        return None;
    }

    pub fn try_consume_str(&mut self, s: &str) -> Option<StringSlice<'a>> {
        self.checkout();
        let mut chars = s.chars();

        while let Some(c) = chars.next() {
            if !self.is_char(c) {
                self.rollback();
                return None;
            }
            self.next();
        }

        if let Some(s) = self.commit() {
            return Some(s);
        }
        self.rollback();
        return None;
    }
}

#[cfg(test)]
mod test {
    use crate::string::StringSlice;

    use super::StringParser;

    #[test]
    fn try_consume_str() {
        let s = "some string";
        let mut parser = StringParser::new(s);

        assert_eq!(
            parser.try_consume_str(s),
            Some(StringSlice {
                src: s,
                start: 0,
                end: s.len()
            })
        );

        let mut parser = StringParser::new(s);

        assert_eq!(parser.try_consume_str("This will fail."), None);
    }

    #[test]
    fn while_char() {
        let s = "sssssssss";
        let mut parser = StringParser::new(s);

        assert_eq!(
            parser.while_char('s'),
            Some(StringSlice {
                src: s,
                start: 0,
                end: s.len()
            })
        );

        let mut parser = StringParser::new(s);

        assert_eq!(parser.while_char('t'), None);
    }

    #[test]
    fn while_func() {
        let s = "sssssssss";
        let mut parser = StringParser::new(s);

        assert_eq!(
            parser.while_func(char::is_alphabetic),
            Some(StringSlice {
                src: s,
                start: 0,
                end: s.len()
            })
        );

        let mut parser = StringParser::new(s);

        assert_eq!(parser.while_func(char::is_numeric), None);
    }
}
