// Copyright (c)  YISH. All rights reserved.
// Licensed under the MIT License. See License.txt in the project root for license information.
use super::{Location, Line, Position, Span};


pub trait Source {
    fn new(s: &str) -> Self
        where
            Self: Sized;

    fn location(&self, byte: Position) -> Option<Location>;

    fn span(&self) -> Span<Position>;

    fn src(&self) -> &str;

    fn src_slice(&self, span: Span<Position>) -> &str;

    fn line_number_at_byte(&self, pos: Position) -> Option<Line>;

    /// Returns the starting position of any comments and whitespace before `end`
    fn comment_start_before(&self, end: Position) -> Position;

    fn comments_between(&self, span: Span<Position>) -> CommentIter;
}

impl Source for super::codespan::FileMap {
    fn new(s: &str) -> Self where Self: Sized,
    {
        super::codespan::FileMap::new("test".into(), s.into())
    }

    /// Returns the line and column location of `byte`
    fn location(&self, byte: Position) -> Option<Location> {
        super::codespan::FileMap::location(self, byte)
            .ok()
            .map(|(line, column)| Location {
                line,
                column,
                absolute: byte,
            })
    }

    fn span(&self) -> Span<Position> {
        super::codespan::FileMap::span(self)
    }

    fn src(&self) -> &str {
        super::codespan::FileMap::src(self)
    }

    fn src_slice(&self, span: Span<Position>) -> &str {
        super::codespan::FileMap::src_slice(self, span).unwrap()
    }

    fn line_number_at_byte(&self, pos: Position) -> Option<Line> {
        self.find_line(pos).ok()
    }

    /// Returns the starting position of any comments and whitespace before `end`
    fn comment_start_before(&self, end: Position) -> Position {
        let mut iter = self.comments_between(Span::new(Position(0), end));
        // Scan from `end` until a non comment token is found
        for _ in iter.by_ref().rev() {}
        Position(iter.src.len() as u32)
    }

    fn comments_between(&self, span: Span<Position>) -> CommentIter {
        CommentIter {
            src: self.src_slice(span).unwrap(),
        }
    }
}

impl Source for () {
    fn new(_: &str) -> Self where Self: Sized,
    {
    }

    fn location(&self, _: Position) -> Option<Location> {
        None
    }

    fn span(&self) -> Span<Position> {
        Span::new(1.into(), 1.into())
    }

    fn src(&self) -> &str {
        ""
    }

    fn src_slice(&self, _: Span<Position>) -> &str {
        panic!("src_slice: Expected FileMap")
    }

    fn line_number_at_byte(&self, _: Position) -> Option<Line> {
        None
    }

    fn comment_start_before(&self, pos: Position) -> Position {
        pos
    }

    fn comments_between(&self, _: Span<Position>) -> CommentIter {
        CommentIter { src: "" }
    }
}

pub struct CommentIter<'a> {
    src: &'a str,
}

impl<'a> Iterator for CommentIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        if self.src.is_empty() {
            None
        } else {
            self.src = self
                .src
                .trim_matches(|c: char| c.is_whitespace() && c != '\n');
            if self.src.starts_with("//") && !self.src.starts_with("///") {
                let comment_line = self.src.lines().next().unwrap();
                self.src = &self.src[comment_line.len()..];
                self.src = if self.src.starts_with("\r\n") {
                    &self.src[2..]
                } else {
                    // \n
                    &self.src[1..]
                };
                Some(comment_line)
            } else if self.src.starts_with("/*") {
                self.src.find("*/").map(|i| {
                    let (comment, rest) = self.src.split_at(i + 2);
                    self.src = rest;
                    comment
                })
            } else if self.src.starts_with('\n') {
                self.src = &self.src[1..];
                Some("")
            } else {
                None
            }
        }
    }
}

impl<'a> DoubleEndedIterator for CommentIter<'a> {
    fn next_back(&mut self) -> Option<&'a str> {
        if self.src.is_empty() {
            None
        } else {
            self.src = self
                .src
                .trim_end_matches(|c: char| c.is_whitespace() && c != '\n');
            if self.src.ends_with('\n') {
                let comment_line = self.src[..self.src.len() - 1].lines().next_back().unwrap();
                let trimmed = comment_line.trim_start();

                let newline_len = if self.src.ends_with("\r\n") { 2 } else { 1 };
                self.src = &self.src[..(self.src.len() - newline_len)];

                if trimmed.starts_with("//") && !trimmed.starts_with("///") {
                    self.src = &self.src[..(self.src.len() - 2 - trimmed.len() - 1)];
                    Some(trimmed)
                } else {
                    Some("")
                }
            } else if self.src.ends_with("*/") {
                self.src.rfind("/*").map(|i| {
                    let (rest, comment) = self.src.split_at(i);
                    self.src = rest;
                    comment
                })
            } else {
                None
            }
        }
    }
}

