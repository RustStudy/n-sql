// Copyright 2019 The n-sql Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use ast::{Expression, DatetimeType};

#[derive(Clone, Debug)]
pub struct DatetimeDiffFn {
    pub start: Box<Expression>,
    pub end: Box<Expression>,
    pub diff_type: DatetimeType
}

impl DatetimeDiffFn {
    pub fn new(start: Box<Expression>, end: Box<Expression>, diff_type: DatetimeType) -> DatetimeDiffFn {
        DatetimeDiffFn{start, end, diff_type}
    }
}