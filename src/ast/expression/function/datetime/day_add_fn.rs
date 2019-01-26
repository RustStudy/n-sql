// Copyright 2019 The n-sql Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use ast::Expression;

#[derive(Clone, Debug)]
pub struct DayAddFn {
    pub expr: Box<Expression>,
    pub offset: Box<Expression>
}

impl DayAddFn {
    pub fn new(expr: Box<Expression>, offset: Box<Expression>) -> DayAddFn {
        DayAddFn{expr, offset}
    }
}
