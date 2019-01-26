// Copyright 2019 The n-sql Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use ast::expression::Expression;
use std::fmt::{Debug, Display, Error, Formatter};

#[derive(Clone, Debug)]
pub struct CastFn {
    pub expr: Box<Expression>,
    pub data_type: DataType,
}
impl CastFn {
    pub fn new(expr: Box<Expression>, data_type: DataType) -> CastFn {
        CastFn { expr, data_type }
    }
}

#[derive(Debug)]
pub struct DataType {
    pub data_type: String,
}

impl Display for DataType {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.data_type)
    }
}

impl Clone for DataType {
    fn clone(&self) -> Self {
        DataType {
            data_type: self.data_type.clone(),
        }
    }
}

impl DataType {
    pub fn new(data_type: String) -> DataType {
        DataType { data_type }
    }
}
