// Copyright 2019 The n-sql Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

pub use super::codespan::{
    ByteIndex as Position, ByteOffset, ColumnIndex as Column, ColumnOffset, LineIndex as Line,
    LineOffset, Span,
};
use std::fmt;

#[derive(Copy, Clone, Default, Eq, PartialEq, Debug, Hash, Ord, PartialOrd)]
pub struct Location {
    pub line: Line,
    pub column: Column,
    pub absolute: Position,
}

impl Location {
    pub fn shift(mut self, ch: char) -> Location {
        if ch == '\n' {
            self.line += LineOffset(1);
            self.column = Column(1);
        } else {
            self.column += ColumnOffset(1);
        }
        self.absolute += ByteOffset(ch.len_utf8() as i64);
        self
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f, "Line: {}, Column: {}",
            self.line.number(),
            self.column.number()
        )
    }
}


#[derive(Copy, Clone, Debug, Eq, PartialEq, Default)]
pub struct Spanned<T, Position> {
    pub span: Span<Position>,
    pub value: T,
}

impl<T, Position: Ord> Spanned<T, Position> {
    pub fn map<U, F: FnMut(T) -> U>(self, mut f: F) -> Spanned<U, Position> {
        Spanned { span: self.span, value: f(self.value), }
    }
    pub fn new(span: Span<Position>, value: T) -> Spanned<T, Position> {
        Spanned { span, value }
    }
}

impl<T: fmt::Display, Position: fmt::Display + Copy> fmt::Display for Spanned<T, Position> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.span.start(), self.value)
    }
}


pub fn spanned<T, Position: Ord>(start: Position, end: Position, value: T) -> Spanned<T, Position>
{
    Spanned::new(Span::new(start, end), value)
}

pub trait HasSpan {
    fn span(&self) -> Span<Position>;
}

