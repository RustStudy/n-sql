// Copyright 2019 The n-sql Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use ast::{Expression, DatetimeType};

#[derive(Clone, Debug)]
pub struct ExtractFn {
    pub expr: Box<Expression>,
    pub extract_type: DatetimeType,
}


impl ExtractFn {
    pub fn new(extract_type: DatetimeType, expr: Box<Expression>) -> ExtractFn {
        ExtractFn { expr, extract_type }
    }
}
