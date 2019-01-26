// Copyright 2019 The n-sql Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fmt::{Debug, Display, Error, Formatter};

#[derive(Debug)]
pub struct StringValue {
    pub value: String,
}

impl Display for StringValue {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.value)
    }
}

impl Clone for StringValue {
    fn clone(&self) -> Self {
        StringValue {
            value: self.value.clone(),
        }
    }
}

impl From<String> for StringValue {
    fn from(value: String) -> Self {
        StringValue { value }
    }
}
