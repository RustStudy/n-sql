// Copyright (c)  YISH. All rights reserved.
// Licensed under the MIT License. See License.txt in the project root for license information.

use super::location::{Location, Line, Column};
use super::parser_source::ParserSource;
use std::str::Chars;

pub struct CharLocations<'input> {
    pub location: Location,
    pub chars: Chars<'input>,
}

impl<'input> CharLocations<'input> {
    pub fn new<S>(input: &'input S) -> CharLocations<'input>
        where
            S: ?Sized + ParserSource,
    {
        CharLocations {
            location: Location {
                line: Line(0),
                column: Column(1),
                absolute: input.start_index(),
            },
            chars: input.src().chars(),
        }
    }
}

impl<'input> Iterator for CharLocations<'input> {
    type Item = (Location, char);

    fn next(&mut self) -> Option<(Location, char)> {
        self.chars.next().map(|ch| {
            let location = self.location;
            self.location = self.location.shift(ch);
            // HACK: The layout algorithm expects `1` indexing for columns -
            // this could be altered in the future though
            if self.location.column == Column(0) {
                self.location.column = Column(1);
            }
            (location, ch)
        })
    }
}
