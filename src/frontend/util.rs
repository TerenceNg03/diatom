use std::{ops::BitOr, str::Chars};

use super::lexer::Token;

pub struct FileIterator<'a> {
    location: LineLocation,
    iterator: Chars<'a>,
}

impl<'a> FileIterator<'a> {
    pub fn new(file_content: &'a str) -> Self {
        Self {
            location: LineLocation::new(0, 0),
            iterator: file_content.chars(),
        }
    }

    /// Return next character. Useful to lookahead.
    pub fn peek(&self) -> Option<char> {
        let mut iter = self.iterator.clone();
        iter.next()
    }

    /// Return the next two character
    pub fn peek2(&self) -> (Option<char>, Option<char>) {
        let mut iter = self.iterator.clone();
        match iter.next() {
            c1 @ Some(_) => (c1, iter.next()),
            None => (None, None),
        }
    }

    /// Return current location's clone.
    pub fn get_location(&self) -> LineLocation {
        self.location.clone()
    }
}

impl<'a> Iterator for FileIterator<'a> {
    type Item = char;
    fn next(&mut self) -> Option<Self::Item> {
        let next_item = self.iterator.next();
        match next_item {
            Some(c) => {
                if c == '\n' {
                    self.location.line += 1;
                    self.location.offset = 0;
                } else {
                    self.location.offset += 1;
                };
            }
            None => (),
        };
        next_item
    }
}

/// Indicate a location in a specific line.
#[derive(Clone, PartialEq, Eq)]
#[cfg_attr(test, derive(Debug))]
pub struct LineLocation {
    pub line: usize,
    pub offset: usize,
}

impl LineLocation {
    pub fn new(line: usize, offset: usize) -> Self {
        Self { line, offset }
    }
}

impl Default for LineLocation {
    fn default() -> Self {
        Self { line: 0, offset: 0 }
    }
}

impl PartialOrd for LineLocation {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(match self.line.cmp(&other.line) {
            std::cmp::Ordering::Equal => self.offset.cmp(&other.offset),
            ord => ord,
        })
    }
}

impl Ord for LineLocation {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.line.cmp(&other.line) {
            std::cmp::Ordering::Equal => self.offset.cmp(&other.offset),
            ord => ord,
        }
    }
}
/// Indicate location of a range of text in a specific file.
#[cfg_attr(test, derive(Debug))]
#[derive(Clone)]
pub struct FileLocation {
    pub start: LineLocation,
    pub end: LineLocation,
}

impl FileLocation {
    pub fn new(start: LineLocation, end: LineLocation) -> Self {
        Self { start, end }
    }
}

impl Default for FileLocation {
    fn default() -> Self {
        Self {
            start: Default::default(),
            end: Default::default(),
        }
    }
}

impl BitOr for FileLocation {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            start: if self.start > rhs.start {
                rhs.start
            } else {
                self.start
            },
            end: if self.end < rhs.end {
                rhs.end
            } else {
                self.end
            },
        }
    }
}

pub struct TokenIterator<'a> {
    iterator: std::slice::Iter<'a, (Token, FileLocation)>,
    location: FileLocation,
}

impl<'a> TokenIterator<'a> {
    pub fn new(vec: &'a Vec<(Token, FileLocation)>) -> Self {
        TokenIterator {
            iterator: vec.iter(),
            location: Default::default(),
        }
    }

    pub fn peek(&self) -> Option<&Token> {
        let mut iter = self.iterator.clone();
        iter.next().and_then(|x| Some(&x.0))
    }

    pub fn peek2(&self) -> (Option<&Token>, Option<&Token>) {
        let mut iter = self.iterator.clone();
        match iter.next() {
            t1 @ Some(_) => {
                let t1 = t1.and_then(|x| Some(&x.0));
                match iter.next() {
                    t2 @ Some(_) => {
                        let t2 = t2.and_then(|x| Some(&x.0));
                        (t1, t2)
                    }
                    None => (t1, None),
                }
            }
            None => (None, None),
        }
    }

    pub fn get_location(&self) -> FileLocation {
        self.location.clone()
    }

    pub fn reset_location_counter(&mut self) {
        self.location.start = self.location.end.clone();
    }
}

impl<'a> Iterator for TokenIterator<'a> {
    type Item = &'a Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.iterator.next().and_then(|x| {
            self.location.end = x.1.end.clone();
            Some(&x.0)
        })
    }
}
