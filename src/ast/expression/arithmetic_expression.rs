// Copyright 2019 The n-sql Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use ast::Expression;
use std::fmt::{Debug, Error, Formatter};

#[derive(Copy, Clone, Debug)]
pub enum ArithmeticOperator {
    // '*'
    Mul,
    // '/'
    Div,

    // '+'
    Add,

    // '-'
    Sub,
}

#[derive(Clone, Debug)]
pub struct ArithmeticExpression {
    pub left: Box<Expression>,
    pub op: ArithmeticOperator,
    pub right: Box<Expression>,
}

impl ArithmeticExpression {
    pub fn new(
        left: Box<Expression>,
        op: ArithmeticOperator,
        right: Box<Expression>,
    ) -> ArithmeticExpression {
        ArithmeticExpression { left, op, right }
    }
}
