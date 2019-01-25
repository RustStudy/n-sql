// Copyright (c)  YISH. All rights reserved.
// Licensed under the MIT License. See License.txt in the project root for license information.

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
