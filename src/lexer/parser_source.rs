// Copyright 2019 The n-sql Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::location::Position;
use super::location::Span;
use super::location::ByteOffset;

pub trait ParserSource {
    fn src(&self) -> &str;
    fn start_index(&self) -> Position;

    fn span(&self) -> Span<Position> {
        let start = self.start_index();
        Span::new(start, start + ByteOffset(self.src().len() as i64))
    }
}

impl<'a, S> ParserSource for &'a S
    where
        S: ?Sized + ParserSource,
{
    fn src(&self) -> &str {
        (**self).src()
    }
    fn start_index(&self) -> Position {
        (**self).start_index()
    }
}

impl ParserSource for str {
    fn src(&self) -> &str {
        self
    }
    fn start_index(&self) -> Position {
        Position(1)
    }
}
