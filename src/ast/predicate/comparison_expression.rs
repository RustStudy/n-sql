// Copyright 2019 The n-sql Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::super::Expression;
use core::fmt::Write;
use std::fmt::{Debug, Error, Formatter};

#[derive(Copy, Clone, Debug)]
pub enum ComparisonOperator {
    Equal,
    NotEqual,
    Less,
    Greater,
    LessOrEqual,
    GreaterOrEqual,
}

#[derive(Clone, Debug)]
pub struct ComparisonExpression {
    pub left: Box<Expression>,
    pub op: ComparisonOperator,
    pub right: Box<Expression>,
}

impl ComparisonExpression {
    pub fn new(
        left: Box<Expression>,
        op: ComparisonOperator,
        right: Box<Expression>,
    ) -> ComparisonExpression {
        ComparisonExpression { left, op, right }
    }
}
