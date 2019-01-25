// Copyright (c)  YISH. All rights reserved.
// Licensed under the MIT License. See License.txt in the project root for license information.

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
