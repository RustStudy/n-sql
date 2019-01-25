// Copyright (c)  YISH. All rights reserved.
// Licensed under the MIT License. See License.txt in the project root for license information.
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
