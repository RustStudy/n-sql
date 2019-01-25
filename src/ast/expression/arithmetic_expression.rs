// Copyright (c)  YISH. All rights reserved.
// Licensed under the MIT License. See License.txt in the project root for license information.

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
