// Copyright 2019 The n-sql Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
use std::fmt::{Display, Error, Formatter};

#[derive(Debug)]
pub struct Identifier {
    value: String,
}

impl Display for Identifier {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.value)
    }
}

impl Clone for Identifier {
    fn clone(&self) -> Self {
        Identifier {
            value: self.value.clone(),
        }
    }
}

impl From<String> for Identifier {
    fn from(value: String) -> Self {
        Identifier { value }
    }
}

impl<'input> From<&'input str> for Identifier {
    fn from(value: &'input str) -> Self {
        Identifier { value: value.to_string() }
    }
}
