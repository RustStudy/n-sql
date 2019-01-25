// Copyright (c)  YISH. All rights reserved.
// Licensed under the MIT License. See License.txt in the project root for license information.

use std::fmt::{Debug, Display, Error, Formatter};

mod numeric_value;
mod string_value;

pub use self::numeric_value::*;
pub use self::string_value::*;

#[derive(Clone, Debug)]
pub enum ConstantValue {
    String(StringValue),
    Numeric(NumericValue),
    Null,
}

impl Display for ConstantValue {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self)
    }
}

impl From<StringValue> for ConstantValue {
    fn from(v: StringValue) -> Self {
        ConstantValue::String(v)
    }
}

impl From<NumericValue> for ConstantValue {
    fn from(v: NumericValue) -> Self {
        ConstantValue::Numeric(v)
    }
}

impl From<i32> for ConstantValue {
    fn from(value: i32) -> Self {
        ConstantValue::Numeric(value.into())
    }
}

impl From<String> for ConstantValue {
    fn from(value: String) -> Self {
        ConstantValue::String(value.into())
    }
}
